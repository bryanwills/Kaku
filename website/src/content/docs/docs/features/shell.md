---
title: Shell 集成
description: Kaku 自动配置的 Shell 插件与工具
---

## 默认 Shell 插件

Kaku 首次启动和 `kaku init` 会配置 shell 集成。zsh 用户会获得一组内建插件：

- `z`：根据历史记录快速跳转常用目录。
- `zsh-completions`：补充常见 CLI 工具的 completion。
- `zsh-syntax-highlighting`：实时命令高亮。
- `zsh-autosuggestions`：基于历史命令的输入建议。

fish 用户运行 `kaku init` 后，会生成 `~/.config/kaku/fish/kaku.fish`。`kaku doctor` 会检查 zsh 和 fish 集成路径。

## 可选工具

`kaku init` 可以安装和配置下面这些常用工具：

- Starship：快速提示符。
- Delta：带语法高亮的 diff 和 grep pager。
- Lazygit：终端 Git UI。
- Yazi：终端文件管理器。

## Smart Tab

如果你使用自己的补全工作流，例如 `fzf-tab`，可以关闭 Kaku 的 Smart Tab。

zsh：

```zsh
export KAKU_SMART_TAB_DISABLE=1
```

fish：

```fish
set -gx KAKU_SMART_TAB_DISABLE 1
```

把这段放在加载 Kaku shell 集成之前。

## 诊断

如果 `y` 不能同步目录、`kaku` 命令丢失、completion 没生效，先运行：

```sh
kaku doctor
```

Doctor 会提示缺少的初始化片段和可修复项。
