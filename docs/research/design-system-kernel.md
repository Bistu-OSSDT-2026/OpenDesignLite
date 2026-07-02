# 轻量 Design System 内核调研

**日期**：2026-07-01  
**结论**：`od-core` 不应内置 React、Tailwind、Radix、shadcn/ui 或 Web Components 运行时。更合适的路线是内置一个极轻的、框架无关的 **design token + layout primitive + component recipe + pattern guideline** 内核，输出 plain CSS / JSON / HTML 约定，再由可选 adapter 生成 Tailwind、shadcn/ui、React、Web Components 等目标代码。

## 判断标准

1. **必须轻**：core 不引入前端 runtime，不要求 Node dev server，不绑定特定框架。
2. **必须好看**：默认 token、排版、层级、阴影、圆角、留白要明显优于裸 HTML 和普通 classless CSS。
3. **必须适合 AI 生成**：不仅有组件，还要有 layout primitive、pattern recipe 和反模板化规则。
4. **必须 local-first**：静态文件可读、可拷贝、可预览、可交接。

## 推荐方案

采用四层内核：

```text
od-core design kernel
  tokens       -> primitive + semantic CSS variables / JSON tokens
  primitives   -> Stack, Inline, Grid, Container, Section, Split, Sidebar
  recipes      -> Button, Input, Card, Dialog shell, Tabs, Table, EmptyState
  patterns     -> artifact workspace, dashboard, landing, settings, gallery
```

第一版只需要随 artifact 写出一份静态 CSS 和一份设计说明，不需要任何 JS 组件库。

## 候选对比

| 候选 | 轻量程度 | 视觉质量 | 框架耦合 | 结论 |
|------|----------|----------|----------|------|
| Open Props | 很轻，CSS variables，可拆子集 | token 丰富，适合做底座 | 无 | 推荐借鉴/精选内置 |
| 自研 CSS variables | 最轻，无 runtime | 取决于默认值 | 无 | 必须作为 core API |
| Pico CSS | 轻，纯 CSS | 文档/MVP 感强，高级感有限 | 无 | 可借鉴 reset/form，不宜全量绑定 |
| MVP.css / Simple.css | 极轻 | 普通，适合 fallback | 无 | 可作为 fallback 参考 |
| Radix Colors | 轻，颜色体系优秀 | 很强，暗色/对比度成熟 | 无组件耦合 | 推荐借鉴色阶语义 |
| Style Dictionary | 构建期工具 | 不负责视觉 | 无 runtime | 适合 token 导入/导出，不进 runtime |
| Lightning CSS | 本地编译工具 | 不负责视觉 | 无 runtime | 适合作 CSS 编译/压缩工具 |
| Tailwind CSS | 产物可小，工具链较重 | 强，生态强 | utility/class 体系绑定 | 不进 core，可做 adapter |
| UnoCSS | 比 Tailwind 更轻 | 强，依赖配置 | atomic 体系绑定 | 可选 adapter，不做 core 基础 |
| shadcn/ui | 源码分发，视觉默认值优秀 | 很强 | React + Tailwind + Radix | 学习 registry/recipe，不内置 |
| Radix Themes | 中等 | 稳定好看 | React | 不适合 core，只做 React adapter |
| Ariakit | 中等 | 无样式，行为强 | React | 不适合 core，可做交互参考 |
| Headless UI | 中等 | 无样式 | React/Vue + Tailwind 生态 | 不适合 core |
| Shoelace / Web Awesome | 中等偏重 | 组件完整 | Web Components/Lit | 不适合最小 core，可选 adapter |
| Material Web | 中等偏重 | Material 质量高但风格强 | Web Components/Lit | 不适合 core |
| BeerCSS / daisyUI | 轻到中等 | 出活快但风格强 | Material 或 Tailwind 绑定 | 不适合 core 基础 |

## 为什么不能直接塞 shadcn/ui

shadcn/ui 的优势是“好看的源码分发”和 registry 生态，不是轻 runtime。它默认依赖 React、Tailwind、Radix primitives，会把 `od-core` 从本地文件内核拉向前端框架内核，违反当前边界。

可借鉴的部分：

- registry / recipe 分发方式。
- variant 命名，例如 `size`、`variant`、`tone`。
- 好看的默认 token 和组件比例。
- AI 友好的组件源码模式。

不应复制的部分：

- React 组件 runtime。
- Tailwind class 作为唯一表达。
- Radix 交互依赖进入 core。

## 接近 Claude Design 的关键

Claude Artifacts 的视觉质量通常来自设计约束，而不是重组件库：

- 暖中性色画布，不用纯白纯黑。
- 内容优先，低对比背景层、细边框、轻阴影。
- 排版层级强，正文可读，标题有编辑感。
- 留白充足，卡片不过度堆叠。
- 圆角中等，阴影克制。
- 组件安静，焦点在内容结构。
- 根据题材切换 visual brief，避免统一 SaaS 模板。

所以 core 需要的是“生成高质量 UI 的最小语法”，而不是一个完整 UI kit。

## 建议内置内容

### Token

- `color`: neutral、warm、accent、success、warning、danger。
- `type`: sans、serif、mono、字号、行高、字重、字距。
- `space`: 4px 基准的紧凑 scale。
- `radius`: sm、md、lg、xl、2xl、full。
- `shadow`: xs、sm、md、glow、inset。
- `motion`: fast、base、slow、standard ease、reduced motion。
- `z`: dropdown、overlay、modal、toast。

### Semantic Token

- `--od-bg-canvas`
- `--od-bg-surface`
- `--od-bg-elevated`
- `--od-text-primary`
- `--od-text-secondary`
- `--od-border-subtle`
- `--od-accent-solid`
- `--od-accent-soft`
- `--od-focus-ring`

### Layout Primitive

- Stack
- Inline
- Cluster
- Grid
- Container
- Section
- Split
- Sidebar
- Frame

### Component Recipe

- Button
- Input
- Textarea
- Select
- Checkbox
- Card
- Badge
- Tabs
- Dialog shell
- Popover shell
- Table
- EmptyState
- Toast shell

### Pattern Recipe

- artifact workspace
- command workspace
- document editor
- dashboard
- settings panel
- gallery board
- landing page
- onboarding
- comparison

## 第一版 MVP

建议第一版只做：

1. 一份 `od-design.css`，纯 CSS variables + reset + 基础 recipe。
2. 一份 `design-tokens.json`，便于未来导出到 Style Dictionary / Figma tokens。
3. 5 个 visual brief：`editorial`、`studio`、`workbench`、`instrument`、`gallery`。
4. 8 个 pattern recipe，供 skills 和 prompts 使用。
5. 不引入任何 Rust 之外的运行时依赖。

## 最终建议

`od-core` 应把 design system 定义成“可生成、可导出、可解释的设计语言”，不要定义成“组件库”。

推荐路径：

1. core 内置 token schema 和默认主题。
2. templates 使用静态 CSS recipe 提升默认观感。
3. skills 使用 visual brief + pattern recipe 约束生成。
4. adapters 后续再支持 shadcn/ui、Tailwind、UnoCSS、Web Components。

这条路线最轻，也最接近项目当前的本地二进制、静态产物、框架无关方向。
