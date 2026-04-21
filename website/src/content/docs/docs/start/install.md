---
title: 安装
description: 通过 DMG 或 Homebrew 安装 Kaku
---

## 通过 Homebrew

```sh
brew install tw93/tap/kakuku
```

Homebrew 上存在一个早期的同名 `kaku` 包。请使用 `tw93/tap/kakuku`，这样安装到的是这个仓库发布的 Kaku。

## 通过 DMG

访问 [GitHub Releases](https://github.com/tw93/Kaku/releases/latest) 下载最新版本的 DMG 文件，拖到应用程序文件夹即可。

Kaku 当前只支持 macOS。DMG 已经过 Apple 公证，正常情况下不会出现系统安全拦截。

## 首次启动

Kaku 会自动配置你的 shell 环境，无需额外步骤。

启动后建议运行：

```sh
kaku doctor
```

`kaku doctor` 会检查 shell 集成、命令路径、可选工具和常见配置问题。

## 如果命令丢失

如果 App 能打开，但终端里找不到 `kaku` 命令，可以运行：

```sh
/Applications/Kaku.app/Contents/MacOS/kaku init --update-only
exec zsh -l
```

然后再执行 `kaku doctor` 验证。

## 更新

使用 Homebrew 安装时：

```sh
brew upgrade tw93/tap/kakuku
```

也可以在 Kaku 内运行 `kaku update` 或使用菜单栏更新入口。
