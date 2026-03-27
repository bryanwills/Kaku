# Features

## Kaku Assistant

Kaku Assistant monitors your shell for failed commands and automatically suggests a fix. When a command exits with an error, the assistant sends the context to an LLM and displays a safe correction inline.

**Setup**

Run `kaku ai` to open the AI settings panel. Enable Kaku Assistant, pick a provider, and enter your API key.

| Provider | Base URL | Models |
| :--- | :--- | :--- |
| VivGrid (default) | `https://api.vivgrid.com/v1` | DeepSeek-V3.2 |
| MiniMax | `https://api.minimax.io/v1` | MiniMax-M2.7, M2.7-highspeed, M2.5, M2.5-highspeed |
| OpenAI | `https://api.openai.com/v1` | (free text) |
| Custom | (manual) | (free text) |

Selecting a provider auto-fills the base URL and populates the model dropdown.

**assistant.toml fields**

The config lives at `~/.config/kaku/assistant.toml`:

| Field | Description |
| :--- | :--- |
| `enabled` | `true` to enable, `false` to disable |
| `api_key` | Your provider API key |
| `model` | Model identifier, e.g. `DeepSeek-V3.2` |
| `base_url` | OpenAI-compatible API root URL |
| `custom_headers` | Extra HTTP headers for enterprise proxies, e.g. `["X-Customer-ID: your-id"]` |

**Applying a suggestion**

When a suggestion is ready, press `Cmd + Shift + E` to paste it into the terminal. Dangerous commands (e.g. `rm -rf`, `git reset --hard`) are pasted but never auto-executed.

**What gets skipped**

The assistant does not trigger on: `Ctrl+C` exits, help flags, bare package manager calls, git pull conflicts, or non-shell foreground processes.

---

## Lazygit Integration

Press `Cmd + Shift + G` to launch lazygit in the current pane. Kaku auto-detects the lazygit binary from PATH or common Homebrew locations.

When a git repo has uncommitted changes and lazygit has not been used in that directory yet, Kaku shows a one-time hint to remind you it is available.

Install lazygit with `brew install lazygit` or via `kaku init`.

---

## Yazi File Manager

Press `Cmd + Shift + Y` to launch yazi in the current pane. The shell wrapper `y` also launches yazi and syncs the shell working directory on exit.

**Theme sync**: Kaku automatically updates `~/.config/yazi/theme.toml` to match the active color scheme (Kaku Dark or Kaku Light). No manual yazi theme setup needed.

Install yazi with `brew install yazi` or via `kaku init`.

---

## Remote Files

Press `Cmd + Shift + R` to mount the current SSH session's remote filesystem locally via `sshfs` and open it in yazi.

Kaku auto-detects the SSH target from the active pane. The mount lives at `~/Library/Caches/dev.kaku/sshfs/<host>`.

Requirements: `sshfs` installed (`brew install macfuse sshfs`) and passwordless SSH auth (key-based) for the remote host.

---

## Shell Suite

Kaku ships a curated set of shell plugins that load automatically inside Kaku sessions.

**Zsh plugins (built-in)**

- **z**: Smarter `cd` that learns your most-used directories. Use `z <dir>`, `z -l <dir>` to list matches, `z -t` for recent directories.
- **zsh-completions**: Extended completions for common CLI tools.
- **zsh-syntax-highlighting**: Real-time command coloring and error highlighting.
- **zsh-autosuggestions**: Fish-style history-based completions as you type.

**Fish support**

Run `kaku init` to provision `~/.config/kaku/fish/kaku.fish` for fish users. `kaku doctor` verifies both zsh and fish integration paths.

**Optional tools (installed via `kaku init`)**

- **Starship**: Fast, customizable prompt with git and environment info.
- **Delta**: Syntax-highlighting pager for git diff and grep.
- **Lazygit**: Terminal git UI.
- **Yazi**: Terminal file manager.

**Disabling Smart Tab**

If you use your own completion workflow like `fzf-tab`, add this before sourcing the Kaku shell integration:

```zsh
export KAKU_SMART_TAB_DISABLE=1
```

```fish
set -gx KAKU_SMART_TAB_DISABLE 1
```
