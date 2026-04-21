---
title: 从 iTerm2 迁移
description: iTerm2 用户切换到 Kaku 的完整指南
---

## 熟悉的快捷键

Kaku 保留了 iTerm2 用户最常用的肌肉记忆：

| 操作 | Kaku 快捷键 |
| :--- | :--- |
| 新建标签 | `Cmd + T` |
| 新建窗口 | `Cmd + N` |
| 关闭当前 pane / 标签 | `Cmd + W` |
| 垂直分屏 | `Cmd + D` |
| 水平分屏 | `Cmd + Shift + D` |
| 切换标签 | `Cmd + Shift + [` / `Cmd + Shift + ]` |
| 跳转标签 1 到 9 | `Cmd + 1` 到 `Cmd + 9` |
| 清屏和 scrollback | `Cmd + K` |

`Cmd + W` 在 Kaku 中是智能行为：多 pane 时关闭当前 pane，多标签或多窗口时关闭当前标签，否则隐藏应用。

## 配置迁移

iTerm2 的 plist 配置不能直接导入。建议先使用 Kaku 默认值，再迁移真正需要的部分：

- 字体：在 `~/.config/kaku/kaku.lua` 中设置 `config.font` 和 `config.font_size`。
- 主题：使用 `config.color_scheme = "Kaku Dark"` 或 `"Kaku Light"` 固定明暗模式。
- 透明度：设置 `config.window_background_opacity`，需要时再加 `config.macos_window_background_blur`。
- 快捷键：用 `table.insert(config.keys, ...)` 追加，避免覆盖默认快捷键。

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font = wezterm.font("JetBrains Mono")
config.font_size = 16
config.window_background_opacity = 0.95

return config
```

## iTerm2 用户会注意到的差异

- Kaku 使用 WezTerm 的 Lua 配置系统，不使用 plist Profile。
- 主题默认跟随 macOS 明暗模式。
- Shell 集成由 `kaku init` 和 `kaku doctor` 管理。
- AI 功能是内建能力，包括错误修复、自然语言转命令和 AI Chat。
- Lazygit、Yazi、远程文件浏览等常用工作流有默认快捷键。

完整设置参考 [Lua 配置](/docs/config/lua/)。
