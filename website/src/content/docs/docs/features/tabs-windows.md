---
title: 标签与窗口
description: 标签栏、窗口管理
---

## 常用快捷键

| 操作 | 快捷键 |
| :--- | :--- |
| 新建窗口 | `Cmd + N` |
| 新建标签 | `Cmd + T` |
| 关闭当前 pane / 标签 / 隐藏应用 | `Cmd + W` |
| 关闭当前标签 | `Cmd + Shift + W` |
| 恢复关闭的标签 | `Cmd + Shift + T` |
| 切换到标签 1 到 9 | `Cmd + 1` 到 `Cmd + 9` |
| 上一个 / 下一个标签 | `Cmd + Shift + [` / `Cmd + Shift + ]` |
| 全屏 | `Cmd + Ctrl + F` |
| 全局窗口 | `Cmd + Opt + Ctrl + K` |

`Cmd + W` 是智能关闭：如果当前 tab 有多个 pane，会关闭 pane；否则在多 tab 或多窗口时关闭 tab；如果只剩一个窗口一个 tab，则隐藏应用。

## 标签栏行为

默认只有一个标签时隐藏标签栏。你可以在 Lua 中调整：

```lua
config.tab_bar_at_bottom = false
config.tab_title_show_basename_only = true
```

`tab_title_show_basename_only` 会把标签标题简化为当前目录名，更适合同时打开多个项目时快速识别。

## 工作目录继承

Kaku 默认让新窗口、新标签和新分屏继承当前工作目录。也可以显式设置：

```lua
config.window_inherit_working_directory = true
config.tab_inherit_working_directory = true
config.split_pane_inherit_working_directory = true
```

## 标签重命名

双击标签标题可以直接重命名。脚本里也可以使用：

```sh
kaku cli set-tab-title "api-server"
```

更多窗口和 pane 操作见 [快捷键](/docs/config/keybindings/) 和 [CLI 命令](/docs/reference/cli/)。
