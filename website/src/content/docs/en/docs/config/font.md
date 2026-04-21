---
title: Fonts & Rendering
description: Font configuration and glyph rendering
---

## Default font

Kaku ships with JetBrains Mono, with rendering tuned specifically for macOS.

PingFang SC is used as the CJK fallback. Kaku also picks a default font size based on display density: usually 15 on low-resolution displays and 17 on high-resolution displays.

Ligatures are disabled by default so commands, paths, operators, and AI-generated shell snippets are easier to inspect character by character.

## Custom font

Point `config.font` at any monospace font you like. Fallback font chains are supported for CJK and emoji glyphs.

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font = wezterm.font("Fira Code")
config.font_size = 16

return config
```

For fallback chains, use WezTerm's font API:

```lua
config.font = wezterm.font_with_fallback({
  "JetBrains Mono",
  "PingFang SC",
  "Apple Color Emoji",
})
```

## Ligatures

Kaku disables ligatures by default. To turn them back on:

```lua
config.harfbuzz_features = {}
```

You can also use WezTerm's `harfbuzz_features` option to disable or enable specific font features.

## Line height and cursor

```lua
config.line_height = 1.28
config.default_cursor_style = "BlinkingBar"
config.cursor_thickness = "2px"
config.cursor_blink_rate = 500
```

## Font changes do not apply

First confirm the font is installed in macOS. After editing `kaku.lua`, open a new tab or restart Kaku. If it still does not apply, run:

```sh
kaku doctor
```

See [Lua Configuration](/en/docs/config/lua/) for more options.
