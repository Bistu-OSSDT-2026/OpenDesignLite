# 团队分工

目标：5 人推进 M1/M2。核心链路集中、边界清楚、每人有可验证交付。

## 角色

### Product & Kernel Lead

- PRD、架构边界、roadmap
- `manifest.json` / `handoff.md` / 主文件规则 → [specs/artifact-workspace.md](../specs/artifact-workspace.md)
- `od-core` 数据结构与 run lifecycle
- 轻量 design kernel 边界：token、primitive、recipe、pattern
- CLI/MCP 统一语义
- 第一版生成链路（M3）与最终集成

**M1 交付**：artifact API、`init/new/preview/handoff` 协议、端到端 HTML demo

### Preview

- 原生预览窗口（Rust + system WebView），由 `odl preview` 与 MCP `artifact_preview` 共用拉起
- HTML/Slides WebView、Markdown 方案、文件监视、错误页

**M1 交付**：`odl preview` 打开窗口，改文件自动刷新

### Skills & Templates

- `skills/html-page`、`docs-polish`、`slides-html`
- `templates/`  starter
- 静态 `--od-*` token、visual brief、pattern recipe 的模板落地
- smoke prompts（M3 前）

**M1 交付**：三份 SKILL v1、starter 模板 → [specs/built-in-skills.md](../specs/built-in-skills.md)

### Export & Packaging

- ZIP、自包含 HTML、MD；PDF 调研可后置

**M4 交付**：`odl export` → [specs/export.md](../specs/export.md)  
**M1 可选**：最小 ZIP 若时间允许

### MCP Bridge

- `od-mcp`：stdio transport、tool schema、转发到 `od-core`
- 面向 opencode / Codex / Claude Code 的 MCP 配置说明与交接模板

**M2 交付**：`crates/od-mcp` 可被 MCP 客户端连接，`artifact_create` / `artifact_preview` 走通闭环

## 工作流

### M1：本地产物闭环

| 标准 | Owner |
|------|-------|
| 创建 artifact、主文件、handoff | Kernel Lead |
| 本地预览 + 刷新 | Preview |
| starter、design kernel 默认样式与 skill 文案 | Skills |
| 接口契约文档 | Kernel Lead + 各 spec owner |

### M2：Agent 桥接

| 标准 | Owner |
|------|-------|
| handoff 可被外部 Agent 理解 | Kernel Lead |
| MCP 草案 | Kernel Lead |
| MCP 桥接走通闭环 | MCP Bridge |

## 周节奏

- **周初**：每人一个可合并小目标
- **周中**：只同步阻塞与接口变更
- **周末**：端到端 demo，重链路不重功能数

## 接口变更

变更前同步并更新 spec — 见 [architecture/boundaries.md](../architecture/boundaries.md#集成契约变更需同步)。

## 近期任务板

| 任务 | 角色 | 产出 |
|------|------|------|
| Artifact workspace spec | Kernel Lead | [artifact-workspace.md](../specs/artifact-workspace.md) |
| Preview spike | Preview | [preview.md](../specs/preview.md) + 可运行窗口 |
| Built-in skill v1 | Skills | 三个 `SKILL.md` |
| Lightweight design starter | Skills + Kernel Lead | `templates/` 静态 CSS 与 visual brief 草案 |
| Handoff 格式 | Kernel Lead | [handoff.md](../specs/handoff.md) |
| CLI 规范 | Kernel Lead | [cli.md](../specs/cli.md) |
| MCP 桥接 spike | MCP Bridge | `crates/od-mcp` stdio + tool schema |
| Smoke prompts | Skills | [smoke-prompts.md](../specs/smoke-prompts.md) |
| Release checklist | Kernel Lead | [release-checklist.md](../specs/release-checklist.md) |
