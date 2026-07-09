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

具体可执行的视觉约束（色板、字体、行宽、分隔、阴影、动效）见 [`visual-briefs.md`](../visual-briefs.md) 的 `editorial` 小节，照做即可。不要在 `--od-*` 之外另起一套 token。

## Expected output

```
index.html
assets/od-design.css
manifest.json
handoff.md          # intent + next steps + open questions
```

`index.html` links the stylesheet with `<link rel="stylesheet" href="assets/od-design.css">`. Every path is relative to the artifact root.

## How to build

1. Start from `templates/basic.html` — it already wires the CSS and uses the right primitives (`.od-container`, `.od-stack`, `.od-hero`, `.od-card`). Keep its class names; add content, not a parallel CSS system.
2. Style through the existing `--od-*` tokens and `.od-*` classes. When a token you need doesn't exist, add a new `--od-*` variable inside the CSS rather than dropping a raw hex into markup — so the next human/agent still has one token namespace.
3. For images, use pure-CSS placeholders (a `.od-frame` with a `--od-bg-surface` block and a label, or a gradient of two `--od-*` colors). Fetching remote images breaks the offline promise.
4. Keep JS minimal — only for genuine interaction (a menu toggle, a tab swap). The default happy path needs zero JS.

## Constraints — and why

- Keep JSX/TSX, Tailwind, shadcn/ui, or any CDN UI kit out. The contract is "plain file, opens offline, no build"; a framework makes the file un-openable without a toolchain.
- Do not start a dev server. The artifact *is* the file; `odl preview` handles viewing. A running process misrepresents what's being delivered.
- Do not depend on remote fonts or images. A file that breaks offline fails the core promise; system fonts and CSS placeholders already cover the editorial look.
- Do not maintain a second token system. Two namespaces drift and confuse the next editor — everything routes through `--od-*`.
- Keep at most one primary CTA; the rest are `data-variant="secondary"` or `ghost`. Multiple competing primaries dilute the page's point.

## Self-check before finishing

- [ ] Opens by double-click with no internet — fonts, images, CSS all local.
- [ ] Uses `--od-*` tokens exclusively in new CSS; no raw hex slipped into markup.
- [ ] At most one primary CTA on the page.
- [ ] Body copy stays readable (not edge-to-edge); it respects the `.od-container` measure.
- [ ] `:focus-visible` is visible on every link, button, and input.
- [ ] `odl preview` from this directory renders without errors.
- [ ] `handoff.md` says what the page is for and what's deliberately left as a next step.