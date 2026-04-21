---
title: Natural Language to Command
description: Describe what you want in plain English, Kaku generates the shell command
---

## Usage

Type `#` followed by a natural-language description at the shell prompt and press Enter. Kaku asks the LLM to translate it into a shell command and injects the result back into the prompt for you to review before running.

```sh
# list all files modified in the last 7 days
# find and kill the process on port 3000
# compress the src folder excluding node_modules
```

Chinese prompts work too:

```sh
# 找出最近 7 天修改过的所有 Markdown 文件
# 查看 3000 端口被哪个进程占用
# 打包 src 目录但排除 node_modules
```

## Review before running

The generated command is inserted into the prompt only. It is not executed automatically. You can:

- Press Enter to run it.
- Edit the command first.
- Press `Ctrl + C` to discard it.

If the model cannot produce a safe command, Kaku inserts a short explanation instead of forcing a command.

## Supported shells

The `#` workflow works in zsh and fish. Kaku sends the current directory and Git branch as context so the model can generate a command that fits the current project.

## Difference from error recovery

- Natural language to command is user-initiated. It turns your intent into a shell command.
- Error recovery is automatic after a failed command. It explains and suggests a fix.

Both use the same Kaku Assistant configuration from `kaku ai`.
