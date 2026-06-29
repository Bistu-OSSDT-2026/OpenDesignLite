# Team Plan

目标：5 人小队推进 Open Design Lite 的 M1/M2。分工原则是核心链路集中、模块边界清楚、每个人都能交付可验证成果。

## Roles

### Product & Kernel Lead

负责产品判断、核心 kernel、agent bridge 的主线集成。

主要范围：

- 维护 PRD、架构边界和 roadmap。
- 定义 artifact workspace 格式：`manifest.json`、`handoff.md`、primary file 规则。
- 推进 `od-core` 的核心数据结构和 run lifecycle。
- 设计 CLI/MCP 的统一命令语义，确保 shell、agent、插件都走同一套内核。
- 负责第一版 AI 生成链路：HTML / Docs / Slides 的 prompt composition、provider 接入、handoff prompt。
- 做最终集成和 release checklist。

第一阶段交付：

- `od-core` artifact/workspace API。
- `od-cli` 的 `init/new/preview/handoff` 命令协议。
- `docs/specs/artifact-workspace.md`。
- 第一条端到端 demo：创建 HTML artifact，预览，生成 handoff。

### Preview Shell

负责本地预览体验，让产物打开快、刷新快、错误清楚。

主要范围：

- 调研并实现最小 native shell 方案，优先 Rust + system WebView。
- HTML / Slides 预览窗口。
- Markdown 预览方案。
- 文件变化自动刷新。
- 预览错误展示：文件不存在、HTML 加载失败、资源路径错误。

第一阶段交付：

- `odl preview <artifact-dir>` 能打开本地窗口。
- 修改 `index.html` 或 `slides.html` 后自动刷新。
- 简单错误页。

### Skills & Templates

负责内置技能和 starter 模板，把“开箱即用”做扎实。

主要范围：

- 完善 `skills/html-page`、`skills/docs-polish`、`skills/slides-html`。
- 为每个 skill 写清楚输入、输出、质量标准和 handoff 要求。
- 维护 `templates/` 下的基础 HTML / Slides 模板。
- 准备 6-9 个 smoke prompts，用来验证生成质量。

第一阶段交付：

- 三个内置 skill 的 v1 文案。
- HTML page starter、docs starter、slides starter。
- `docs/specs/built-in-skills.md`。

### Export & Packaging

负责产物打包、导出和本地安装体验。

主要范围：

- Artifact ZIP 打包。
- Self-contained HTML 检查。
- Markdown 原样导出。
- 初步 PDF 方案调研，先不强行实现。
- 本地二进制发布脚本雏形。

第一阶段交付：

- `odl export <artifact-dir> --format zip`。
- `odl export <artifact-dir> --format html|md` 的最小实现。
- `docs/specs/export.md`。

### Editor Adapters

负责薄插件/编辑器接入验证，不做重逻辑。

主要范围：

- 验证 Cursor / Zed / VS Code / Codex 哪个最适合第一版薄集成。
- 设计“打开当前 artifact preview”和“发送当前文件到 odl”的最小命令。
- 不持有 artifact storage，不持有模型配置。
- 通过 CLI 或 MCP 与 kernel 通信。

第一阶段交付：

- `apps/extensions/adapter-plan.md`。
- 选定第一个 editor target。
- 一个最小 proof：从编辑器命令调用 `odl preview` 或 `odl handoff`。

## Workstreams

### M1: Local Artifact Loop

成功标准：

- 能创建 artifact folder。
- 能生成或放置 `index.html` / `doc.md` / `slides.html`。
- 能用本地 shell 快速预览。
- 能生成 `handoff.md`。
- 不需要 web dev server。

Owner：Product & Kernel Lead

Supporting：

- Preview Shell：预览窗口和刷新。
- Skills & Templates：starter artifacts。
- Export & Packaging：最小 ZIP/HTML/MD export。
- Editor Adapters：验证薄接入路径。

### M2: Agent Bridge

成功标准：

- 外部 agent 能通过 handoff 文件理解 artifact。
- MCP tool surface 有稳定草案。
- CLI commands 可被插件或 agent 调用。

Owner：Product & Kernel Lead

Supporting：

- Editor Adapters：选一个 editor 做 proof。
- Skills & Templates：agent-facing skill 文案。
- Export & Packaging：handoff bundle。

## Weekly Rhythm

- 周初：每人确认一个可合并的小目标。
- 周中：只看阻塞和接口变化。
- 周末：跑一次端到端 demo，不追求功能多，只追求链路顺。

## Interface Contracts

这些接口变更前需要同步：

- artifact folder layout
- skill front matter
- CLI command names and flags
- MCP tool names
- previewable file detection
- export formats

## Near-Term Task Board

| Task | Role | Output |
|---|---|---|
| Artifact workspace spec | Product & Kernel Lead | `docs/specs/artifact-workspace.md` |
| Real preview shell spike | Preview Shell | `odl preview` opens a window |
| Built-in skill v1 | Skills & Templates | three polished `SKILL.md` files |
| ZIP export | Export & Packaging | `odl export --format zip` |
| Editor adapter decision | Editor Adapters | `apps/extensions/adapter-plan.md` |
| Handoff prompt format | Product & Kernel Lead | `docs/specs/handoff.md` |
| Smoke prompt set | Skills & Templates | `docs/specs/smoke-prompts.md` |
| Release checklist | Product & Kernel Lead | `docs/specs/release-checklist.md` |
