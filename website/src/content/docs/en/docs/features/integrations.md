---
title: Lazygit / Yazi Integration
description: One-key launchers for Lazygit and Yazi built into Kaku
---

## Lazygit

Press `⌘⇧G` to launch Lazygit in the current pane.

Kaku resolves `lazygit` from `PATH`, `/opt/homebrew/bin`, `/usr/local/bin`, and other common Homebrew locations.

When a Git repo has uncommitted changes and Lazygit has not been used in that directory yet, Kaku shows a one-time hint to remind you that the shortcut is available.

Install it with:

```sh
brew install lazygit
```

Or run `kaku init` to install optional CLI tools.

## Yazi

Press `⌘⇧Y`, or type `y` in the shell, to launch the Yazi file manager.

The `y` shell wrapper syncs your shell working directory to the directory where you exit Yazi. This depends on Kaku shell integration, so run `kaku doctor` if it does not work.

Kaku manages a theme block in `~/.config/yazi/theme.toml` so Yazi follows `Kaku Dark` or `Kaku Light`.

Install it with:

```sh
brew install yazi
```

Or run `kaku init`.

## Remote files

Inside an SSH session, press `Cmd + Shift + R` to detect the remote host, mount it locally with `sshfs`, and browse it in Yazi.

The mount path is:

```text
~/Library/Caches/dev.kaku/sshfs/<host>
```

Requirements:

- `macfuse` and `sshfs` are installed.
- The remote host supports key-based SSH login without an interactive password.
- The active pane contains a detectable SSH target.

```sh
brew install macfuse sshfs
```

If remote browsing fails, first confirm that plain `ssh <host>` works without a password prompt, then run `kaku doctor`.
