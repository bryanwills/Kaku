---
title: Migrating from iTerm2
description: A guide for iTerm2 users switching to Kaku
---

## Familiar shortcuts

Kaku keeps the iTerm2 keybindings you already have muscle memory for: `⌘T` new tab, `⌘W` close, `⌘D` split.

| Action | Kaku shortcut |
| :--- | :--- |
| New tab | `Cmd + T` |
| New window | `Cmd + N` |
| Close current pane / tab | `Cmd + W` |
| Split vertically | `Cmd + D` |
| Split horizontally | `Cmd + Shift + D` |
| Switch tabs | `Cmd + Shift + [` / `Cmd + Shift + ]` |
| Jump to tab 1 to 9 | `Cmd + 1` to `Cmd + 9` |
| Clear screen and scrollback | `Cmd + K` |

`Cmd + W` is smart: it closes the current pane when there are multiple panes, closes the current tab when there are multiple tabs or windows, and hides the app when only one tab remains.

## Configuration

iTerm2 plist profiles cannot be imported directly, but Kaku defaults are tuned to feel good out of the box. Start with the defaults, then migrate only the settings you actually miss:

- Font: set `config.font` and `config.font_size` in `~/.config/kaku/kaku.lua`.
- Theme: use `config.color_scheme = "Kaku Dark"` or `"Kaku Light"` to pin appearance.
- Transparency: set `config.window_background_opacity`, and optionally `config.macos_window_background_blur`.
- Keybindings: append with `table.insert(config.keys, ...)` so Kaku defaults stay intact.

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font = wezterm.font("JetBrains Mono")
config.font_size = 16
config.window_background_opacity = 0.95

return config
```

## Differences to expect

- Kaku uses WezTerm's Lua configuration system, not plist profiles.
- Theme follows macOS light/dark appearance by default.
- Shell integration is managed by `kaku init` and `kaku doctor`.
- AI features are built in: error recovery, natural language to command, and AI Chat.
- Lazygit, Yazi, and remote file browsing have default shortcuts.

See [Lua Configuration](/en/docs/config/lua/) for the full setup guide.
