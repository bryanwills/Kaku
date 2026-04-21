---
title: Automatic Error Recovery
description: When a command fails, Kaku suggests a fix
---

## How it works

When a shell command exits with a non-zero status, Kaku sends the error output to your configured LLM and asks for a suggested fix. Press `⌘⇧E` to apply the suggestion in one keystroke.

Kaku Assistant collects the relevant context for that failed command:

- The failed command.
- Exit code.
- Current working directory.
- Current Git branch.
- Visible terminal error output.

That context is sent to the LLM provider configured in `kaku ai`. When the model returns a suggestion, Kaku displays the explanation and candidate command inline in the terminal.

## Applying a suggestion

Press `Cmd + Shift + E` to paste the suggested command back into the prompt. Kaku does not execute it for you; you still review the command and press Enter yourself.

Dangerous commands are marked for extra review, for example commands that delete files, reset Git history, or change permissions. They may be loaded into the prompt, but they are never auto-executed.

## When it does not trigger

To reduce noise, Kaku does not call the model for:

- Commands interrupted by `Ctrl + C`.
- Help flags such as `--help` or `-h`.
- Bare package manager invocations such as just `npm` or `brew`.
- `git pull` conflict states that require human judgment.
- Foreground processes that are not regular shell commands.

## Configuration

Run:

```sh
kaku ai
```

Enable Kaku Assistant, choose a provider, and enter your API key. You can also edit `~/.config/kaku/assistant.toml` directly:

```toml
enabled = true
model = "gpt-5.4-mini"
base_url = "https://api.openai.com/v1"
# api_key = "<your_api_key>"
```

Set `enabled = false` to disable automatic error recovery.
