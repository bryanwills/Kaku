---
title: 自然语言转命令
description: 用自然语言描述需求，Kaku 生成对应的 shell 命令
---

## 使用方式

在 shell 提示符中输入 `#` 加一段自然语言描述，然后按回车。Kaku 会在 shell 执行之前拦截这行内容，调用 LLM 生成命令，并把结果注入回提示符。

```sh
# list all files modified in the last 7 days
# find and kill the process on port 3000
# compress the src folder excluding node_modules
```

中文也可以用：

```sh
# 找出最近 7 天修改过的所有 Markdown 文件
# 查看 3000 端口被哪个进程占用
# 打包 src 目录但排除 node_modules
```

## 审查后执行

生成结果只会回填到提示符，不会自动执行。你可以：

- 直接按回车执行。
- 修改命令后再执行。
- 用 `Ctrl + C` 放弃。

如果模型认为无法生成安全命令，Kaku 会注入一段简短说明，而不是强行给出命令。

## 支持的 Shell

`#` 前缀工作流支持 zsh 和 fish。当前目录和 Git 分支会作为上下文一起发送给模型，方便模型生成更贴近当前项目的命令。

## 和错误修复的区别

- 自然语言转命令是你主动发起，用来把意图变成命令。
- 错误自动修复是命令失败后自动触发，用来解释并修复失败原因。

两者使用同一个 Kaku Assistant 配置，入口都是 `kaku ai`。
