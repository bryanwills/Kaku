---
title: AI Provider Configuration
description: Configure Copilot, Codex, Gemini, and custom OpenAI-compatible providers
---

## Built-in provider presets

Run `kaku ai` to open the AI settings panel. The Provider dropdown lists built-in presets. Picking one fills the Base URL, auth type, and model list.

| Provider | Base URL | Auth | Models |
| :--- | :--- | :--- | :--- |
| Copilot | `https://api.githubcopilot.com` | Copilot login | `gpt-4.1`, `gpt-4.5`, `claude-sonnet-4-5`, `o4-mini` |
| Codex | `https://api.openai.com/v1` | Codex / OpenAI auth | `codex-mini-latest`, `o4-mini`, `o3` |
| Gemini | `https://generativelanguage.googleapis.com` | Gemini API key | `gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-2.0-flash` |
| Custom | Manual | API key | Free text |

## Custom provider

Pick **Custom** to manually enter a Base URL and API Key for any OpenAI-compatible endpoint.

The endpoint needs to support the Chat Completions API. A typical config:

```toml
enabled = true
api_key = "sk-..."
model = "your-fast-model"
chat_model = "your-strong-model"
base_url = "https://your-gateway.example.com/v1"
```

## Inline model and chat model

Kaku uses AI in two places:

- `model`: error recovery and natural language to command. Use a fast, cheaper model.
- `chat_model`: AI Chat opened with `Cmd + L`. Use a stronger model if you want better agent behavior.

If `chat_model_choices` is set, AI Chat cycles only through those models. Otherwise Kaku tries to read available models from the provider's `/models` endpoint.

```toml
chat_model = "gpt-5.4"
chat_model_choices = ["gpt-5.4", "gpt-5.4-mini"]
```

## Enterprise proxies and extra headers

For company API gateways, add extra headers:

```toml
custom_headers = ["X-Customer-ID: your-id", "X-Org: your-org"]
```

`Authorization` and `Content-Type` are reserved and cannot be overridden.

## Web search

AI Chat web search is disabled by default. Enable Brave, PipeLLM, or Tavily in `kaku ai` and provide the matching API key.

```toml
web_search_provider = "brave"
web_search_api_key = "..."
```

When enabled, AI Chat can use search and page-reading tools. Only enable this with providers you trust, and avoid sending sensitive data to external services.
