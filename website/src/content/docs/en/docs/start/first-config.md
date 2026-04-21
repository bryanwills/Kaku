---
title: First Configuration
description: Kaku works out of the box, but you can customize everything with Lua
---

## Zero config by default

Kaku runs with no configuration at all: the default font, color scheme, and keybindings are tuned to feel great on day one.

On first launch, Kaku creates `~/.config/kaku/kaku.lua`. That file loads the bundled Kaku defaults first, then applies your overrides, so you only need to write the lines you actually want to change.

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font_size = 16
config.copy_on_select = false

return config
```

## When you need to configure

When you want to customize keybindings, colors, fonts, or an AI provider, head to [Lua Configuration](/en/docs/config/lua/).

Most users only touch a small set of options:

- Font size, font family, or ligature behavior.
- Forced dark or light theme, or a few color overrides.
- Additional keybindings.
- Scrollbar, transparency, window padding, or copy-on-select behavior.
- Kaku Assistant provider, model, and API key.

## Open configuration

Common entry points:

```sh
kaku config
```

- Press `Cmd + ,` in Kaku to open the settings panel.
- Edit `~/.config/kaku/kaku.lua` directly.
- Use `kaku ai` for AI settings. The assistant config lives at `~/.config/kaku/assistant.toml`.

## Recommended order

1. Use the defaults for a while first.
2. Change only the options that affect daily input comfort, such as font size and theme.
3. Add keybindings with `table.insert(config.keys, ...)`; do not replace the whole `config.keys` table.
4. Open a new tab or window to verify the change.

See [Lua Configuration](/en/docs/config/lua/) and [Keybindings](/en/docs/config/keybindings/) for the full reference.
