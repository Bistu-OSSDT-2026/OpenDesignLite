# ADR 0002：轻量 Design Kernel

**状态**：已采纳  
**日期**：2026-07-01  
**替代**：内置 shadcn/ui、Tailwind、Radix Themes、Shoelace、Material Web 或自带前端应用运行时

## 背景

产品需要让 HTML、文档、幻灯片在本地预览时默认更好看，但 `od-core` 的首要约束仍是轻量、本地优先、框架无关。若直接内置现成 UI kit，会引入 React、Tailwind、Web Components、Lit 或 Node 工具链，和二进制壳层优先的路线冲突。

调研见 [design-system-kernel.md](../research/design-system-kernel.md)。

## 决策

`od-core` 采用最轻且不耦合的设计内核路线：**CSS variables token + layout primitive + component recipe + pattern guideline**。

内核表达设计语言，不表达具体前端框架组件。

```text
design kernel
  tokens       # primitive + semantic CSS variables / JSON tokens
  primitives   # Stack / Inline / Grid / Container / Section / Split
  recipes      # Button / Input / Card / Tabs / Table / EmptyState 等样式约定
  patterns     # artifact workspace / dashboard / landing / settings 等生成约束
```

第一版允许随 artifact 输出静态 CSS 与设计说明，不引入 JS 组件库或前端构建链。

## 边界

**允许进入 core**

- 自研 `--od-*` CSS variables。
- 可序列化的 token schema。
- 框架无关的 layout primitive 名称与语义。
- 框架无关的 component recipe 与 visual brief。
- 可选的静态 CSS starter，用于模板默认观感。

**不允许进入 core**

- React / Vue / Svelte 组件实现。
- Tailwind / UnoCSS 作为唯一样式表达。
- Radix / Headless UI / Ariakit 交互 runtime。
- Shoelace / Material Web / Lit Web Components runtime。
- 需要 dev server 才能预览的设计系统。

**可做 adapter**

- shadcn/ui 输出目标。
- Tailwind / UnoCSS 输出目标。
- React / Web Components 输出目标。
- Style Dictionary / Figma tokens 导入导出。

## 后果

**正面**

- core 保持极轻，无前端 runtime 绑定。
- 产物仍是普通文件，可离线预览和交接。
- AI 生成有统一视觉约束，不必依赖某个组件库。
- 后续可以按目标栈生成 shadcn/ui、Tailwind、Web Components，而不污染内核。

**负面**

- 第一版需要自己维护默认 token 与 recipe 质量。
- 复杂交互组件不会自动获得 Radix/Headless UI 的完整行为。
- 视觉质量依赖 templates、skills、prompts 是否正确使用 design kernel。

## 备注

- Open Props、Radix Colors、Pico CSS 只能作为参考或精选来源，不作为强绑定 runtime。
- shadcn/ui 值得借鉴 registry、recipe、默认比例，但不进入 core runtime。
- 若未来要引入前端框架级 design system，必须另开 ADR。
