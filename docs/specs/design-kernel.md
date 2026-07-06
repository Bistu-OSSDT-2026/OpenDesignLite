# Design Kernel

**状态**：草案  
**里程碑**：M1  
**实现位置**：`crates/od-core`、`templates/`、`skills/`

## 目的

定义最轻且不耦合的设计系统契约。Design kernel 只提供 `--od-*` token、静态 CSS recipe、layout primitive、pattern 与 visual brief，不引入 React、Tailwind、Radix、shadcn/ui、Lit 或 Web Components runtime。

## 范围

- 包含 token 命名规则。
- 包含 `od-design.css` 分层与 class 命名。
- 包含 visual brief 名称与用途。
- 包含模板如何引用 design kernel。
- 不包含具体组件框架实现。
- 不包含 Tailwind/shadcn adapter 细节。

## 默认资产

M1 默认内置两类资产：

```text
od-design.css          # 静态 CSS，写入 artifact assets/ 或 inline
design-tokens.json     # 可选，供后续导出与 adapter 使用
```

M1 实现可先只写出 `assets/od-design.css`。`design-tokens.json` 可以后置，但 token 命名必须按本文约束。

## CSS 变量

所有对外变量必须使用 `--od-*` 前缀。

最小语义 token：

```css
:root {
  --od-bg-canvas: #f7f3ed;
  --od-bg-surface: #fffaf3;
  --od-bg-elevated: #ffffff;
  --od-text-primary: #171412;
  --od-text-secondary: #5f5750;
  --od-text-muted: #8a8178;
  --od-border-subtle: rgba(28, 22, 17, 0.1);
  --od-border-default: rgba(28, 22, 17, 0.18);
  --od-accent-solid: #8f5b3f;
  --od-accent-soft: #f0dfd2;
  --od-accent-text: #6f432d;
  --od-focus-ring: rgba(143, 91, 63, 0.38);
}
```

最小 scale token：

| 类别 | 变量 |
|------|------|
| space | `--od-space-1` 到 `--od-space-12` |
| radius | `--od-radius-sm`、`md`、`lg`、`xl`、`2xl`、`full` |
| shadow | `--od-shadow-xs`、`sm`、`md`、`glow` |
| font | `--od-font-sans`、`serif`、`mono` |
| text | `--od-text-xs`、`sm`、`base`、`lg`、`xl`、`2xl`、`4xl` |
| motion | `--od-duration-fast`、`base`、`slow`、`--od-ease-standard` |

## CSS 分层

`od-design.css` 必须按以下 layer 组织：

```css
@layer od.reset, od.tokens, od.base, od.primitives, od.recipes, od.patterns, od.utilities;
```

各层职责：

| Layer | 职责 |
|-------|------|
| `od.reset` | box sizing、媒体元素、表单继承、基础重置。 |
| `od.tokens` | `:root`、主题、暗色变量。 |
| `od.base` | `body`、标题、段落、链接、表格、代码、focus。 |
| `od.primitives` | layout primitive class。 |
| `od.recipes` | button、card、input、badge、table 等组件 recipe。 |
| `od.patterns` | artifact、doc、slide、dashboard、hero 等 pattern。 |
| `od.utilities` | 少量工具 class，例如 `od-sr-only`、`od-muted`。 |

## Layout Primitive

M1 class 命名：

| Primitive | Class | 用途 |
|-----------|-------|------|
| Container | `.od-container` | 页面宽度与边距。 |
| Stack | `.od-stack` | 垂直布局，使用 gap。 |
| Inline | `.od-inline` | 水平布局，可 wrap。 |
| Cluster | `.od-cluster` | 工具栏、标签组、按钮组。 |
| Grid | `.od-grid` | 响应式卡片网格。 |
| Section | `.od-section` | 页面区块 padding。 |
| Split | `.od-split` | 两栏布局。 |
| Frame | `.od-frame` | 固定比例媒体或预览。 |

Primitive 必须只表达结构，不能包含强品牌色或具体业务语义。

## Component Recipe

M1 最小 recipe：

| Recipe | Class | 变体 |
|--------|-------|------|
| Button | `.od-button` | `[data-variant="primary|secondary|ghost"]` |
| Card | `.od-card` | `[data-tone="plain|warm|accent"]` |
| Input | `.od-input` | invalid、disabled、focus-visible |
| Badge | `.od-badge` | `[data-tone="neutral|accent|success|warning|danger"]` |
| Table | `.od-table` | 默认文档表格。 |
| Empty state | `.od-empty` | 图形、标题、说明、动作。 |

Recipe 必须使用 semantic token，不得直接依赖外部库 token 名称。

## Pattern

M1 pattern class：

| Pattern | Class | 用途 |
|---------|-------|------|
| Artifact | `.od-artifact` | HTML artifact 默认页面框架。 |
| Document | `.od-doc` | Markdown 预览包装。 |
| Slides | `.od-slide` | 单页 slide 样式基线。 |
| Hero | `.od-hero` | 页面首屏或 intro。 |
| Dashboard | `.od-dashboard` | 指标/卡片网格。 |

Pattern 是生成约束，不是固定模板。skills 可以组合 pattern 与 primitive。

## Visual Brief

M1 支持三种 brief：

| Brief | 用途 | 视觉方向 |
|-------|------|----------|
| `editorial` | 文档、landing、叙事页面 | 暖纸色、强排版、克制边框。 |
| `studio` | 作品集、设计稿、展示页面 | 大留白、图像优先、安静 surface。 |
| `workbench` | 工具、dashboard、artifact workspace | 清晰层级、紧凑控件、低对比背景。 |

后续可增加 `instrument`、`gallery`，但 M1 skills 不得引用未定义 brief。

## 模板引用规则

HTML 与 slides starter 应优先使用：

```html
<link rel="stylesheet" href="assets/od-design.css" />
```

允许 inline 模式：

```html
<style data-od-design>
  /* embedded od-design.css */
</style>
```

规则：

- 默认写出外部 CSS，便于人类和 Agent 编辑。
- 单文件导出或用户明确要求时可 inline。
- 不要求 Node、PostCSS、Tailwind 或 Sass 构建。
- 不允许模板引入 CDN UI kit 作为默认视觉基础。

## Token JSON 草案

`design-tokens.json` 使用 DTCG-inspired 子集：

```json
{
  "schemaVersion": 1,
  "color": {
    "bg": {
      "canvas": {
        "$type": "color",
        "$value": "#f7f3ed",
        "$description": "Default warm page canvas"
      }
    }
  }
}
```

M1 不要求实现复杂 alias、`$extends`、token transform。后续 adapter 可以导出 Style Dictionary、Tokens Studio、Tailwind config。

## 可访问性规则

- 所有可交互元素必须有可见 `:focus-visible`。
- 默认正文对比度应达到 WCAG AA。
- 交互目标建议不小于 40px。
- `prefers-reduced-motion` 必须禁用非必要动画。
- 不使用颜色作为唯一状态表达。

## 测试

- HTML starter 引用 `assets/od-design.css` 后可离线打开。
- Markdown preview wrapper 使用 `.od-doc` 后排版可读。
- Slides 使用 `.od-slide` 后单页在 16:9 与窄屏下不溢出。
- focus ring 在按钮、链接、输入框上可见。
- 无 Node 或前端构建工具也能生成 artifact。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
