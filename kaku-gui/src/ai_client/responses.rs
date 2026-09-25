//! Responses API wire format: request translation, JSON and SSE response
//! parsing, tool-call assembly, and citations.

use anyhow::{Context, Result};
use reqwest::header::CONTENT_TYPE;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};

use super::transport::{
    add_stream_bytes, add_stream_event, read_response_body_capped, read_sse_line_capped,
};
use super::{
    sse_data_payload, ApiMessage, ChatStepResult, ToolCall, ToolCallBuf, MAX_RESPONSE_CITATIONS,
    MAX_RESPONSE_CITATION_TITLE_CHARS, MAX_RESPONSE_CITATION_URL_CHARS, MAX_RESPONSE_OUTPUT_ITEMS,
    MAX_RESPONSE_TOOL_ARGUMENT_BYTES, MAX_RESPONSE_TOOL_CALLS,
};

pub(super) fn translate_responses_messages(
    messages: &[ApiMessage],
) -> (String, Vec<serde_json::Value>) {
    use serde_json::json;

    let mut instructions = String::new();
    let mut input = Vec::new();
    for ApiMessage(message) in messages {
        if let Some(item) = message.get("kaku_responses_output_item") {
            // A reasoning stub without its encrypted payload cannot be
            // replayed under `store: false` (providers reject it). Skip the
            // stub, keep the rest of the transcript.
            if item["type"].as_str() == Some("reasoning")
                && item["encrypted_content"].as_str().is_none()
            {
                continue;
            }
            input.push(item.clone());
            continue;
        }
        let role = message["role"].as_str().unwrap_or("user");

        if role == "tool" {
            input.push(json!({
                "type": "function_call_output",
                "call_id": message["tool_call_id"].as_str().unwrap_or(""),
                "output": message["content"].as_str().unwrap_or(""),
            }));
            continue;
        }

        if role == "assistant" {
            if let Some(tool_calls) = message["tool_calls"].as_array() {
                for call in tool_calls {
                    input.push(json!({
                        "type": "function_call",
                        "call_id": call["id"].as_str().unwrap_or(""),
                        "name": call["function"]["name"].as_str().unwrap_or(""),
                        "arguments": call["function"]["arguments"].as_str().unwrap_or("{}"),
                    }));
                }
                continue;
            }
        }

        let content = message["content"].as_str().unwrap_or("");
        if content.is_empty() {
            continue;
        }
        if role == "system" {
            if !instructions.is_empty() {
                instructions.push_str("\n\n");
            }
            instructions.push_str(content);
            continue;
        }
        let content_type = if role == "assistant" {
            "output_text"
        } else {
            "input_text"
        };
        input.push(json!({
            "type": "message",
            "role": role,
            "content": [{ "type": content_type, "text": content }],
        }));
    }

    // Responses rejects empty input. One-shot helpers sometimes supply only a
    // system message, so promote that text to input instead of sending an
    // instructions-only request.
    if input.is_empty() && !instructions.is_empty() {
        input.push(json!({
            "type": "message",
            "role": "user",
            "content": [{ "type": "input_text", "text": std::mem::take(&mut instructions) }],
        }));
    }

    (instructions, input)
}

pub(super) fn translate_responses_tools(
    tools: &[serde_json::Value],
    tools_enabled: bool,
    native_web_search: bool,
) -> Vec<serde_json::Value> {
    use serde_json::{json, Value};

    if !tools_enabled {
        return Vec::new();
    }

    let mut translated = tools
        .iter()
        .filter_map(|tool| {
            let function = tool.get("function")?;
            let mut translated = json!({
                "type": "function",
                "name": function.get("name")?,
                "description": function.get("description").cloned().unwrap_or(Value::Null),
                "parameters": function
                    .get("parameters")
                    .cloned()
                    .unwrap_or_else(|| json!({ "type": "object", "properties": {} })),
            });
            if let Some(strict) = function.get("strict").and_then(Value::as_bool) {
                translated["strict"] = Value::Bool(strict);
            }
            Some(translated)
        })
        .collect::<Vec<_>>();

    if native_web_search {
        translated.push(json!({ "type": "web_search" }));
    }
    translated
}

pub(super) fn parse_responses_http(
    response: reqwest::blocking::Response,
    cancelled: &AtomicBool,
    on_token: &mut dyn FnMut(&str),
    on_reasoning: &mut dyn FnMut(&str),
    provider_label: &str,
) -> Result<ChatStepResult> {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    if content_type
        .as_deref()
        .is_some_and(content_type_is_event_stream)
    {
        return parse_responses_sse(
            BufReader::new(response),
            cancelled,
            on_token,
            on_reasoning,
            provider_label,
        );
    }

    let bytes = read_response_body_capped(response, provider_label)?;
    let looks_json = content_type.as_deref().is_some_and(content_type_is_json)
        || bytes
            .iter()
            .copied()
            .find(|byte| !byte.is_ascii_whitespace())
            .is_some_and(|byte| matches!(byte, b'{' | b'['));
    if looks_json {
        let value = serde_json::from_slice::<serde_json::Value>(&bytes)
            .with_context(|| format!("parse {provider_label} JSON response"))?;
        return parse_responses_value(&value, on_token, on_reasoning, provider_label);
    }

    parse_responses_sse(
        std::io::Cursor::new(bytes),
        cancelled,
        on_token,
        on_reasoning,
        provider_label,
    )
}

fn content_type_is_json(value: &str) -> bool {
    let media_type = value
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    media_type == "application/json"
        || (media_type.starts_with("application/") && media_type.ends_with("+json"))
}

fn content_type_is_event_stream(value: &str) -> bool {
    value
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .eq_ignore_ascii_case("text/event-stream")
}

fn parse_responses_sse<R: BufRead>(
    mut reader: R,
    cancelled: &AtomicBool,
    on_token: &mut dyn FnMut(&str),
    on_reasoning: &mut dyn FnMut(&str),
    provider_label: &str,
) -> Result<ChatStepResult> {
    let mut calls: Vec<(String, ToolCallBuf)> = Vec::new();
    let mut citations = Vec::new();
    let mut saw_text_delta = false;
    let mut saw_reasoning_delta = false;
    let mut streamed_text = String::new();
    let mut completed_output_text = String::new();
    let mut response_items = Vec::new();
    let mut indexless_item_positions = std::collections::HashMap::new();
    let mut call_alias_positions = std::collections::HashMap::new();
    let mut call_output_positions = std::collections::HashMap::new();
    let mut completed = false;
    let mut stream_bytes = 0usize;
    let mut stream_events = 0usize;

    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(ChatStepResult::empty());
        }
        let Some(mut line_bytes) = read_sse_line_capped(&mut reader, provider_label)? else {
            break;
        };
        add_stream_bytes(&mut stream_bytes, line_bytes.len(), provider_label)?;
        while matches!(line_bytes.last(), Some(b'\n' | b'\r')) {
            line_bytes.pop();
        }
        let line = std::str::from_utf8(&line_bytes)
            .with_context(|| format!("{provider_label} SSE line was not UTF-8"))?;
        let Some(data) = sse_data_payload(line) else {
            continue;
        };
        add_stream_event(&mut stream_events, provider_label)?;
        if data.trim() == "[DONE]" {
            break;
        }
        let event = serde_json::from_str::<serde_json::Value>(data)
            .with_context(|| format!("parse {provider_label} SSE event"))?;

        match event["type"].as_str() {
            Some("response.output_text.delta") => {
                if let Some(delta) = event["delta"].as_str() {
                    saw_text_delta = true;
                    streamed_text.push_str(delta);
                    on_token(delta);
                }
            }
            Some("response.reasoning_summary_text.delta")
            | Some("response.reasoning_text.delta") => {
                if let Some(delta) = event["delta"].as_str() {
                    saw_reasoning_delta = true;
                    on_reasoning(delta);
                }
            }
            Some("response.output_text.annotation.added") => {
                collect_response_annotation(&event["annotation"], &mut citations);
            }
            Some("response.output_item.added") | Some("response.output_item.done") => {
                let item = &event["item"];
                upsert_response_item(
                    &mut response_items,
                    item,
                    event["output_index"].as_u64(),
                    &mut indexless_item_positions,
                )?;
                if item["type"] == "function_call" {
                    upsert_response_call(
                        &mut calls,
                        item,
                        event["output_index"].as_u64(),
                        &mut call_alias_positions,
                        &mut call_output_positions,
                    )?;
                } else if item["type"] == "message" {
                    collect_response_citations(item, &mut citations);
                }
            }
            Some("response.function_call_arguments.delta") => {
                let item_id = event["item_id"].as_str().unwrap_or("");
                let updated_arguments = if let Some(buffer) = upsert_stream_call(
                    &mut calls,
                    item_id,
                    event["output_index"].as_u64(),
                    &mut call_alias_positions,
                    &mut call_output_positions,
                )? {
                    if let Some(delta) = event["delta"].as_str() {
                        append_tool_arguments(&mut buffer.arguments, delta, provider_label)?;
                    }
                    Some(buffer.arguments.clone())
                } else {
                    None
                };
                if let Some(arguments) = updated_arguments {
                    update_response_item_arguments(&mut response_items, item_id, &arguments);
                }
            }
            Some("response.function_call_arguments.done") => {
                let item_id = event["item_id"].as_str().unwrap_or("");
                let updated_arguments = if let Some(buffer) = upsert_stream_call(
                    &mut calls,
                    item_id,
                    event["output_index"].as_u64(),
                    &mut call_alias_positions,
                    &mut call_output_positions,
                )? {
                    if let Some(arguments) = event["arguments"].as_str() {
                        set_tool_arguments(&mut buffer.arguments, arguments, provider_label)?;
                    }
                    Some(buffer.arguments.clone())
                } else {
                    None
                };
                if let Some(arguments) = updated_arguments {
                    update_response_item_arguments(&mut response_items, item_id, &arguments);
                }
            }
            Some("response.completed") => {
                let completed_response = &event["response"];
                validate_completed_response(completed_response, provider_label)?;
                completed_output_text = completed_response["output_text"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                if let Some(output) = completed_response["output"].as_array() {
                    validate_response_item_count(output.len(), provider_label)?;
                    if !output.is_empty() {
                        response_items = output.clone();
                        let completed_calls = response_calls(completed_response, provider_label)?;
                        if !completed_calls.is_empty() {
                            calls = completed_calls;
                        }
                    }
                }
                emit_response_output(
                    completed_response,
                    !saw_text_delta,
                    !saw_reasoning_delta,
                    on_token,
                    on_reasoning,
                );
                collect_response_citations(completed_response, &mut citations);
                completed = true;
                break;
            }
            Some("response.failed") | Some("response.incomplete") => {
                let message = response_error_message(&event)
                    .or_else(|| response_error_message(&event["response"]))
                    .unwrap_or("unknown error");
                anyhow::bail!("{provider_label} failed: {message}");
            }
            Some("error") => {
                let message = response_error_message(&event).unwrap_or("unknown error");
                anyhow::bail!("{provider_label} failed: {message}");
            }
            _ => {}
        }
    }

    if cancelled.load(Ordering::Relaxed) {
        return Ok(ChatStepResult::empty());
    }
    if !completed {
        anyhow::bail!("{provider_label} stream ended before response.completed");
    }
    validate_response_call_buffers(&calls, provider_label)?;
    sync_response_call_items(&mut response_items, &calls, provider_label)?;
    let citations_text = format_response_citations(&citations);
    if !citations_text.is_empty() {
        on_token(&citations_text);
    }
    if !response_items.iter().any(response_message_has_text) {
        let mut final_text = if streamed_text.is_empty() {
            completed_output_text
        } else {
            streamed_text
        };
        final_text.push_str(&citations_text);
        if !final_text.is_empty() {
            upsert_synthesized_response_message(&mut response_items, final_text, provider_label)?;
        }
    }
    validate_response_item_count(response_items.len(), provider_label)?;
    Ok(ChatStepResult {
        tool_calls: tool_calls_from_buffers(calls),
        response_items,
    })
}

fn parse_responses_value(
    response: &serde_json::Value,
    on_token: &mut dyn FnMut(&str),
    on_reasoning: &mut dyn FnMut(&str),
    provider_label: &str,
) -> Result<ChatStepResult> {
    validate_completed_response(response, provider_label)?;

    let mut citations = Vec::new();
    let mut output = response["output"].as_array().cloned().unwrap_or_default();
    validate_response_item_count(output.len(), provider_label)?;
    emit_response_output(response, true, true, on_token, on_reasoning);
    collect_response_citations(response, &mut citations);

    let citations_text = format_response_citations(&citations);
    if !citations_text.is_empty() {
        on_token(&citations_text);
    }
    if !output.iter().any(response_message_has_text) {
        let mut text = response["output_text"].as_str().unwrap_or("").to_string();
        text.push_str(&citations_text);
        if !text.is_empty() {
            upsert_synthesized_response_message(&mut output, text, provider_label)?;
        }
    }
    Ok(ChatStepResult {
        tool_calls: tool_calls_from_buffers(response_calls(response, provider_label)?),
        response_items: output,
    })
}

fn validate_completed_response(response: &serde_json::Value, provider_label: &str) -> Result<()> {
    match response["status"].as_str() {
        Some("completed") => Ok(()),
        Some("failed" | "incomplete") => {
            let message = response_error_message(response).unwrap_or("unknown error");
            anyhow::bail!("{provider_label} failed: {message}")
        }
        Some(status) => anyhow::bail!("{provider_label} returned unexpected status `{status}`"),
        None => anyhow::bail!("{provider_label} response omitted completion status"),
    }
}

fn emit_response_output(
    response: &serde_json::Value,
    emit_text: bool,
    emit_reasoning: bool,
    on_token: &mut dyn FnMut(&str),
    on_reasoning: &mut dyn FnMut(&str),
) {
    let mut emitted_text = false;
    if let Some(output) = response["output"].as_array() {
        for item in output {
            match item["type"].as_str() {
                Some("message") if emit_text => {
                    if let Some(content) = item["content"].as_array() {
                        for part in content {
                            if let Some(text) =
                                part["text"].as_str().or_else(|| part["refusal"].as_str())
                            {
                                emitted_text = true;
                                on_token(text);
                            }
                        }
                    }
                }
                Some("reasoning") if emit_reasoning => {
                    if let Some(summary) = item["summary"].as_array() {
                        for part in summary {
                            if let Some(text) = part["text"].as_str() {
                                on_reasoning(text);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if emit_text && !emitted_text {
        if let Some(text) = response["output_text"].as_str() {
            on_token(text);
        }
    }
}

fn response_calls(
    response: &serde_json::Value,
    provider_label: &str,
) -> Result<Vec<(String, ToolCallBuf)>> {
    let mut calls = Vec::new();
    let mut aliases = std::collections::HashMap::new();
    let mut output_positions = std::collections::HashMap::new();
    if let Some(output) = response["output"].as_array() {
        for (output_index, item) in output.iter().enumerate() {
            if item["type"] == "function_call" {
                upsert_response_call(
                    &mut calls,
                    item,
                    Some(output_index as u64),
                    &mut aliases,
                    &mut output_positions,
                )?;
            }
        }
    }
    validate_response_call_buffers(&calls, provider_label)?;
    Ok(calls)
}

fn upsert_response_call(
    calls: &mut Vec<(String, ToolCallBuf)>,
    item: &serde_json::Value,
    output_index: Option<u64>,
    aliases: &mut std::collections::HashMap<String, usize>,
    output_positions: &mut std::collections::HashMap<u64, usize>,
) -> Result<()> {
    let item_id = item["id"].as_str().filter(|value| !value.is_empty());
    let call_id = item["call_id"].as_str().filter(|value| !value.is_empty());
    let mut aliases_for_item = Vec::with_capacity(2);
    if let Some(item_id) = item_id {
        aliases_for_item.push(item_id);
    }
    if let Some(call_id) = call_id {
        aliases_for_item.push(call_id);
    }
    if let Some(buffer) = upsert_call_identity(
        calls,
        &aliases_for_item,
        output_index,
        aliases,
        output_positions,
    )? {
        if let Some(call_id) = item["call_id"].as_str().filter(|value| !value.is_empty()) {
            buffer.id = call_id.to_string();
        }
        if let Some(name) = item["name"].as_str().filter(|value| !value.is_empty()) {
            buffer.name = name.to_string();
        }
        if let Some(arguments) = item["arguments"].as_str().filter(|value| !value.is_empty()) {
            set_tool_arguments(&mut buffer.arguments, arguments, "Responses API")?;
        }
    }
    Ok(())
}

fn upsert_stream_call<'a>(
    calls: &'a mut Vec<(String, ToolCallBuf)>,
    item_id: &str,
    output_index: Option<u64>,
    aliases: &mut std::collections::HashMap<String, usize>,
    output_positions: &mut std::collections::HashMap<u64, usize>,
) -> Result<Option<&'a mut ToolCallBuf>> {
    let identities = (!item_id.is_empty())
        .then_some(item_id)
        .into_iter()
        .collect::<Vec<_>>();
    upsert_call_identity(calls, &identities, output_index, aliases, output_positions)
}

fn upsert_call_identity<'a>(
    calls: &'a mut Vec<(String, ToolCallBuf)>,
    identities: &[&str],
    output_index: Option<u64>,
    aliases: &mut std::collections::HashMap<String, usize>,
    output_positions: &mut std::collections::HashMap<u64, usize>,
) -> Result<Option<&'a mut ToolCallBuf>> {
    let position = identities
        .iter()
        .find_map(|identity| aliases.get(*identity).copied())
        .or_else(|| output_index.and_then(|index| output_positions.get(&index).copied()));

    let position = match position {
        Some(position) => position,
        None => {
            if identities.is_empty() && output_index.is_none() {
                return Ok(None);
            }
            if calls.len() >= MAX_RESPONSE_TOOL_CALLS {
                anyhow::bail!("Responses API returned too many function calls");
            }
            let storage_id = identities
                .first()
                .map(|identity| (*identity).to_string())
                .unwrap_or_else(|| format!("output_index:{}", output_index.unwrap_or_default()));
            calls.push((storage_id, ToolCallBuf::default()));
            calls.len() - 1
        }
    };

    for identity in identities {
        aliases.insert((*identity).to_string(), position);
    }
    if let Some(index) = output_index {
        output_positions.insert(index, position);
    }
    Ok(Some(&mut calls[position].1))
}

fn upsert_response_item(
    items: &mut Vec<serde_json::Value>,
    item: &serde_json::Value,
    output_index: Option<u64>,
    indexless_positions: &mut std::collections::HashMap<u64, usize>,
) -> Result<()> {
    let id = item["id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .or_else(|| item["call_id"].as_str().filter(|value| !value.is_empty()));
    if let Some(id) = id {
        if let Some(position) = items.iter().position(|existing| {
            existing["id"].as_str() == Some(id) || existing["call_id"].as_str() == Some(id)
        }) {
            items[position] = item.clone();
            if let Some(index) = output_index {
                indexless_positions.insert(index, position);
            }
            return Ok(());
        }
    }
    // Custom `/responses` endpoints may omit item ids on either the `.added`
    // or the `.done` event. `output_index` is stable across both, so track it
    // for every stored item; whichever identity the later event carries, it
    // replaces instead of duplicating (positions are stable: parsing only
    // appends or replaces in place).
    if let Some(index) = output_index {
        if let Some(&position) = indexless_positions.get(&index) {
            items[position] = item.clone();
            return Ok(());
        }
    }
    validate_response_item_count(items.len() + 1, "Responses API")?;
    if let Some(index) = output_index {
        indexless_positions.insert(index, items.len());
    }
    items.push(item.clone());
    Ok(())
}

fn update_response_item_arguments(items: &mut [serde_json::Value], item_id: &str, arguments: &str) {
    if item_id.is_empty() {
        return;
    }
    if let Some(item) = items.iter_mut().find(|item| {
        item["id"].as_str() == Some(item_id) || item["call_id"].as_str() == Some(item_id)
    }) {
        item["arguments"] = serde_json::Value::String(arguments.to_string());
    }
}

fn sync_response_call_items(
    items: &mut Vec<serde_json::Value>,
    calls: &[(String, ToolCallBuf)],
    provider_label: &str,
) -> Result<()> {
    for (item_id, call) in calls {
        let existing = items.iter_mut().find(|item| {
            (!item_id.is_empty() && item["id"].as_str() == Some(item_id.as_str()))
                || (!call.id.is_empty() && item["call_id"].as_str() == Some(call.id.as_str()))
        });
        if let Some(item) = existing {
            item["arguments"] = serde_json::Value::String(call.arguments.clone());
            continue;
        }

        validate_response_item_count(items.len() + 1, provider_label)?;
        let mut item = serde_json::json!({
            "type": "function_call",
            "call_id": call.id,
            "name": call.name,
            "arguments": call.arguments,
        });
        if !item_id.is_empty() {
            item["id"] = serde_json::Value::String(item_id.clone());
        }
        items.push(item);
    }
    Ok(())
}

fn validate_response_item_count(count: usize, provider_label: &str) -> Result<()> {
    if count > MAX_RESPONSE_OUTPUT_ITEMS {
        anyhow::bail!(
            "{provider_label} returned more than {} output items",
            MAX_RESPONSE_OUTPUT_ITEMS
        );
    }
    Ok(())
}

pub(super) fn append_tool_arguments(
    target: &mut String,
    delta: &str,
    provider_label: &str,
) -> Result<()> {
    let new_len = target
        .len()
        .checked_add(delta.len())
        .ok_or_else(|| anyhow::anyhow!("{provider_label} tool arguments overflowed"))?;
    if new_len > MAX_RESPONSE_TOOL_ARGUMENT_BYTES {
        anyhow::bail!(
            "{provider_label} tool arguments exceeded {} bytes",
            MAX_RESPONSE_TOOL_ARGUMENT_BYTES
        );
    }
    target.push_str(delta);
    Ok(())
}

fn set_tool_arguments(target: &mut String, arguments: &str, provider_label: &str) -> Result<()> {
    if arguments.len() > MAX_RESPONSE_TOOL_ARGUMENT_BYTES {
        anyhow::bail!(
            "{provider_label} tool arguments exceeded {} bytes",
            MAX_RESPONSE_TOOL_ARGUMENT_BYTES
        );
    }
    target.clear();
    target.push_str(arguments);
    Ok(())
}

fn validate_response_call_buffers(
    calls: &[(String, ToolCallBuf)],
    provider_label: &str,
) -> Result<()> {
    if calls.len() > MAX_RESPONSE_TOOL_CALLS {
        anyhow::bail!(
            "{provider_label} returned more than {} function calls",
            MAX_RESPONSE_TOOL_CALLS
        );
    }
    for (_, call) in calls {
        if call.arguments.len() > MAX_RESPONSE_TOOL_ARGUMENT_BYTES {
            anyhow::bail!(
                "{provider_label} tool arguments exceeded {} bytes",
                MAX_RESPONSE_TOOL_ARGUMENT_BYTES
            );
        }
    }
    Ok(())
}

fn tool_calls_from_buffers(calls: Vec<(String, ToolCallBuf)>) -> Vec<ToolCall> {
    calls
        .into_iter()
        .map(|(_, buffer)| buffer)
        .filter(|buffer| !buffer.name.is_empty())
        .map(|buffer| ToolCall {
            id: buffer.id,
            name: buffer.name,
            arguments: buffer.arguments,
        })
        .collect()
}

fn response_error_message(value: &serde_json::Value) -> Option<&str> {
    value["error"]["message"]
        .as_str()
        .or_else(|| value["message"].as_str())
        .or_else(|| value["incomplete_details"]["reason"].as_str())
}

fn collect_response_citations(value: &serde_json::Value, citations: &mut Vec<(String, String)>) {
    if let Some(content) = value["content"].as_array() {
        for part in content {
            if let Some(annotations) = part["annotations"].as_array() {
                for annotation in annotations {
                    collect_response_annotation(annotation, citations);
                }
            }
        }
    }
    if let Some(output) = value["output"].as_array() {
        for item in output {
            collect_response_citations(item, citations);
        }
    }
}

fn collect_response_annotation(
    annotation: &serde_json::Value,
    citations: &mut Vec<(String, String)>,
) {
    if annotation["type"] != "url_citation" {
        return;
    }
    if citations.len() >= MAX_RESPONSE_CITATIONS {
        return;
    }
    let Some(url) = annotation["url"].as_str().filter(|url| !url.is_empty()) else {
        return;
    };
    // Truncate before deduplicating so stored (truncated) URLs compare
    // against the same shape.
    let url = url
        .chars()
        .take(MAX_RESPONSE_CITATION_URL_CHARS)
        .collect::<String>();
    if citations.iter().any(|(_, existing)| existing == &url) {
        return;
    }
    let title = annotation["title"]
        .as_str()
        .filter(|title| !title.is_empty())
        .unwrap_or(&url)
        .replace(['\n', '\r'], " ")
        .chars()
        .take(MAX_RESPONSE_CITATION_TITLE_CHARS)
        .collect();
    citations.push((title, url));
}

fn format_response_citations(citations: &[(String, String)]) -> String {
    if citations.is_empty() {
        return String::new();
    }
    let mut out = String::from("\n\nSources:\n");
    for (title, url) in citations {
        // Kaku's compact Markdown renderer intentionally drops link targets,
        // so keep the URL visible for copying and terminal link detection.
        out.push_str(&format!("- {title}: {url}\n"));
    }
    out
}

fn synthesized_response_message(text: String) -> serde_json::Value {
    serde_json::json!({
        "type": "message",
        "role": "assistant",
        "status": "completed",
        "content": [{
            "type": "output_text",
            "text": text,
            "annotations": [],
        }],
    })
}

fn response_message_has_text(item: &serde_json::Value) -> bool {
    item["type"] == "message"
        && item["content"].as_array().is_some_and(|content| {
            content.iter().any(|part| {
                part["text"]
                    .as_str()
                    .or_else(|| part["refusal"].as_str())
                    .is_some_and(|text| !text.is_empty())
            })
        })
}

fn upsert_synthesized_response_message(
    items: &mut Vec<serde_json::Value>,
    text: String,
    provider_label: &str,
) -> Result<()> {
    let message = synthesized_response_message(text);
    if let Some(existing) = items.iter_mut().find(|item| item["type"] == "message") {
        *existing = message;
    } else {
        validate_response_item_count(items.len() + 1, provider_label)?;
        items.push(message);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        content_type_is_json, parse_responses_sse, parse_responses_value,
        translate_responses_messages, translate_responses_tools,
    };
    use crate::ai_client::{ApiMessage, MAX_RESPONSE_TOOL_ARGUMENT_BYTES};
    use std::io::Cursor;
    use std::sync::atomic::AtomicBool;
    #[test]
    fn responses_translation_flattens_functions_and_adds_native_search() {
        let messages = vec![
            ApiMessage::system("Be concise"),
            ApiMessage::user("Search this"),
            ApiMessage::assistant_tool_calls(serde_json::json!([{
                "id": "call_1",
                "type": "function",
                "function": { "name": "pwd", "arguments": "{}" }
            }])),
            ApiMessage::tool_result("call_1", "pwd", "/tmp"),
        ];
        let (instructions, input) = translate_responses_messages(&messages);
        assert_eq!(instructions, "Be concise");
        assert_eq!(input[0]["content"][0]["type"], "input_text");
        assert_eq!(input[1]["type"], "function_call");
        assert_eq!(input[2]["type"], "function_call_output");

        let tools = vec![serde_json::json!({
            "type": "function",
            "function": {
                "name": "pwd",
                "description": "Print cwd",
                "parameters": { "type": "object", "properties": {} },
                "strict": true
            }
        })];
        let translated = translate_responses_tools(&tools, true, true);
        assert_eq!(translated[0]["name"], "pwd");
        assert_eq!(translated[0]["strict"], true);
        assert_eq!(translated[1], serde_json::json!({ "type": "web_search" }));
        assert!(translate_responses_tools(&tools, false, true).is_empty());
    }

    #[test]
    fn responses_translation_replays_raw_reasoning_items_before_tool_outputs() {
        let reasoning = serde_json::json!({
            "type": "reasoning",
            "id": "rs_1",
            "encrypted_content": "opaque-provider-state",
            "summary": []
        });
        let function_call = serde_json::json!({
            "type": "function_call",
            "id": "fc_1",
            "call_id": "call_1",
            "name": "pwd",
            "arguments": "{}"
        });
        let messages = vec![
            ApiMessage::responses_output_item(reasoning.clone()),
            ApiMessage::responses_output_item(function_call.clone()),
            ApiMessage::tool_result("call_1", "pwd", "/tmp"),
        ];

        let (_, input) = translate_responses_messages(&messages);
        assert_eq!(input[0], reasoning);
        assert_eq!(input[1], function_call);
        assert_eq!(input[2]["type"], "function_call_output");
    }

    #[test]
    fn upsert_response_item_replaces_across_mixed_identity_events() {
        let added = serde_json::json!({ "type": "message", "id": "msg_1", "content": [] });
        let done = serde_json::json!({
            "type": "message",
            "content": [{ "type": "output_text", "text": "hi" }]
        });

        // added carries an id, done does not: output_index must still dedup.
        let mut items = Vec::new();
        let mut positions = std::collections::HashMap::new();
        super::upsert_response_item(&mut items, &added, Some(0), &mut positions).unwrap();
        super::upsert_response_item(&mut items, &done, Some(0), &mut positions).unwrap();
        assert_eq!(items.len(), 1, "mixed-identity events must not duplicate");
        assert_eq!(items[0], done);

        // Reverse order: added without id, done with id.
        let mut items = Vec::new();
        let mut positions = std::collections::HashMap::new();
        super::upsert_response_item(&mut items, &done, Some(0), &mut positions).unwrap();
        super::upsert_response_item(&mut items, &added, Some(0), &mut positions).unwrap();
        assert_eq!(items.len(), 1, "mixed-identity events must not duplicate");
        assert_eq!(items[0], added);
    }

    #[test]
    fn responses_sse_deduplicates_function_calls_across_mixed_identities() {
        let stream = concat!(
            "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"id\":\"fc_1\",\"call_id\":\"call_1\",\"name\":\"fs_write\",\"arguments\":\"{}\"}}\n\n",
            "data: {\"type\":\"response.output_item.done\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"call_id\":\"call_1\",\"name\":\"fs_write\",\"arguments\":\"{}\"}}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"output\":[]}}\n\n"
        );
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |_| {},
            &mut |_| {},
            "test",
        )
        .unwrap();

        assert_eq!(result.response_items.len(), 1);
        assert_eq!(
            result.tool_calls.len(),
            1,
            "one output item must execute once"
        );
        assert_eq!(result.tool_calls[0].id, "call_1");
    }

    #[test]
    fn responses_sse_uses_output_index_for_late_item_id_deltas() {
        let stream = concat!(
            "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"call_id\":\"call_1\",\"name\":\"fs_write\",\"arguments\":\"\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.delta\",\"output_index\":0,\"item_id\":\"fc_1\",\"delta\":\"{}\"}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"output\":[]}}\n\n"
        );
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |_| {},
            &mut |_| {},
            "test",
        )
        .unwrap();

        assert_eq!(result.tool_calls.len(), 1);
        assert_eq!(result.tool_calls[0].id, "call_1");
        assert_eq!(result.tool_calls[0].arguments, "{}");
        assert_eq!(result.response_items.len(), 1);
        assert_eq!(result.response_items[0]["arguments"], "{}");
    }

    #[test]
    fn responses_translation_skips_reasoning_stubs_without_encrypted_content() {
        let stub = serde_json::json!({
            "type": "reasoning",
            "id": "rs_1",
            "summary": []
        });
        let message_item = serde_json::json!({
            "type": "message",
            "id": "msg_1",
            "role": "assistant",
            "content": [{ "type": "output_text", "text": "hi" }]
        });
        let messages = vec![
            ApiMessage::responses_output_item(stub),
            ApiMessage::responses_output_item(message_item.clone()),
        ];

        let (_, input) = translate_responses_messages(&messages);
        assert_eq!(
            input.len(),
            1,
            "content-less reasoning stub must be dropped"
        );
        assert_eq!(input[0], message_item);
    }

    #[test]
    fn recognizes_vendor_json_content_types() {
        assert!(content_type_is_json("application/json; charset=utf-8"));
        assert!(content_type_is_json("application/problem+json"));
        assert!(content_type_is_json("application/vnd.openai.response+json"));
        assert!(!content_type_is_json("text/event-stream"));
    }

    #[test]
    fn responses_json_parses_text_citations_reasoning_and_function_calls() {
        let response = serde_json::json!({
            "status": "completed",
            "output": [
                {
                    "type": "reasoning",
                    "summary": [{ "type": "summary_text", "text": "Checking sources" }]
                },
                {
                    "type": "message",
                    "content": [{
                        "type": "output_text",
                        "text": "Current answer",
                        "annotations": [{
                            "type": "url_citation",
                            "url": "https://example.com/source",
                            "title": "Example source"
                        }]
                    }]
                },
                {
                    "type": "function_call",
                    "id": "fc_1",
                    "call_id": "call_1",
                    "name": "pwd",
                    "arguments": "{}"
                }
            ]
        });
        let mut text = String::new();
        let mut reasoning = String::new();
        let result = parse_responses_value(
            &response,
            &mut |token| text.push_str(token),
            &mut |token| reasoning.push_str(token),
            "test",
        )
        .unwrap();

        assert_eq!(reasoning, "Checking sources");
        assert!(text.starts_with("Current answer"));
        assert!(text.contains("Example source: https://example.com/source"));
        assert_eq!(result.tool_calls.len(), 1);
        assert_eq!(result.tool_calls[0].id, "call_1");
        assert_eq!(result.tool_calls[0].name, "pwd");
        assert_eq!(result.tool_calls[0].arguments, "{}");
        assert_eq!(result.response_items.len(), 3);
    }

    #[test]
    fn responses_sse_parses_streamed_text_citations_and_function_calls() {
        let stream = concat!(
            "data: {\"type\":\"response.reasoning_summary_text.delta\",\"delta\":\"Thinking\"}\n\n",
            "data: {\"type\":\"response.output_text.delta\",\"delta\":\"Answer\"}\n\n",
            "data: {\"type\":\"response.output_text.annotation.added\",\"annotation\":{\"type\":\"url_citation\",\"url\":\"https://example.com\",\"title\":\"Example\"}}\n\n",
            "data: {\"type\":\"response.output_item.added\",\"item\":{\"type\":\"function_call\",\"id\":\"fc_1\",\"call_id\":\"call_1\",\"name\":\"pwd\",\"arguments\":\"\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.delta\",\"item_id\":\"fc_1\",\"delta\":\"{}\"}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"output\":[]}}\n\n"
        );
        let mut text = String::new();
        let mut reasoning = String::new();
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |token| text.push_str(token),
            &mut |token| reasoning.push_str(token),
            "test",
        )
        .unwrap();

        assert_eq!(reasoning, "Thinking");
        assert!(text.starts_with("Answer"));
        assert!(text.contains("Example: https://example.com"));
        assert_eq!(result.tool_calls.len(), 1);
        assert_eq!(result.tool_calls[0].id, "call_1");
        assert_eq!(result.tool_calls[0].arguments, "{}");
        assert_eq!(result.response_items[0]["arguments"], "{}");
        let message = result
            .response_items
            .iter()
            .find(|item| item["type"] == "message")
            .expect("delta-only response should synthesize a replayable message item");
        assert_eq!(message["content"][0]["text"], text);
    }

    #[test]
    fn responses_json_output_text_synthesizes_replayable_message() {
        let response = serde_json::json!({
            "status": "completed",
            "output": [],
            "output_text": "final answer",
        });
        let mut text = String::new();
        let result = parse_responses_value(
            &response,
            &mut |token| text.push_str(token),
            &mut |_| {},
            "test",
        )
        .unwrap();

        assert_eq!(text, "final answer");
        assert_eq!(result.response_items.len(), 1);
        assert_eq!(result.response_items[0]["type"], "message");
        assert_eq!(
            result.response_items[0]["content"][0]["text"],
            "final answer"
        );
    }

    #[test]
    fn responses_sse_rejects_eof_before_response_completed() {
        let stream = concat!(
            "data: {\"type\":\"response.output_item.added\",\"item\":{\"type\":\"function_call\",\"id\":\"fc_1\",\"call_id\":\"call_1\",\"name\":\"pwd\",\"arguments\":\"{}\"}}\n\n",
            "data: [DONE]\n\n"
        );
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |_| {},
            &mut |_| {},
            "test",
        );

        assert!(
            result.is_err(),
            "truncated streams must never execute tools"
        );
    }

    #[test]
    fn responses_sse_rejects_malformed_data_event() {
        let stream = concat!(
            "data: {not-json}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"output\":[]}}\n\n"
        );
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |_| {},
            &mut |_| {},
            "test",
        );

        assert!(
            result.is_err(),
            "malformed lifecycle events must fail closed"
        );
    }

    #[test]
    fn responses_sse_rejects_oversized_tool_arguments() {
        let oversized = "x".repeat(MAX_RESPONSE_TOOL_ARGUMENT_BYTES + 1);
        let stream = format!(
            "data: {{\"type\":\"response.output_item.added\",\"item\":{{\"type\":\"function_call\",\"id\":\"fc_1\",\"call_id\":\"call_1\",\"name\":\"pwd\",\"arguments\":\"\"}}}}\n\ndata: {{\"type\":\"response.function_call_arguments.delta\",\"item_id\":\"fc_1\",\"delta\":\"{}\"}}\n\ndata: {{\"type\":\"response.completed\",\"response\":{{\"status\":\"completed\",\"output\":[]}}}}\n\n",
            oversized
        );
        let result = parse_responses_sse(
            Cursor::new(stream),
            &AtomicBool::new(false),
            &mut |_| {},
            &mut |_| {},
            "test",
        );

        assert!(result.is_err());
    }
}
