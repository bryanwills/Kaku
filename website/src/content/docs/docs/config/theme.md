---
title: 主题与配色
description: Kaku 的主题切换与自定义配色
---

## 跟随系统

Kaku 默认跟随 macOS 系统明暗模式。系统切到深色时使用 `Kaku Dark`，切到浅色时使用 `Kaku Light`。

这个默认行为适合大多数用户，特别是白天浅色、夜间深色的 macOS 设置。

## 手动切换

在 `~/.config/kaku/kaku.lua` 中设置 `config.color_scheme` 可以固定主题：

```lua
local config = require("kaku").config

config.color_scheme = "Kaku Dark"
-- config.color_scheme = "Kaku Light"

return config
```

## 颜色覆盖

某些 CLI 工具会输出自己的硬编码颜色。如果你想把某个颜色映射到 Kaku 主题里的另一个颜色，可以使用 `config.color_overrides`：

```lua
config.color_overrides = {
  ["#6E6E6E"] = "#3A3942",
}
```

这适合微调提示符、diff、日志或 AI 工具输出中的个别颜色。

## 透明和模糊

```lua
config.window_background_opacity = 0.92
config.macos_window_background_blur = 20
```

`macos_window_background_blur` 取值通常在 `0` 到 `100` 之间。透明度过低会影响文本可读性，建议先从 `0.92` 到 `0.98` 之间调试。

## 交通灯按钮

Kaku 默认把 macOS 关闭、最小化、缩放按钮整合到标签栏区域。如果你想隐藏按钮，但保留窗口边缘缩放和拖拽：

```lua
config.window_decorations = "RESIZE"
```

更多外观设置见 [Lua 配置](/docs/config/lua/)。
