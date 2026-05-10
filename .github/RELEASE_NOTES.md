# V0.10.0 Chat 🪄

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.10.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **AI Chat**: One engine, two surfaces: `Cmd+L` opens the in-terminal panel inside Kaku, while `k` or `kaku chat` drops the same engine into an alternate-screen TUI that works in any terminal or over SSH, with streaming Markdown, syntax highlighting, shell context, project tools, web search, memory, theme detection, safer cancel and approval semantics, and inline `#` queries that land in shell history.
2. **AI Configuration and Safety**: Assistant settings now use Simple and Deep models per task with live model loading, proxy-aware requests, OAuth setup, and broader provider responses, while stricter shell approvals, sensitive-path guards covering search and project tools, tighter file write and patch limits, failed-command context, and clearer parse errors keep the engine inside safe lines.
3. **Smart Close Protection**: `Cmd+W` and `Cmd+Shift+W` now ask before killing a pane that runs claude, codex, cursor-agent, gemini, vim, cargo, npm, or any non-shell process, while bare shells still close silently.
4. **Window Snapshots**: Kaku auto-saves multi-tab and multi-pane layouts, restored via `Cmd+Option+Shift+T`, Shell → Restore Previous Window, or the Command Palette.
5. **AppleScript Dictionary**: Kaku ships a full AppleScript dictionary, so windows, tabs, and panes are scriptable from Shortcuts, Hammerspoon, or any automation tool that drives macOS apps.
6. **Animated Tab Drag**: Drag a tab and the neighbors slide into place instead of snapping, so reordering reads as one fluid gesture.
7. **Softer Dark Theme**: Kaku Dark now uses lower-saturation highlight colors and a slightly dimmed foreground, reducing glare on long sessions while keeping the Aura aesthetic.
8. **Cold Start and Shell Speed**: Faster startup through Lua bytecode caching, deferred font and config initialization, and cached shell user vars.
9. **Background Updates**: Updates download in the background and fail closed on checksum mismatch, with more reliable proxy and MacPorts detection along the way.
10. **Bug Fixes**: Fullscreen crashes, display races, resize gaps, cursor reflow, links, selection, light-theme readability, and TUI copy are all addressed.

### 更新日志

1. **AI 对话**：同一套引擎，两种入口：`Cmd+L` 在 Kaku 内打开对话面板，执行 `k` 或 `kaku chat` 把同一套引擎放进 alternate-screen TUI，任何终端或 SSH 远端都能用，流式 Markdown、语法高亮、shell 上下文、项目工具、网页搜索、本地记忆样样齐全，主题自动识别、取消和审批语义更稳，`#` 查询自动存入 shell 历史。
2. **AI 配置与安全**：Assistant 设置改为 Simple Model 与 Deep Model，支持在线模型加载、代理感知请求、OAuth 配置以及更多 provider 响应格式，shell 审批更严，敏感路径保护扩展到搜索类工具，文件写入与 patch 上限收紧，失败命令上下文和解析错误一并更稳。
3. **智能关闭保护**：`Cmd+W` 和 `Cmd+Shift+W` 在 pane 里跑着 claude、codex、cursor-agent、gemini、vim、cargo、npm 这类有状态进程时会先弹确认，bare shell 仍然直接关。
4. **窗口快照**：Kaku 自动保存多 tab、多 pane 布局，需要时按 `Cmd+Option+Shift+T`，或从 Shell → Restore Previous Window、命令面板恢复。
5. **AppleScript 字典**：Kaku 内置完整 AppleScript dictionary，窗口、tab、pane 都能被 Shortcuts、Hammerspoon 等自动化工具脚本化驱动。
6. **拖拽标签页动画**：拖动 tab 时相邻 tab 会平滑让位，不再生硬切换，重新排序变成一个连贯的动作。
7. **深色主题更柔和**：Kaku Dark 默认调色板降低了高亮色饱和度，前景文字微微调暗，长时间盯屏更不刺眼，整体仍然是 Aura 风格。
8. **冷启动与 Shell 速度**：通过 Lua 字节码缓存、字体与配置延迟初始化、shell 用户变量缓存，启动更轻。
9. **后台更新**：更新改为后台下载，checksum 不通过则失败关闭，代理与 MacPorts 检测一并更稳。
10. **问题修复**：修复全屏崩溃和卡住、显示器竞态、resize 缝隙、光标 reflow、链接、选择、浅色主题可读性和 TUI 复制。

Special thanks to @s010s, @SherlockSalvatore, @darion-yaphet, @ddotz, @beautifulrem, @yxspace, and @fanweixiao for their contributions to this release.

> https://github.com/tw93/Kaku
