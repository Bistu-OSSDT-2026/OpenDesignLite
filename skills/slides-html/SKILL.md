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

**定调**：画廊墙 / 设计评图板——每一页是单个想法的安静舞台，留白本身就是设计，不是空隙。one idea per slide, and motion steals attention — if you want a "build", make more slides instead of animating.

具体可执行的视觉约束（安静 surface、低密度、视觉优先、accent 作记号、零动效、一致 16:9 画框）见 [`visual-briefs.md`](../visual-briefs.md) 的 `studio` 小节，照做即可。

## Expected output

```
slides.html
assets/od-design.css
manifest.json
handoff.md          # speaker intent + follow-up ideas + what got cut
```

## Deck structure

- **Slide 1** — title slide: deck title, one-line subtitle, optional speaker/date. Big, centered, mostly empty.
- **Body slides** — each makes a single, self-contained point. A reader who sees only that slide should get it. If a slide needs more than ~25 words of body text, split it.
- **Section dividers** (when the deck has sections) — sparse: a section title and a number or a one-line preview. Nothing else.
- **Final slide** — a clear closer: takeaways, contact, or a question prompt. Not a "Thank you" wall.
- Keep the deck tight: prefer 8–15 slides over 30.

## How to build

1. Start from `templates/basic.html` — it already sets up `.od-slide-deck`, `.od-slide`, `.od-slide__inner`, the CSS link, and a two-sample starter. Keep those classes; add slides, not a new layout system.
2. Add a minimal keyboard-nav script (Space/PageDown/→ advance, ← back). This is the one place JS is expected and welcome. No scroll-jacking, no transitions.
3. For placeholder visuals use `.od-frame` with `--od-bg-surface` or a `linear-gradient` of two `--od-*` colors — never remote URLs.
4. Wire all styling through `--od-*` tokens and `.od-*` classes; do not start a parallel CSS namespace.

## Constraints — and why

- Keep JSX/TSX, Tailwind, shadcn/ui, or any CDN UI kit out. The file must open offline with no build; frameworks break that.
- Do not start a dev server. `odl preview` is the viewer; the artifact is the file.
- Do not use remote fonts or images, or a CDN. A deck that fails offline is broken at its core promise.
- Do not let a slide overflow. A slide that scrolls or clips is a failed slide — split or cut, don't cram.
- Keep transitions static by default. Motion distracts from the idea and breaks static export / PDF capture; use more slides instead.

## Self-check before finishing

- [ ] Every slide fits inside its 16:9 `.od-slide__inner` with no overflow.
- [ ] Keyboard nav works: Space/→ advances, ← goes back; no scroll needed.
- [ ] Pure-CSS placeholders only — no `<img src="http...">`, no remote font `<link>`.
- [ ] One accent (terracotta) used as punctuation, not as content-slide background.
- [ ] No slide carries more than ~25 body words.
- [ ] `odl preview` renders the deck without errors.
- [ ] `handoff.md` captures speaker intent and any slides you cut (with why, so a collaborator can restore them).