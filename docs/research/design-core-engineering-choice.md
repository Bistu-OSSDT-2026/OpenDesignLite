# Design Core 工程抉择调研

**日期**：2026-07-07  
**结论**：继续采用轻量 design kernel，但必须把“好看”工程化为默认静态资产、visual brief、pattern recipe、skill 约束和验收清单。`od-core` 只拥有设计语义契约；`od-design.css`、templates、skills 负责把契约落成可预览、可交接、默认好看的文件产物。

## 问题

目标同时有两个：

1. Design system 要轻：不引入前端 runtime、Node dev server、框架组件库或远程 UI kit。
2. 结果要好看：默认 HTML、docs、slides 不能像裸 HTML，也不能是通用 SaaS 套壳。

这两个目标的冲突点在于：重 UI kit 通常能更快保证视觉质量，但会污染 `od-core` 的本地优先、框架无关边界；纯 classless CSS 足够轻，但很难稳定产出高级感。

## 决策

采用“轻内核 + 强默认资产 + 生成约束”的路线：

```text
od-core
  design semantics: kernel version, --od-* token contract, visual brief, manifest.design

shared assets
  od-design.css: reset, tokens, base, primitives, recipes, patterns

templates / skills
  use assets/od-design.css, od-* classes, visual brief, pattern recipe

adapters
  optional Tailwind / shadcn / React / Web Components export targets, not core runtime
```

核心判断：**好看不是靠把 shadcn/ui、Tailwind 或 React 塞进 core，而是靠让静态 CSS、模板和 skills 默认使用同一套设计语言。**

## Core 边界

允许进入 `od-core`：

- `KERNEL_VERSION`、`STYLESHEET_ASSET`、`TOKEN_PREFIX` 等稳定常量。
- `VisualBrief`：M1 只支持 `editorial`、`studio`、`workbench`。
- `manifest.design`：`kernelVersion`、`stylesheet`、`visualBrief`。
- token schema、recipe 名称、pattern 名称等框架无关语义。
- artifact、skill、handoff 对 design kernel 的引用规则。

不允许进入 `od-core`：

- React / Vue / Svelte 组件实现。
- Tailwind / UnoCSS 作为唯一表达。
- shadcn/ui、Radix、Headless UI、Ariakit runtime。
- Shoelace、Material Web、Lit 或其他 Web Components runtime。
- 需要 dev server 才能预览的设计系统。
- 由 preview、shell、extension 反向决定的视觉规则。

可作为后续 adapter：

- Tailwind config / UnoCSS preset。
- shadcn/ui registry 或源码生成。
- React / Web Components 输出目标。
- Style Dictionary、Tokens Studio、Figma tokens 导入导出。

## 为什么这样既轻又能好看

### 轻量来源

- Artifact 仍是普通文件：HTML、Markdown、CSS、manifest、handoff。
- CSS 是静态资产，不需要构建链。
- `od-core` 不依赖浏览器 UI runtime。
- preview、MCP、shell 都读取同一份 manifest 和设计语义，不复制设计逻辑。

### 视觉质量来源

- 一套非裸 HTML 的 warm neutral 默认 token：canvas、surface、text、border、accent、shadow、radius。
- Layout primitive 约束结构：`.od-container`、`.od-stack`、`.od-inline`、`.od-grid`、`.od-section`、`.od-split`、`.od-frame`。
- Component recipe 提供产品质感：`.od-button`、`.od-card`、`.od-input`、`.od-badge`、`.od-table`、`.od-empty`。
- Pattern recipe 提供页面组织：`.od-artifact`、`.od-doc`、`.od-slide`、`.od-hero`、`.od-dashboard`。
- Visual brief 让同一内核能切换题材气质，避免所有产物都像同一个 SaaS 模板。
- Skills 明确禁止 AI slop：不默认蓝紫渐变、玻璃拟态、随机图标、远程字体、CDN UI kit、每块区域自创颜色。

## M1 必须落地的最小集

1. 提供一份公共 `od-design.css`，默认写入 artifact 的 `assets/od-design.css`。
2. HTML、docs、slides starter 默认引用 `assets/od-design.css`。
3. Starter 至少使用一个 pattern recipe，不能只有标题和段落。
4. `SKILL.md` 声明默认 `visualBrief`、推荐 pattern、禁止项和模板路径。
5. `manifest.design` 写入 `kernelVersion`、`stylesheet`、`visualBrief`。
6. `handoff.md` 记录 design kernel 约束，要求后续 agent 保留 `--od-*` 与 `od-*` recipe。
7. Markdown preview wrapper 使用同一 stylesheet 和 `.od-doc`。

M1 不做：

- 不做完整 design token transform。
- 不做 Tailwind/shadcn adapter。
- 不做复杂交互组件行为。
- 不要求 `design-tokens.json` 必须生成，可后置。

## 当前仓库差距

- 已有 ADR 0002 采纳轻量 design kernel 路线。
- 已有 `docs/specs/design-kernel.md` 定义 token、layer、primitive、recipe、pattern、brief。
- `crates/od-core/src/design` 已有 kernel version、stylesheet path、token prefix、brief 等语义入口。
- ✅ `crates/od-core/src/design` 已补齐 framework-agnostic 语义契约：`catalog`（primitive/recipe/pattern class 名 + 校验）、`brief`（视觉方向 + 推荐 pattern）、`guardrails`（反 AI slop 禁止项 + `references_stylesheet` / `uses_design_language` 质量标记）。handoff 已改为消费该契约。
- 当前尚未发现实际公共 `od-design.css`（`catalog` class 名已就绪，等待 CSS 落地）。
- 顶层 `templates/html-page/basic.html` 与 `templates/slides/basic.html` 仍偏内联裸 CSS，没有默认引用 `assets/od-design.css`。
- 部分 `skills/*.md` 尚未充分声明 template、visual brief、pattern recipe 和反 AI slop 规则。
- `od-cli new` 仍有 fallback CSS/HTML 常量；长期应迁移到统一资产和 skill template。

## 工程顺序

1. 先定 `od-design.css` 公共资产来源，避免 CLI、preview、shell 各维护一份 CSS。
2. 接入 skills/templates：让内置 skill 能稳定找到模板，并默认引用 design asset。
3. 补齐 `odl new` 生成闭环：主文件、manifest、assets、handoff 一起生成。
4. preview 优先读 manifest，再 fallback 到主文件检测。
5. Markdown preview 使用同一 design wrapper。
6. 最后再做 WebView/watch 等体验增强，不要先引入重依赖。

## 质量门槛

自动检查：

- 产物引用 `assets/od-design.css` 或内联 `<style data-od-design>`。
- 使用 `--od-*` token 或 `od-*` primitive/recipe。
- 不引入 React、Tailwind、shadcn、CDN UI kit、远程字体、远程脚本作为默认路径。
- 可离线打开，可由 `odl preview` 预览。
- 窄屏不明显溢出，focus ring 可见，正文对比度达到 WCAG AA。

人工检查：

- 第一屏明显优于裸 HTML。
- 信息层级清晰：标题、说明、分组、行动或下一步明确。
- 留白、边框、阴影克制，不靠装饰堆叠撑视觉。
- 风格符合 brief，不是统一蓝紫 SaaS 套壳。
- 内容可读、可手改、可交接。

## 反模式

- 为了“好看”把 shadcn/ui、Tailwind、Radix 或 React 放进 `od-core`。
- 每个模板各写一套颜色、间距、圆角和阴影。
- preview 或 shell 自己推断 design system，而不是读 manifest/design kernel。
- 默认依赖 CDN 字体、远程图片、远程脚本或 dev server。
- 把 pattern 变成固定页面模板库，导致所有产物长得一样。

## 最终建议

工程抉择不是在“轻”和“好看”之间二选一，而是拆分职责：

- `od-core` 保持轻，只管设计语言契约。
- `od-design.css` 承担默认视觉质量。
- templates 和 skills 承担生成结构与题材风格。
- handoff 和 smoke checks 承担质量守门。
- adapters 承担未来框架生态接入。

这条路线符合当前本地二进制、静态资产、薄插件、plain file handoff 的产品原则，也给后续 Tailwind/shadcn/React 输出留出了扩展口。
