---
name: slides-html
mode: slides
description: Create lightweight browser-previewable slides as a single HTML file. Use whenever the user asks for a slide deck, presentation slides, talk slides, pitch deck, or "slides for X" — even if they just say "make slides".
template: templates/basic.html
visualBrief: studio
---

# slides-html

You produce a compact HTML slide deck that previews instantly in a browser and exports cleanly. One idea per slide. No build tools, no framework, no server — the file is the deck.

## Visual brief

**定调**：画廊墙 / 设计评图板——每一页是单个想法的安静舞台，留白本身就是设计，不是空隙。

具体可执行的视觉约束见 [`visual-briefs.md`](../visual-briefs.md) 的 `studio` 小节。

## Expected output

```
slides.html
assets/od-design.css
manifest.json
handoff.md
```

## Deck structure

- Slide 1: title, one-line subtitle, optional speaker/date.
- Body slides: one self-contained point per slide.
- Section dividers: sparse title plus number or one-line preview.
- Final slide: takeaways, contact, or question prompt.

## How to build

1. Start from `templates/basic.html`; keep `.od-slide-deck`, `.od-slide`, and `.od-slide__inner`.
2. Add minimal keyboard navigation when useful.
3. Use pure-CSS placeholders instead of remote images.
4. Wire styling through `--od-*` tokens and `.od-*` classes.

## Constraints

- Keep JSX/TSX, Tailwind, shadcn/ui, and CDN UI kits out.
- Do not start a dev server.
- Do not let a slide overflow; split or cut instead.
- Keep transitions static by default.

## Preview

When the user asks to preview: call MCP `artifact_preview` with this artifact `dir`. Do **not** open a system browser yourself. Defaults (`externalBrowser` false, `watch` true) open the persistent live-reloading window. See [`preview-via-mcp`](../preview-via-mcp/SKILL.md).

## Self-check

- [ ] Every slide fits inside its 16:9 frame.
- [ ] Keyboard navigation works if included.
- [ ] No remote font, image, or script dependency.
- [ ] `handoff.md` captures speaker intent and cut material.
