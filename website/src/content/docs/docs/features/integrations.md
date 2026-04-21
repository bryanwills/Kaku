---
title: Lazygit / Yazi 集成
description: Kaku 内建 Lazygit 和 Yazi 快捷启动
---

## Lazygit

按 `Cmd + Shift + G` 在当前 pane 中打开 Lazygit。Kaku 会从 `PATH`、`/opt/homebrew/bin` 和 `/usr/local/bin` 等常见位置查找 `lazygit`。

如果当前 Git 仓库有未提交修改，而你还没有在这个目录里用过 Lazygit，Kaku 会给出一次性提示，提醒你可以用快捷键打开。

安装方式：

```sh
brew install lazygit
```

或运行 `kaku init` 让 Kaku 安装可选 CLI 工具。

## Yazi

按 `Cmd + Shift + Y` 打开 Yazi 文件管理器。也可以在 shell 中输入：

```sh
y
```

`y` 包装命令会在退出 Yazi 后把 shell 的当前目录同步到你最后停留的位置。这个能力依赖 Kaku 的 shell 集成，如果没有生效，先运行 `kaku doctor`。

Kaku 会管理 `~/.config/yazi/theme.toml` 中的主题块，使 Yazi 跟随 `Kaku Dark` 或 `Kaku Light`。

安装方式：

```sh
brew install yazi
```

或运行 `kaku init`。

## 远程文件

在 SSH 会话中按 `Cmd + Shift + R`，Kaku 会尝试识别当前远程主机，通过 `sshfs` 挂载到本机，并用 Yazi 浏览。

挂载目录位于：

```text
~/Library/Caches/dev.kaku/sshfs/<host>
```

前置条件：

- 安装 `macfuse` 和 `sshfs`。
- 远程主机支持基于密钥的免密 SSH。
- 当前 pane 中能识别出 SSH 目标。

```sh
brew install macfuse sshfs
```

如果远程文件浏览失败，先确认普通 `ssh <host>` 能免密登录，再运行 `kaku doctor` 检查环境。
