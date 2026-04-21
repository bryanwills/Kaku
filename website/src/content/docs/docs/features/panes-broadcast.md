---
title: 分屏与广播输入
description: 分屏布局和多 pane 广播输入
---

## 分屏

| 操作 | 快捷键 |
| :--- | :--- |
| 垂直分屏 | `Cmd + D` |
| 水平分屏 | `Cmd + Shift + D` |
| 切换分屏方向 | `Cmd + Shift + S` |
| 放大 / 还原当前 pane | `Cmd + Shift + Enter` |
| 在 pane 之间移动焦点 | `Cmd + Opt + 方向键` |
| 调整 pane 大小 | `Cmd + Ctrl + 方向键` |

新分屏默认继承当前工作目录。也可以在 Lua 中显式设置：

```lua
config.split_pane_inherit_working_directory = true
```

## 广播输入

广播输入用于把同一段键盘输入发送到多个 pane。适合同时在多个服务目录运行相同命令，或在多个远程主机上执行一致的检查。

| 模式 | 快捷键 | 范围 |
| :--- | :--- | :--- |
| 当前标签广播 | `Cmd + Opt + I` | 当前 tab 内所有 pane |
| 全部标签广播 | `Cmd + Shift + I` | 当前窗口内所有 tab 的 pane |

再次按相同快捷键会关闭对应广播模式。Kaku 只会从当前活动 pane 的输入开始广播，overlay、搜索框等非终端输入不会被广播到其他 pane。

## 使用建议

- 先在一个 pane 中确认命令正确，再开启广播。
- 对删除、重置、部署这类命令保持谨慎。
- 广播模式只负责复制输入，不会替你确认远端命令是否安全。

完整快捷键见 [快捷键](/docs/config/keybindings/)。
