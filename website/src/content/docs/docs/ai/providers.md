---
title: AI Provider 配置
description: 配置 Copilot、Codex、Gemini 和自定义 OpenAI-compatible Provider
---

## 内建 Provider 预设

打开 `kaku ai` 可以看到 Provider 下拉菜单。选择预设后，Kaku 会自动填入 Base URL、认证方式和可选模型。

| Provider | Base URL | 认证方式 | 模型 |
| :--- | :--- | :--- | :--- |
| Copilot | `https://api.githubcopilot.com` | Copilot 登录 | `gpt-4.1`、`gpt-4.5`、`claude-sonnet-4-5`、`o4-mini` |
| Codex | `https://api.openai.com/v1` | Codex / OpenAI 认证 | `codex-mini-latest`、`o4-mini`、`o3` |
| Gemini | `https://generativelanguage.googleapis.com` | Gemini API Key | `gemini-2.5-pro`、`gemini-2.5-flash`、`gemini-2.0-flash` |
| Custom | 手动填写 | API Key | 自由填写 |

## 自定义 Provider

选 "Custom" 手动填写任何兼容 OpenAI 协议的 Base URL 和 API Key。

自定义服务需要兼容 Chat Completions 接口。常见配置：

```toml
enabled = true
api_key = "sk-..."
model = "your-fast-model"
chat_model = "your-strong-model"
base_url = "https://your-gateway.example.com/v1"
```

## Inline 模型和 Chat 模型

Kaku 有两类 AI 使用场景：

- `model`：用于错误修复和自然语言转命令，建议选择速度快、成本低的模型。
- `chat_model`：用于 `Cmd + L` 打开的 AI Chat，可以选择更强的模型。

如果设置了 `chat_model_choices`，AI Chat 会只在这些模型之间切换。否则 Kaku 会尝试从 Provider 的 `/models` 接口读取可用模型。

```toml
chat_model = "gpt-5.4"
chat_model_choices = ["gpt-5.4", "gpt-5.4-mini"]
```

## 企业代理和额外 Header

如果公司网关需要额外 Header，可以配置：

```toml
custom_headers = ["X-Customer-ID: your-id", "X-Org: your-org"]
```

`Authorization` 和 `Content-Type` 是保留 Header，不能覆盖。

## Web Search

AI Chat 的网页搜索默认关闭。需要时可以在 `kaku ai` 中启用 Brave、PipeLLM 或 Tavily，并配置对应 API Key。

```toml
web_search_provider = "brave"
web_search_api_key = "..."
```

启用后，AI Chat 可以使用搜索和读取网页工具。只把它用于你信任的 Provider，并避免把敏感信息发给外部服务。
