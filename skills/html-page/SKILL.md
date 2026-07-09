---
name: html-page
mode: html
description: Create or improve a single-file HTML artifact with inline CSS and minimal JavaScript. Use whenever the user asks for a landing page, web page, article page, marketing page, microsite, or any standalone HTML you can open offline — even if they just say "make a page".
template: templates/basic.html
visualBrief: editorial
---

# html-page

You produce one fast, self-contained HTML file a human can open by double-clicking. No build step, no server, no framework. The goal is a page that looks designed the moment it renders — not a skeleton to style later.

## Visual brief

**定调**：暖纸感的杂志页，靠排版和留白说话，装饰极度克制——不是通用 web 模板。fewer colors, more whitespace; three colors and one accent is done, seven usually isn't.

具体可执行的视觉约束见 [`visual-briefs.md`](../visual-briefs.md) 的 `editorial` 小节。不要在 `--od-*` 之外另起一套 token。

## Expected output

```
index.html
assets/od-design.css
manifest.json
handoff.md
```

`index.html` links the stylesheet with `<link rel="stylesheet" href="assets/od-design.css">`. Every path is relative to the artifact root.

## How to build

1. Start from `templates/basic.html`; keep its `.od-*` primitives and add content rather than a parallel CSS system.
2. Style through existing `--od-*` tokens and `.od-*` classes. Add new `--od-*` variables when needed instead of raw hex in markup.
3. Use pure-CSS placeholders for images so the artifact opens offline.
4. Keep JavaScript minimal and only for real interaction.

## Constraints

- Keep JSX/TSX, Tailwind, shadcn/ui, and CDN UI kits out.
- Do not start a dev server; the artifact is the file.
- Do not depend on remote fonts or images.
- Keep at most one primary CTA.

## Self-check

- [ ] Opens without internet.
- [ ] Uses `--od-*` tokens and `.od-*` classes.
- [ ] Body copy respects `.od-container` measure.
- [ ] `:focus-visible` is visible on every interactive element.
