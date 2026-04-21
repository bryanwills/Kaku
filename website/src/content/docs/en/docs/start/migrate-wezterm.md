---
title: Migrating from WezTerm
description: A guide for WezTerm users switching to Kaku
---

## Zero-effort config migration

Your existing `~/.wezterm.lua` or `~/.config/wezterm/wezterm.lua` can be reused by Kaku as-is.

Kaku uses WezTerm's configuration system, so familiar Lua config, `wezterm.action`, font config, and color schemes continue to work.

The recommended path is to copy only the custom parts you need into `~/.config/kaku/kaku.lua` while keeping Kaku's bundled defaults loaded first.

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font_size = 16
config.window_background_opacity = 0.96

return config
```

Reusing a full `.wezterm.lua` can work, but it may overwrite Kaku's tuned keybindings, theme behavior, and window defaults.

## What's different

Kaku adds AI-focused configuration options on top of WezTerm and ships tuned defaults for macOS. See [Lua Configuration](/en/docs/config/lua/) for the Kaku-specific options.

The main differences:

- macOS-only, with less cross-platform surface area.
- JetBrains Mono by default, with PingFang SC as the CJK fallback.
- System-aware theme switching with `Kaku Dark` and `Kaku Light`.
- Built-in Kaku Assistant: error recovery, natural language to command, and AI Chat.
- `Cmd + Shift + G` opens Lazygit, `Cmd + Shift + Y` opens Yazi, and `Cmd + Shift + R` browses SSH remote files.
- `kaku config`, `kaku ai`, and `kaku doctor` manage configuration and diagnostics.

## Migration checklist

1. Install and open Kaku first, then try the defaults.
2. Copy only essential overrides such as font, transparency, and a few personal keybindings.
3. Append custom keybindings to `config.keys`; do not replace the whole table.
4. Run `kaku doctor` to verify shell integration.

See [Lua Configuration](/en/docs/config/lua/) and [CLI Commands](/en/docs/reference/cli/) for details.
