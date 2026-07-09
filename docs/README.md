# Open Design Lite 文档

通过 MCP 接入编码 Agent 的轻量、本地优先设计助手。Agent 一句话驱动，预览自动弹出并实时刷新；命令名 `odl` 的 CLI 是脚本化辅助。

## 阅读顺序

| 文档 | 用途 |
|------|------|
| [product/prd.md](product/prd.md) | 做什么、不做什么、MVP 模式 |
| [architecture/overview.md](architecture/overview.md) | 系统分层、模块边界、数据模型 |
| [product/roadmap.md](product/roadmap.md) | 里程碑 M0–M3 |
| [decisions/](decisions/) | 已采纳的架构决策（ADR） |
| [specs/](specs/) | 实现规范与接口契约 |
| [team/plan.md](team/plan.md) | 5 人分工与接口变更规则 |
| [research/kernel-candidates.md](research/kernel-candidates.md) | 历史调研结论（已否决 fork 路线） |
| [research/design-system-kernel.md](research/design-system-kernel.md) | 轻量设计内核调研 |

## 仓库地图

```text
crates/
  od-core/        产物、工作区、技能、运行原语
  od-mcp/         MCP 工具面（stdio server + create/preview/handoff/export 已接入）
  od-cli/         `odl` 命令入口（含 `odl mcp`；脚本化辅助）
  od-preview/     本地 WebView 预览、Markdown 渲染、文件监听
docs/             本文档树
skills/           内置技能（SKILL.md）
templates/        启动模板
```

## 当前状态

- **定位已定**：MCP 优先 / Agent 驱动，无独立壳层 app（见 [ADR 0001](decisions/0001-binary-shell-first.md)）
- **M1 闭环基本可跑**：`init` / `new` / `preview` / `handoff` / `skill` 已实现
- **M2 stdio 已可启动**：`odl mcp` 提供 JSON-RPC stdio server；create/preview/handoff/export 已接 `tools/call`
- **M3 导出已接入**：`odl export` / `artifact_export` 支持 html / md / zip / pdf（本机浏览器）
- **Design kernel 路线已定**：极轻、框架无关，使用 CSS variables + recipe，不内置 UI runtime
- **下一优先级**：补齐各 Agent 的 MCP 配置说明与真实客户端联调；M1 错误页/smoke 收尾

## 文档语言

产品与技术文档以中文为主。对外 README 保留简短英文摘要。
