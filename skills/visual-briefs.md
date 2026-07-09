# Visual Briefs

内置技能共享的**视觉简报**。每个 brief 是一组可执行的视觉约束，供 skill 在生成 HTML / Slides 时套用，保证默认观感一致、克制、不落入 AI slop。

所有 brief 只在 design kernel 的 `--od-*` token、`.od-*` primitive / recipe / pattern 之上表达，**不另起一套 token 或颜色体系**。命名的单一来源是 `crates/od-core/src/design/catalog.rs`，完整契约见 [`docs/specs/design-kernel.md`](../docs/specs/design-kernel.md)。

M1 支持三种 brief：`editorial`、`studio`、`workbench`。skill front matter 的 `visualBrief` 字段只能取这三者之一。

## 通用约束（所有 brief）

- 只用 `--od-*` token 表达颜色、间距、圆角、阴影、字号；需要新色时新增 `--od-*` 变量，不写裸 hex。
- 颜色克制：三色 + 一个 accent 通常足够；七色几乎总是过多。
- 留白优先于分隔线与边框；用 `--od-space-*` 建立节奏。
- 禁止蓝紫渐变、霓虹光晕、无意义大阴影等 AI slop 套路。
- 每个可交互元素必须有可见 `:focus-visible`；正文对比度达 WCAG AA；尊重 `prefers-reduced-motion`。
- 产物必须离线可开：不依赖远程字体 / 图片 / CDN，图片用纯 CSS placeholder。

---

## `editorial`

**用途**：文档、landing、叙事型页面（`html-page`、`docs-polish` 默认）。

**定调**：暖纸感的杂志页，靠排版和留白说话，装饰极度克制——不是通用 SaaS 模板。

**可执行约束**：

- 背景走暖色纸感：`--od-bg-canvas` / `--od-bg-surface`，正文 `--od-text-primary`，次要信息 `--od-text-secondary` / `--od-text-muted`。
- 正文用 serif 或高可读 sans（`--od-font-serif` / `--od-font-sans`），行宽受 `.od-container` measure 约束，不铺满整屏。
- 层级靠字号（`--od-text-*`）和间距（`--od-space-*`）建立，而不是靠边框和底色块；边框只用 `--od-border-subtle`。
- accent 仅点缀（链接、单个 CTA、强调词），用 `--od-accent-solid` / `--od-accent-text`；最多一个主 CTA。
- 结构用 `.od-container`、`.od-stack`、`.od-section`；正文页面套 `.od-artifact`，Markdown 预览套 `.od-doc`。

---

## `studio`

**用途**：作品集、设计稿展示、幻灯片（`slides-html` 默认）。

**定调**：画廊墙 / 设计评图板——每一页是单个想法的安静舞台，留白本身就是设计，不是空隙。

**可执行约束**：

- 大留白、图像/内容优先，surface 安静：背景多用 `--od-bg-surface` / `--od-bg-elevated`，避免高饱和底色。
- 一屏一个焦点：slide 用 `.od-slide`，内容居中于 `.od-slide__inner`，不让单页溢出 16:9 —— 宁可拆页或删减。
- 标题稀疏有力，正文短；用 `--od-text-2xl` / `--od-text-4xl` 拉出标题层级，配大 `--od-space-*` 呼吸。
- 媒体位用 `.od-frame` 固定比例，纯 CSS placeholder 代替远程图片。
- accent 极简，只用于分区编号、进度或单个强调；过渡默认静态。

---

## `workbench`

**用途**：工具、dashboard、artifact workspace 型页面。

**定调**：清晰的信息层级、紧凑但不拥挤的控件、低对比背景，让数据和操作是主角。

**可执行约束**：

- 低对比中性背景（`--od-bg-canvas` / `--od-bg-surface`），用 `--od-border-default` 分区，密度高于 editorial。
- 用 `.od-grid` / `.od-cluster` / `.od-split` 组织卡片与控件，pattern 套 `.od-dashboard`；卡片用 `.od-card`。
- 控件走 recipe：`.od-button`（`data-variant`）、`.od-input`、`.od-badge`（`data-tone`）、`.od-table`；空态用 `.od-empty`。
- accent 用于状态与主操作，语义色（success/warning/danger）经 `.od-badge` 的 `data-tone` 表达，不用颜色作为唯一状态信号。
- 交互目标不小于 40px，focus ring 清晰可见。

---

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-08 | 初版：补齐 `html-page` / `slides-html` SKILL.md 引用的共享视觉简报，覆盖 editorial / studio / workbench。 |
