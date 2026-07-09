# Smoke Prompts

**状态**：草案  
**里程碑**：M2  
**实现位置**：`docs/specs/smoke-prompts.md`

## 目的

定义固定生成提示词，用于验证外部 Agent 经 MCP/CLI 产出的 HTML、docs、slides 是否符合 design kernel 默认视觉质量。Smoke prompts 不要求 Agent 输出完全一致，但要求产物可预览、结构合理、遵守本地文件与 design kernel 约束。

## 评估标准

每条 prompt 输出必须满足：

- 不需要 dev server。
- 不默认引入 React、Tailwind、shadcn/ui、CDN UI kit。
- 使用 `--od-*` token 或 `od-design.css`。
- 可由 `odl preview` / MCP `artifact_preview` 打开。
- 移动端不明显破版。
- focus 状态可见。
- handoff 能说明下一步。

## Agent preview behavior

联调外部 Agent 时，预览路径应为 MCP `artifact_preview`（默认 `externalBrowser: false`），而不是 Agent 自行 `start` / `xdg-open` / `open` / Playwright 打开系统浏览器。人类手工验收仍可用 `odl preview`。

## Prompts

### HTML 1：Editorial Landing

```text
Create a calm editorial landing page for a local-first design assistant. Use warm neutral colors, strong typography, one hero, three proof points, and a compact CTA. Avoid SaaS blue gradients.
```

### HTML 2：Workbench Dashboard

```text
Create a lightweight dashboard for artifact previews. Include recent artifacts, status cards, and a small activity list. Use a workbench visual brief and keep the layout useful on mobile.
```

### HTML 3：Gallery Board

```text
Create a gallery-style page for design experiments. Use image placeholders, captions, filters, and an empty state. Keep it static and editable as plain HTML.
```

### Docs 1：PRD Polish

```text
Polish this rough product requirements document into a clear Markdown spec. Preserve technical facts, add headings, open questions, and acceptance criteria.
```

### Docs 2：Architecture Summary

```text
Turn these architecture notes into a concise Markdown document with module boundaries, data flow, risks, and decisions. Avoid marketing language.
```

### Slides 1：Founder Pitch

```text
Create a 7-slide HTML deck for a lightweight local-first design assistant. Include problem, insight, product, workflow, architecture, roadmap, and ask.
```

### Slides 2：Technical Demo

```text
Create a compact technical demo deck showing how an artifact moves from prompt to files to preview to handoff. Use large readable type and print-friendly slide dimensions.
```

## Manual Checks

- Open each artifact with `odl preview` (or via MCP `artifact_preview` when testing agent behavior).
- Confirm agents call `artifact_preview` rather than launching a system browser.
- Resize to mobile width.
- Check keyboard focus on links/buttons.
- Check no remote scripts/styles are required.
- Check handoff references correct files.

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-09 | 明确 Agent 预览应走 MCP `artifact_preview`，不自行打开系统浏览器。 |
