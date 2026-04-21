---
title: 错误自动修复
description: 命令失败时，Kaku 自动建议修复方案
---

## 工作原理

当 shell 命令以非零退出码结束时，Kaku Assistant 会收集这次失败的上下文：

- 失败命令。
- 退出码。
- 当前工作目录。
- 当前 Git 分支。
- 终端中可见的错误输出。

这些信息会发送给你在 `kaku ai` 中配置的 LLM Provider。模型返回建议后，Kaku 会在终端内显示修复说明和候选命令。

## 应用建议

按 `Cmd + Shift + E` 可以把建议命令粘贴回终端提示符。Kaku 不会替你直接执行命令，你仍然需要审查后按回车。

危险命令会被标记为需要特别确认，例如删除文件、重置 Git 历史或修改权限的命令。它们可以被加载到提示符中，但不会自动执行。

## 什么时候不会触发

为了减少噪音，下面这些场景不会自动请求模型：

- 你用 `Ctrl + C` 主动中断命令。
- 命令只是 `--help`、`-h` 等帮助输出。
- 裸包管理器命令，例如只输入 `npm`、`brew`。
- `git pull` 冲突等需要人工决策的状态。
- 前台进程不是常规 shell 命令。

## 配置入口

运行：

```sh
kaku ai
```

打开 Kaku Assistant，选择 Provider，填写 API Key。也可以直接编辑 `~/.config/kaku/assistant.toml`：

```toml
enabled = true
model = "gpt-5.4-mini"
base_url = "https://api.openai.com/v1"
# api_key = "<your_api_key>"
```

如果想完全关闭错误修复，把 `enabled` 设为 `false`。
