# 团队分工

目标：5 人完成 M1 收尾并推进 M2。核心链路集中、边界清楚、每人有可验证交付。

## 角色

### Product & Kernel Lead

- PRD、架构边界、roadmap
- `manifest.json` / `handoff.md` / 主文件规则 → [specs/artifact-workspace.md](../specs/artifact-workspace.md)
- `od-core` 数据结构与 run lifecycle
- 轻量 design kernel 边界：token、primitive、recipe、pattern
- CLI/MCP 统一语义
- 跨 crate 最终集成与发布收尾

**M1 状态**：artifact API、`init/new/preview/handoff/skill` 已基本落地；继续收尾 smoke 与文档一致性。

### Preview

- 原生预览窗口（Rust + system WebView），由 `odl preview` 与 MCP `artifact_preview` 共用拉起
- HTML/Slides WebView、Markdown 方案、文件监视、错误页

**M1 状态**：`odl preview` 已打开窗口并监听刷新；继续完善错误页与平台兼容验证。

### Skills & Templates

- `skills/html-page`、`docs-polish`、`slides-html`
- `templates/`  starter
- 静态 `--od-*` token、visual brief、pattern recipe 的模板落地
- smoke prompts（M2）

**M1 状态**：三份 SKILL v1、starter 模板与 `odl skill show` 已接入；模板去重仍待迁移 → [specs/built-in-skills.md](../specs/built-in-skills.md)

### Export & Packaging

- ZIP、自包含 HTML、MD；PDF 调研可后置

**M3 交付**：`odl export` → [specs/export.md](../specs/export.md)  
**M1 可选**：最小 ZIP 若时间允许

### MCP Bridge

- `od-mcp`：stdio transport、tool schema、转发到 `od-core`
- 面向 opencode / Codex / Claude Code 的 MCP 配置说明与交接模板

**M2 状态**：`odl mcp` + 手写 JSON-RPC stdio 已接 create/preview/handoff；下一步补各 Agent 配置说明与真实客户端联调。

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
| stdio MCP server 可被客户端连接 | MCP Bridge |
| MCP 客户端配置说明与联调 | MCP Bridge + Kernel Lead |

## 周节奏

- **周初**：每人一个可合并小目标
- **周中**：只同步阻塞与接口变更
- **周末**：端到端 demo，重链路不重功能数

## 接口变更

变更前同步并更新 spec — 见 [architecture/boundaries.md](../architecture/boundaries.md#集成契约变更需同步)。

## 近期任务板

| 任务 | 角色 | 产出 |
|------|------|------|
| Artifact workspace 收尾 | Kernel Lead | [artifact-workspace.md](../specs/artifact-workspace.md) 与实际 manifest 一致 |
| Preview 错误页与平台验证 | Preview | [preview.md](../specs/preview.md) + smoke 记录 |
| Skill 模板去重 | Skills | 顶层 fallback 模板与 skill 模板迁移方案 |
| Handoff 行为收尾 | Kernel Lead | [handoff.md](../specs/handoff.md) 与 CLI/MCP 行为一致 |
| CLI smoke | Kernel Lead | [cli.md](../specs/cli.md) 命令全覆盖 |
| MCP 客户端配置与联调 | MCP Bridge | 各 Agent 配置说明 + 端到端验收 |
| Smoke prompts | Skills | [smoke-prompts.md](../specs/smoke-prompts.md) |
| Release checklist | Kernel Lead | [release-checklist.md](../specs/release-checklist.md) |
