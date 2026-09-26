# V0.21.0 Ink

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.21.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Official Homebrew Cask**: Kaku is finally in the official Homebrew cask, so `brew install --cask kaku` installs it directly, `kaku update` recognizes it, and installs from the personal tap keep updating too.
2. **Session Recovery**: Kaku now saves your windows, tabs, and panes while it runs, so a crash or force quit still brings back the last layout.
3. **Context Menu**: Copy appears at the top of the right-click menu when text is selected, so copying with the mouse works even with copy on select turned off.
4. **Tab Bar**: Tabs split into several panes now show their index number too.
5. **Themes**: New installs start in Kaku Dark while existing setups keep their current theme, and bright cyan in Kaku Light is easier to read.
6. **Stability and Security**: A tab that is closing can no longer take every window down, and the TLS library is updated for a security advisory.

### 更新日志

1. **官方 Homebrew**：Kaku 终于进了 Homebrew 官方 cask，直接用 `brew install --cask kaku` 就能安装，`kaku update` 也能识别，之前通过个人 tap 安装的照样能继续更新。
2. **会话恢复**：运行时会定期保存窗口、标签和分屏，崩溃或强制退出后也能恢复上次的布局。
3. **右键菜单**：选中文字后右键菜单顶部会出现复制，关掉选中即复制时也能用鼠标复制。
4. **标签栏**：有分屏的标签也会显示序号。
5. **主题**：新安装默认使用 Kaku Dark，已有安装保持原来的主题，Kaku Light 的亮青色也更容易看清。
6. **稳定与安全**：正在关闭的标签不会再导致所有窗口一起退出，并更新 TLS 库修复一个安全公告中的问题。

Special thanks to @TeamMeng for their contribution to this release.

> https://github.com/tw93/Kaku
