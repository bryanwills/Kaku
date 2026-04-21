---
title: Panes & Broadcast Input
description: Split layouts and broadcasting input across panes
---

## Splitting

| Action | Shortcut |
| :--- | :--- |
| Split vertically | `Cmd + D` |
| Split horizontally | `Cmd + Shift + D` |
| Toggle split direction | `Cmd + Shift + S` |
| Zoom / unzoom current pane | `Cmd + Shift + Enter` |
| Move focus between panes | `Cmd + Opt + Arrows` |
| Resize pane | `Cmd + Ctrl + Arrows` |

New splits inherit the current working directory by default. You can set it explicitly:

```lua
config.split_pane_inherit_working_directory = true
```

## Broadcast input

Send the same keystrokes to multiple panes at once. See [Keybindings](/en/docs/config/keybindings/) for the broadcast shortcuts.

| Mode | Shortcut | Scope |
| :--- | :--- | :--- |
| Current tab broadcast | `Cmd + Opt + I` | All panes in the current tab |
| All tabs broadcast | `Cmd + Shift + I` | Panes in every tab in the current window |

Press the same shortcut again to turn that broadcast mode off. Kaku only broadcasts terminal input from the active pane. Overlay input, search fields, and other non-terminal UI are not broadcast.

## Practical guidance

- Verify the command in one pane before enabling broadcast.
- Be careful with delete, reset, deploy, and permission-changing commands.
- Broadcast mode copies input only; it does not judge whether a remote command is safe.

See [Keybindings](/en/docs/config/keybindings/) for the full shortcut list.
