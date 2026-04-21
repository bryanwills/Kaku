---
title: Install
description: Install Kaku via DMG or Homebrew
---

## Via Homebrew

```sh
brew install tw93/tap/kakuku
```

There is an older unrelated Homebrew package named `kaku`. Use `tw93/tap/kakuku` to install the Kaku release from this repository.

## Via DMG

Grab the latest DMG from [GitHub Releases](https://github.com/tw93/Kaku/releases/latest) and drag Kaku into your Applications folder.

Kaku currently supports macOS only. The DMG is Apple-notarized, so it should open without security prompts in normal setups.

## First launch

Kaku configures your shell environment automatically, no extra setup required.

After launch, run:

```sh
kaku doctor
```

`kaku doctor` checks shell integration, command paths, optional tools, and common configuration issues.

## If the command is missing

If the app opens but your shell cannot find the `kaku` command, run:

```sh
/Applications/Kaku.app/Contents/MacOS/kaku init --update-only
exec zsh -l
```

Then run `kaku doctor` again.

## Updating

If you installed with Homebrew:

```sh
brew upgrade tw93/tap/kakuku
```

You can also run `kaku update` inside Kaku or use the menu bar updater.
