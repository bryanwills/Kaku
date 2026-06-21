# V0.12.3 Cleaner

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.12.3</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **System Proxy**: AI requests to local and self-hosted models no longer fail when a system proxy is on; private and loopback addresses now connect directly.
2. **Terminal Redraw**: Scrolling a selection list in tools like the skills picker no longer leaves garbled rows behind.
3. **Credentials**: API keys and tokens are no longer passed on the command line, so they stay out of process listings.
4. **Transparency**: With window transparency enabled, the top strip and rounded corners match the rest of the window instead of showing a darker band or black corners.
5. **Path Highlight**: An existing directory in the command line no longer carries a stray underline, including the doubled one that appeared on hover.

### 更新日志

1. **系统代理**：开启系统代理时，访问本地和自托管模型的 AI 请求不再失败，私有地址和 loopback 会直连。
2. **终端重绘**：在 skills 选择器这类工具里上下滚动列表，不再残留错乱的行。
3. **凭证**：API key 和 token 不再通过命令行参数传递，不会出现在进程列表里。
4. **透明度**：开启窗口透明后，顶部条和圆角会与窗口其余部分一致，不再出现更深的色带或黑色角块。
5. **路径高亮**：命令行里已存在的目录不再带一条多余的下划线，悬停时也不会再叠出第二条。

> https://github.com/tw93/Kaku
