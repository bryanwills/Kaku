---
title: Shell Integration
description: Shell plugins and tools that Kaku configures automatically
---

## Default shell plugins

On first launch, Kaku configures your shell and preloads a curated set of commonly-used zsh plugins.

The built-in zsh plugins are:

- `z`: jump to frequently used directories from history.
- `zsh-completions`: extra completions for common CLI tools.
- `zsh-syntax-highlighting`: real-time command highlighting.
- `zsh-autosuggestions`: history-based suggestions as you type.

Fish users can run `kaku init` to generate `~/.config/kaku/fish/kaku.fish`. `kaku doctor` checks both zsh and fish integration paths.

## Optional tools

`kaku init` can install and configure:

- Starship: fast prompt.
- Delta: syntax-highlighted diff and grep pager.
- Lazygit: terminal Git UI.
- Yazi: terminal file manager.

## Smart Tab

If you use your own completion workflow, such as `fzf-tab`, disable Kaku Smart Tab.

zsh:

```zsh
export KAKU_SMART_TAB_DISABLE=1
```

fish:

```fish
set -gx KAKU_SMART_TAB_DISABLE 1
```

Place it before loading Kaku shell integration.

## Diagnostics

If `y` does not sync directories, the `kaku` command is missing, or completions do not load, run:

```sh
kaku doctor
```

Doctor reports missing init snippets and fixable setup issues.
