---
title: 首次配置
description: Kaku 开箱即用，但你可以通过 Lua 深度定制
---

## 零配置是默认

Kaku 不需要任何配置就能用。默认字体、主题、窗口行为、Shell 集成和快捷键都已经按 macOS 开发场景调好。

首次启动时，Kaku 会自动创建 `~/.config/kaku/kaku.lua`。这个文件先加载应用内置的默认配置，再应用你的覆盖项，所以你只需要写真正想改的几行。

```lua
local wezterm = require "wezterm"
local config = require("kaku").config

config.font_size = 16
config.copy_on_select = false

return config
```

## 什么时候需要配置

大多数用户只需要改下面几类内容：

- 字体大小、字体族或连字设置。
- 强制深色 / 浅色主题，或者覆盖个别颜色。
- 增加自己的快捷键。
- 开关滚动条、透明度、窗口边距、复制行为。
- 配置 Kaku Assistant 的 Provider、模型和 API Key。

## 打开配置

有三种常用入口：

```sh
kaku config
```

- 在 Kaku 中按 `Cmd + ,` 打开设置面板。
- 直接编辑 `~/.config/kaku/kaku.lua`。
- AI 相关设置用 `kaku ai`，配置文件位于 `~/.config/kaku/assistant.toml`。

## 推荐顺序

1. 先直接使用默认配置一段时间。
2. 只改影响日常输入体验的项，例如字体大小和主题。
3. 追加快捷键时使用 `table.insert(config.keys, ...)`，不要把 `config.keys` 整个替换掉。
4. 修改后新开一个标签或窗口验证效果。

完整配置项见 [Lua 配置](/docs/config/lua/) 和 [快捷键](/docs/config/keybindings/)。
