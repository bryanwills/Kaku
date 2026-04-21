---
title: Tabs & Windows
description: Tab bar and window management
---

## Common shortcuts

| Action | Shortcut |
| :--- | :--- |
| New window | `Cmd + N` |
| New tab | `Cmd + T` |
| Close current pane / tab / hide app | `Cmd + W` |
| Close current tab | `Cmd + Shift + W` |
| Reopen closed tab | `Cmd + Shift + T` |
| Switch to tab 1 to 9 | `Cmd + 1` to `Cmd + 9` |
| Previous / next tab | `Cmd + Shift + [` / `Cmd + Shift + ]` |
| Fullscreen | `Cmd + Ctrl + F` |
| Toggle global window | `Cmd + Opt + Ctrl + K` |

`Cmd + W` is smart: it closes the current pane if there are multiple panes, closes the tab if there are multiple tabs or windows, and hides the app if only one window and one tab remain.

## Tab bar behavior

The tab bar is hidden when only one tab is open. You can tune it in Lua:

```lua
config.tab_bar_at_bottom = false
config.tab_title_show_basename_only = true
```

`tab_title_show_basename_only` shortens tab titles to the current directory name, which is easier to scan when multiple projects are open.

## Working directory inheritance

Kaku makes new windows, tabs, and splits inherit the current working directory by default. You can set this explicitly:

```lua
config.window_inherit_working_directory = true
config.tab_inherit_working_directory = true
config.split_pane_inherit_working_directory = true
```

## Rename tabs

Double-click a tab title to rename it inline. Scripts can also call:

```sh
kaku cli set-tab-title "api-server"
```

See [Keybindings](/en/docs/config/keybindings/) and [CLI Commands](/en/docs/reference/cli/) for more window and pane operations.
