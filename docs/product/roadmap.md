# 路线图

每个里程碑有明确**可演示**的闭环，不堆功能。

## M0：仓库契约（已完成）

- [x] PRD、架构文档
- [x] Rust workspace 脚手架（`od-core` / `od-cli` / `od-mcp` / `od-preview`）
- [x] 内置技能目录与 starter `templates/`
- [x] 最小 CLI：`init`、`new`、`preview`
- [x] Design kernel 路线调研与 ADR：极轻、框架无关、不内置 UI runtime

**出口**：文档与代码目录结构稳定，specs 索引就绪。

## M1：本地产物闭环（基本落地，收尾中）

- [x] `odl init` / `odl new html|docs|slides`
- [x] 原生预览窗口打开产物目录
- [x] 文件监视 → 自动刷新预览
- [x] `handoff.md` 生成与刷新
- [x] `odl skill` / `odl skill show <name>`
- [x] HTML / Slides starter 使用轻量 design token 与静态 CSS recipe
- [ ] 错误页与发布前 smoke 验证继续收尾

**出口**：创建 HTML artifact → 本地窗口预览 → 改文件即刷新 → 有 handoff。这是 MCP（M2）的前置基础。

依赖 spec：[artifact-workspace](../specs/artifact-workspace.md)、[preview](../specs/preview.md)、[handoff](../specs/handoff.md)

## M2：Agent 桥接（MCP，产品主交付）（基本落地，收尾中）

这是产品的核心里程碑：把 M1 的本地产物+预览闭环通过 MCP 暴露给编码 Agent。

- [x] tool 名、DTO 与 JSON Schema：create / preview / export / handoff
- [x] tool-side `run()`：create / preview / handoff / export
- [x] stdio MCP server：`odl mcp` → `od_mcp::serve_stdio()`，`tools/list` / `tools/call` 已接 create/preview/handoff/export
- [ ] 面向 opencode、Codex、Claude Code、Cursor、Zed 的 MCP 配置说明与真实客户端联调
- [ ] Agent 端到端验收：一句话生成 → 预览自动弹出 → 对话微调 → 预览实时刷新

**出口**：在 Agent 里配好 MCP → 一句话生成产物 → 预览自动弹出 → 继续对话微调 → 预览实时刷新。

依赖 spec：[mcp](../specs/mcp.md)、[cli](../specs/cli.md)

## M3：导出（基本落地）

- [x] 自包含 HTML 目录导出
- [x] Markdown 原样（docs → `doc.md`）
- [x] ZIP 产物包（`/` 路径；排除 `.git` / `.log` / `.odl`）
- [x] PDF：本机 Chrome/Edge headless print（无则 `pdf_backend_missing`）
- [ ] HTML 单文件内联（`--single-file`）后续

**出口**：`odl export` / `artifact_export` 覆盖常用格式。

依赖 spec：[export](../specs/export.md)

> **不在 v1 范围**：内置模型调用 / BYOK 生成链路已放弃，产品只编排外部 Agent（见 [ADR 0003](../decisions/0003-no-built-in-model-calls.md)）。
