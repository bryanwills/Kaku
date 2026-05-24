# Kaku Static Website Design

## Visual Theme

Kaku uses a quiet blog-like paper surface: warm page background, white content blocks, dark text, restrained rules, and dark terminal/screenshot surfaces only where the product needs them. Avoid decorative grids, heavy shadows, oversized type, and card-heavy marketing composition.

## Palette

- `#f5f4ed` parchment page background
- `#ffffff` content surface
- `#24292f` primary text, brand, and actions
- `#0d7d4d` terminal green for command output only
- `#101013` terminal canvas
- `#ddd8c9` warm line and divider

## Typography

Use the blog-style Kai stack first: `TsangerJinKai02`, `STKaiti`, `KaiTi`, then system fallbacks. Do not add a font CDN by default. Use monospace only for terminal prompts, command snippets, and small technical labels.

## Components

Buttons are simple 8px-radius rectangles, at least 42px tall, with dark primary or white secondary styles. Cards are plain white blocks with borders and no decorative shadows. Terminal windows keep dark chrome but no extra visual effects.

## Layout

The landing page is product-first: Kaku appears as the first viewport signal, followed by the real screenshot, proof points, workflow sections, and final download CTA. Inner content widths stay close to the blog: 760px for prose and about 960px for page sections. Keep the top navigation minimal: docs, GitHub, and language only. Install details live in docs, while Roadmap and Changelog stay in the footer and docs index.

## Verification

Verify with a static server, desktop screenshot, 375px mobile screenshot, and link checking across Chinese and English pages before pushing.
