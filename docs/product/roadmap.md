# 路线图

每个里程碑有明确**可演示**的闭环，不堆功能。

## M0：仓库契约 ← 当前

- [x] PRD、架构文档
- [x] Rust workspace 脚手架（`od-core` / `od-cli` / `od-mcp` / `od-preview`）
- [x] 内置技能占位、`templates/`
- [x] 最小 CLI：`init`、`new`、`preview`（预览未实现）
- [x] Design kernel 路线调研与 ADR：极轻、框架无关、不内置 UI runtime

**出口**：文档与代码目录结构稳定，specs 索引就绪。

## M1：本地产物闭环

- `odl init` / `odl new html|docs|slides`
- 原生预览窗口打开产物目录
- 文件监视 → 自动刷新预览
- `handoff.md` 生成规范落地
- HTML / Slides starter 使用轻量 design token 与静态 CSS recipe

**出口**：创建 HTML artifact → 本地窗口预览 → 改文件即刷新 → 有 handoff。这是 MCP（M2）的前置基础。

依赖 spec：[artifact-workspace](../specs/README.md)、[preview](../specs/README.md)、[handoff](../specs/README.md)

## M2：Agent 桥接（MCP，产品主交付）

这是产品的核心里程碑：把 M1 的本地产物+预览闭环通过 MCP 暴露给编码 Agent。

- MCP（stdio）暴露 create / preview / export / handoff
- `artifact_create` 生成产物；`artifact_preview` 自动弹出常驻预览窗口
- Agent 迭代改文件 → 预览窗口实时刷新（复用 M1 文件监视）
- 面向 opencode、Codex、Claude Code、Cursor、Zed 的交接模板与配置说明

**出口**：在 Agent 里配好 MCP → 一句话生成产物 → 预览自动弹出 → 继续对话微调 → 预览实时刷新。

依赖 spec：[mcp](../specs/README.md)、[cli](../specs/README.md)

## M3：内置生成

- BYOK OpenAI 兼容提供商
- 直接生成 HTML / 文档 / 幻灯片
- 模式专用提示词与质量检查
- visual brief 与 pattern recipe 进入生成提示词

**出口**：无外部 Agent 也能从提示词到可预览产物。

依赖 spec：[built-in-skills](../specs/README.md)、[smoke-prompts](../specs/README.md)

## M4：导出

- 自包含 HTML
- Markdown 原样
- PDF（从预览打印）
- ZIP 产物包

**出口**：`odl export` 覆盖常用格式。

依赖 spec：[export](../specs/README.md)
