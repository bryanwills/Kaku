---
title: 字体与渲染
description: 字体配置和字符渲染
---

## 默认字体

Kaku 默认使用 JetBrains Mono，并使用 PingFang SC 作为中文 fallback。默认字体大小会根据屏幕密度自动选择：低分屏通常是 15，高分屏通常是 17。

Kaku 也默认关闭连字，让命令、路径、运算符和 AI 生成的 shell 片段更容易逐字符检查。

## 自定义字体

在 `~/.config/kaku/kaku.lua` 中设置：

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font = wezterm.font("Fira Code")
config.font_size = 16

return config
```

如果需要 fallback 链，可以使用 WezTerm 的字体 API：

```lua
config.font = wezterm.font_with_fallback({
  "JetBrains Mono",
  "PingFang SC",
  "Apple Color Emoji",
})
```

## 连字

Kaku 默认禁用连字。想重新开启：

```lua
config.harfbuzz_features = {}
```

如果只想禁用特定特性，可以继续使用 WezTerm 的 `harfbuzz_features` 配置。

## 行高和光标

```lua
config.line_height = 1.28
config.default_cursor_style = "BlinkingBar"
config.cursor_thickness = "2px"
config.cursor_blink_rate = 500
```

## 字体修改没有生效

先确认字体已经安装在 macOS 中。修改 `kaku.lua` 后，新开标签或重启 Kaku 验证。如果仍然不生效，运行：

```sh
kaku doctor
```

更多可用项见 [Lua 配置](/docs/config/lua/)。
