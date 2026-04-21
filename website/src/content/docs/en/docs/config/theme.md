---
title: Theme & Colors
description: Theme switching and custom colors in Kaku
---

## Follow system

By default, Kaku follows the macOS light/dark appearance and switches automatically.

It uses `Kaku Dark` in dark mode and `Kaku Light` in light mode. This default fits most setups, especially if your Mac switches appearance automatically during the day.

## Manual override

Set `config.color_scheme` in `~/.config/kaku/kaku.lua` to pin one theme:

```lua
local config = require("kaku").config

config.color_scheme = "Kaku Dark"
-- config.color_scheme = "Kaku Light"

return config
```

## Color overrides

Some CLI tools output hard-coded colors. Use `config.color_overrides` to remap a specific color while keeping the rest of the theme intact:

```lua
config.color_overrides = {
  ["#6E6E6E"] = "#3A3942",
}
```

This is useful for prompts, diffs, logs, and AI tool output that need one or two color adjustments.

## Transparency and blur

```lua
config.window_background_opacity = 0.92
config.macos_window_background_blur = 20
```

`macos_window_background_blur` is usually between `0` and `100`. Very low opacity hurts readability, so start around `0.92` to `0.98`.

## Traffic light buttons

Kaku integrates the macOS close, minimize, and zoom buttons into the tab bar area by default. To hide them while keeping resize edges and tab-bar dragging:

```lua
config.window_decorations = "RESIZE"
```

See [Lua Configuration](/en/docs/config/lua/) for more appearance options.
