---
title: 从 WezTerm 迁移
description: WezTerm 用户切换到 Kaku 的完整指南
---

## 配置零迁移

Kaku 复用 WezTerm 的配置系统。你熟悉的 Lua 配置、`wezterm.action`、字体配置和配色方案都可以继续使用。

推荐迁移方式是把已有 WezTerm 配置里的自定义部分复制到 `~/.config/kaku/kaku.lua`，并保留 Kaku 的默认配置加载逻辑。

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

-- 复制你原来 .wezterm.lua 中真正需要保留的覆盖项
config.font_size = 16
config.window_background_opacity = 0.96

return config
```

如果你直接复用完整的 `.wezterm.lua`，也能工作，但可能会覆盖 Kaku 已经调好的默认快捷键、主题和窗口行为。

## 差异点

Kaku 是面向 macOS 和 AI 编码工作流的 WezTerm 分支，主要差异包括：

- macOS-only，减少跨平台包袱。
- 默认字体为 JetBrains Mono，中文 fallback 使用 PingFang SC。
- 默认主题跟随系统明暗模式，并提供 `Kaku Dark` 和 `Kaku Light`。
- 内建 Kaku Assistant：错误修复、自然语言转命令、AI Chat。
- `Cmd + Shift + G` 打开 Lazygit，`Cmd + Shift + Y` 打开 Yazi，`Cmd + Shift + R` 浏览 SSH 远程文件。
- 通过 `kaku config`、`kaku ai`、`kaku doctor` 管理配置和诊断。

## 迁移建议

1. 先安装并打开 Kaku，确认默认体验。
2. 只复制字体、透明度、少量自定义快捷键等必要覆盖项。
3. 自定义快捷键要追加到 `config.keys`，不要重新赋值整个表。
4. 运行 `kaku doctor` 检查 shell 集成。

详见 [Lua 配置](/docs/config/lua/) 和 [CLI 命令](/docs/reference/cli/)。
