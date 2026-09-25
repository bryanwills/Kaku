//! Concrete Codex configuration parsing, readiness, fields, and persistence.

use super::super::{
    decode_jwt_payload_with_debug, format_auth_status, read_json_file_with_debug, FieldEntry, Tool,
};
use crate::utils::write_atomic;
use anyhow::Context;
use std::path::{Path, PathBuf};

pub(in crate::ai_config::tui) fn codex_home_dir() -> PathBuf {
    kaku_ai_utils::codex_home_dir(&config::HOME_DIR)
}

/// Get Codex account email and ChatGPT plan tier from the JWT token in
/// `auth.json`. Plan is `None` for non-ChatGPT-tied tokens (e.g. enterprise
/// API keys) so the caller can omit the trailing "· Plan" suffix.
fn get_codex_account() -> Option<(String, Option<String>)> {
    let auth_path = codex_home_dir().join("auth.json");
    let auth_json = read_json_file_with_debug(&auth_path, "codex account")?;

    let token = auth_json.get("tokens")?.get("access_token")?.as_str()?;
    let jwt_data = decode_jwt_payload_with_debug(token, "codex account")?;

    let email = jwt_data
        .get("https://api.openai.com/profile")?
        .get("email")?
        .as_str()?
        .to_string();

    // ChatGPT plan lives in the OpenAI auth claim. Field examples observed:
    // "free", "plus", "pro", "team", "enterprise", "edu".
    let plan = jwt_data
        .get("https://api.openai.com/auth")
        .and_then(|v| v.get("chatgpt_plan_type"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    Some((email, plan))
}

pub(in crate::ai_config::tui) fn codex_connection_is_configured() -> bool {
    codex_connection_is_configured_at(&codex_home_dir(), |name| std::env::var(name).ok())
}

fn codex_connection_is_configured_at<F>(home: &Path, env: F) -> bool
where
    F: Fn(&str) -> Option<String>,
{
    let auth_path = home.join("auth.json");
    let auth = read_json_file_with_debug(&auth_path, "codex readiness");
    if auth_path.exists() && auth.is_none() {
        return false;
    }
    let auth_ready = || {
        let auth = auth.as_ref()?;
        match auth.get("auth_mode").and_then(|value| value.as_str()) {
            Some("apikey") | Some("api-key") | Some("api_key") => auth
                .get("OPENAI_API_KEY")
                .and_then(|value| value.as_str())
                .is_some_and(|value| !value.trim().is_empty())
                .then_some(()),
            Some("chatgpt") => auth
                .get("tokens")
                .and_then(|tokens| tokens.get("access_token"))
                .or_else(|| auth.get("access_token"))
                .and_then(|value| value.as_str())
                .is_some_and(|value| !value.trim().is_empty())
                .then_some(()),
            _ => None,
        }
    };

    let raw = match std::fs::read_to_string(home.join("config.toml")) {
        Ok(raw) => raw,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return auth_ready().is_some()
        }
        Err(_) => return false,
    };
    let parsed = match raw.parse::<toml::Value>() {
        Ok(parsed) => parsed,
        Err(_) => return false,
    };
    for field in [
        "model",
        "model_reasoning_effort",
        "model_reasoning_summary",
        "openai_base_url",
        "chatgpt_base_url",
        "model_provider",
    ] {
        if parsed.get(field).is_some_and(|value| !value.is_str()) {
            return false;
        }
    }
    let provider_id = parsed
        .get("model_provider")
        .and_then(|value| value.as_str())
        .unwrap_or("openai");
    if provider_id == "openai" {
        let valid_override = |field: &str| {
            parsed
                .get(field)
                .and_then(|value| value.as_str())
                .is_none_or(codex_base_url_is_valid)
        };
        return valid_override("openai_base_url")
            && valid_override("chatgpt_base_url")
            && auth_ready().is_some();
    }
    let Some(provider) = parsed
        .get("model_providers")
        .and_then(|value| value.get(provider_id))
        .and_then(|value| value.as_table())
    else {
        return false;
    };
    if ["experimental_bearer_token", "aws", "auth"]
        .iter()
        .any(|field| provider.contains_key(*field))
    {
        return false;
    }
    if provider
        .get("wire_api")
        .is_some_and(|value| value.as_str() != Some("responses"))
    {
        return false;
    }
    let Some(base_url) = provider.get("base_url").and_then(|value| value.as_str()) else {
        return false;
    };
    if !codex_base_url_is_valid(base_url)
        || !codex_string_table_is_valid(provider.get("http_headers"))
        || !codex_string_table_is_valid(provider.get("query_params"))
        || !codex_env_header_table_is_ready(provider.get("env_http_headers"), &env)
        || !codex_optional_retry_is_valid(provider.get("request_max_retries"))
        || !codex_optional_retry_is_valid(provider.get("stream_max_retries"))
        || !codex_optional_non_negative_integer_is_valid(provider.get("stream_idle_timeout_ms"))
    {
        return false;
    }
    let requires_openai_auth = match provider.get("requires_openai_auth") {
        Some(value) => match value.as_bool() {
            Some(value) => value,
            None => return false,
        },
        None => false,
    };
    if let Some(env_key) = provider.get("env_key") {
        let Some(env_key) = env_key.as_str() else {
            return false;
        };
        return env(env_key).is_some_and(|value| !value.is_empty());
    }
    if requires_openai_auth {
        return auth_ready().is_some();
    }
    true
}

fn codex_base_url_is_valid(value: &str) -> bool {
    url::Url::parse(value)
        .is_ok_and(|url| matches!(url.scheme(), "http" | "https") && url.host_str().is_some())
}

fn codex_string_table_is_valid(value: Option<&toml::Value>) -> bool {
    value.is_none_or(|value| {
        value
            .as_table()
            .is_some_and(|table| table.values().all(toml::Value::is_str))
    })
}

fn codex_env_header_table_is_ready<F>(value: Option<&toml::Value>, env: &F) -> bool
where
    F: Fn(&str) -> Option<String>,
{
    value.is_none_or(|value| {
        value.as_table().is_some_and(|table| {
            table.values().all(|value| {
                value
                    .as_str()
                    .and_then(env)
                    .is_some_and(|value| !value.is_empty())
            })
        })
    })
}

fn codex_optional_non_negative_integer_is_valid(value: Option<&toml::Value>) -> bool {
    value.is_none_or(|value| value.as_integer().is_some_and(|value| value >= 0))
}

fn codex_optional_retry_is_valid(value: Option<&toml::Value>) -> bool {
    value.is_none_or(|value| {
        value
            .as_integer()
            .is_some_and(|value| (0..=10).contains(&value))
    })
}

pub(in crate::ai_config::tui) fn extract_codex_fields(raw: &str) -> Vec<FieldEntry> {
    let mut fields = Vec::new();
    let mut has_model = false;

    // Read available models from Codex model cache
    let model_options = read_codex_model_options();

    // Parse TOML manually for the fields we care about
    for line in raw.lines() {
        let line = line.trim();
        if line.starts_with('[') || line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim().trim_matches('"');
            let val = val.trim().trim_matches('"');
            match key {
                "model" => {
                    has_model = true;
                    fields.push(FieldEntry {
                        key: "Model".into(),
                        value: val.to_string(),
                        options: model_options.clone(),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
    }

    if !has_model {
        let default_model = model_options
            .first()
            .cloned()
            .unwrap_or_else(|| "default".into());
        fields.insert(
            0,
            FieldEntry {
                key: "Model".into(),
                value: default_model,
                options: model_options.clone(),
                ..Default::default()
            },
        );
    }

    // Check auth status from auth.json
    let auth_path = codex_home_dir().join("auth.json");
    if let Some(auth) = read_json_file_with_debug(&auth_path, "codex auth status") {
        let auth_mode = auth.get("auth_mode").and_then(|v| v.as_str()).unwrap_or("");
        if !auth_mode.is_empty() {
            let (account, plan) = match get_codex_account() {
                Some((email, plan)) => (Some(email), plan),
                None => (None, None),
            };
            fields.push(FieldEntry {
                key: "Auth".into(),
                value: format_auth_status(account, auth_mode, plan.as_deref()),
                options: vec![],
                editable: false,
            });
        }
    }

    fields
}

/// Read model slugs from Codex's own cache. A custom provider's catalog must
/// never be replaced with the generic OpenAI list from models.dev.
pub(in crate::ai_config::tui) fn read_codex_model_options() -> Vec<String> {
    let cache_path = codex_home_dir().join("models_cache.json");
    if let Some(parsed) = read_json_file_with_debug(&cache_path, "codex model cache") {
        let mut models: Vec<(String, usize)> = parsed
            .get("models")
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter(|m| {
                        m.get("visibility")
                            .and_then(|v| v.as_str())
                            .map(|v| v == "list")
                            .unwrap_or(false)
                    })
                    .filter_map(|m| {
                        let slug = m.get("slug").and_then(|v| v.as_str())?;
                        let priority =
                            m.get("priority").and_then(|v| v.as_u64()).unwrap_or(999) as usize;
                        Some((slug.to_string(), priority))
                    })
                    .collect()
            })
            .unwrap_or_default();
        if !models.is_empty() {
            models.sort_by_key(|(_, p)| *p);
            return models.into_iter().map(|(s, _)| s).collect();
        }
    }

    Vec::new()
}

/// Save a field to Codex TOML config (~/.codex/config.toml)
pub(in crate::ai_config::tui) fn save_codex_field(
    field_key: &str,
    new_val: &str,
) -> anyhow::Result<()> {
    let path = Tool::Codex.config_path();
    save_codex_field_at(&path, field_key, new_val)
}

fn save_codex_field_at(path: &Path, field_key: &str, new_val: &str) -> anyhow::Result<()> {
    let toml_key = match field_key {
        "Model" => "model",
        "Reasoning Effort" => "model_reasoning_effort",
        _ => return Ok(()),
    };

    let raw = if path.exists() {
        std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?
    } else {
        String::new()
    };

    let mut lines: Vec<String> = raw.lines().map(|l| l.to_string()).collect();
    let target = format!("{} = ", toml_key);
    let new_line = format!("{} = \"{}\"", toml_key, new_val);

    let mut found = false;
    let mut in_top_level = true;
    for line in &mut lines {
        let trimmed = line.trim_start();
        // Entering a table section: [section] or [[array-of-tables]]
        if trimmed.starts_with('[') {
            in_top_level = false;
        }
        if in_top_level && trimmed.starts_with(&target) {
            if new_val == "-" || new_val.is_empty() {
                *line = String::new();
            } else {
                *line = new_line.clone();
            }
            found = true;
            break;
        }
    }

    if !found && !new_val.is_empty() && new_val != "-" {
        // Insert before the first [section] or at the end
        let insert_pos = lines
            .iter()
            .position(|l| l.trim_start().starts_with('['))
            .unwrap_or(lines.len());
        lines.insert(insert_pos, new_line);
    }

    // Remove empty lines that resulted from deletion
    let output: Vec<&str> = lines.iter().map(|l| l.as_str()).collect();
    let result = output.join("\n");
    write_atomic(path, result.as_bytes()).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn codex_save_round_trip_for_model_and_reasoning_effort() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("config.toml");

        std::fs::write(
            &path,
            "model = \"old\"\nmodel_reasoning_effort = \"low\"\n\n[projects.\"/tmp\"]\ntrust_level = \"trusted\"\n",
        )
        .expect("seed config");

        save_codex_field_at(&path, "Model", "gpt-5").expect("update model");
        save_codex_field_at(&path, "Reasoning Effort", "high").expect("update effort");
        let saved = std::fs::read_to_string(&path).expect("read config");
        assert!(saved.contains("model = \"gpt-5\""));
        assert!(saved.contains("model_reasoning_effort = \"high\""));
        assert!(saved.contains("[projects.\"/tmp\"]"));

        save_codex_field_at(&path, "Model", "").expect("remove model");
        let saved = std::fs::read_to_string(&path).expect("read config");
        assert!(!saved.contains("model = \"gpt-5\""));
        assert!(saved.contains("model_reasoning_effort = \"high\""));
        assert!(saved.contains("[projects.\"/tmp\"]"));
    }

    #[test]
    fn codex_save_creates_new_top_level_entry_before_sections() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "[profiles.default]\nfoo = \"bar\"\n").expect("seed config");

        save_codex_field_at(&path, "Model", "gpt-5").expect("insert model");
        let saved = std::fs::read_to_string(&path).expect("read config");
        let model_pos = saved.find("model = \"gpt-5\"").expect("model line");
        let section_pos = saved.find("[profiles.default]").expect("section");
        assert!(model_pos < section_pos);
    }

    #[test]
    fn codex_extract_adds_default_model_when_missing() {
        let fields = extract_codex_fields("");
        let model = fields
            .iter()
            .find(|field| field.key == "Model")
            .expect("model field");
        assert!(!model.value.is_empty());
        assert!(!model.value.ends_with(" (default)"));
    }

    #[test]
    fn codex_readiness_matches_runtime_provider_validation() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../tests/fixtures/codex-connection.json"
        )))
        .expect("shared connection fixtures");
        for case in cases.as_array().unwrap() {
            let home = tempdir().unwrap();
            std::fs::write(
                home.path().join("config.toml"),
                case["config"].as_str().unwrap(),
            )
            .unwrap();
            if let Some(auth) = case.get("auth") {
                std::fs::write(home.path().join("auth.json"), auth.to_string()).unwrap();
            }
            let ready = codex_connection_is_configured_at(home.path(), |key| {
                case["env"][key].as_str().map(str::to_owned)
            });
            assert_eq!(ready, case["ready"].as_bool().unwrap(), "{}", case["name"]);
        }

        let dir = tempdir().expect("tempdir");
        let config_path = dir.path().join("config.toml");
        std::fs::write(
            &config_path,
            r#"
model = "custom-chat"
model_provider = "custom"

[model_providers.custom]
base_url = "http://127.0.0.1:58424/v1"
wire_api = "responses"
env_key = "CUSTOM_TOKEN"
request_max_retries = 2

[model_providers.custom.env_http_headers]
x-tenant = "CUSTOM_TENANT"
"#,
        )
        .expect("write Codex config");

        let env = |name: &str| match name {
            "CUSTOM_TOKEN" => Some("token".to_string()),
            "CUSTOM_TENANT" => Some("tenant".to_string()),
            _ => None,
        };
        assert!(codex_connection_is_configured_at(dir.path(), env));

        assert!(!codex_connection_is_configured_at(dir.path(), |name| {
            (name == "CUSTOM_TOKEN").then(|| "token".to_string())
        }));

        std::fs::write(
            &config_path,
            r#"
model_provider = "custom"
[model_providers.custom]
base_url = "not-a-url"
experimental_bearer_token = "unsafe"
"#,
        )
        .expect("write invalid Codex config");
        assert!(!codex_connection_is_configured_at(dir.path(), |_| None));
    }
}
