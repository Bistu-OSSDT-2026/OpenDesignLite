# Open Design Lite 文档

通过 MCP 接入编码 Agent 的轻量、本地优先设计助手。Agent 一句话驱动，预览自动弹出并实时刷新；命令名 `odl` 的 CLI 是脚本化辅助。

## 阅读顺序

| 文档 | 用途 |
|------|------|
| [product/prd.md](product/prd.md) | 做什么、不做什么、MVP 模式 |
| [architecture/overview.md](architecture/overview.md) | 系统分层、模块边界、数据模型 |
| [product/roadmap.md](product/roadmap.md) | 里程碑 M0–M5 |
| [decisions/](decisions/) | 已采纳的架构决策（ADR） |
| [specs/](specs/) | **待编写** 的实现规范（接口契约） |
| [team/plan.md](team/plan.md) | 5 人分工与接口变更规则 |
| [research/kernel-candidates.md](research/kernel-candidates.md) | 历史调研结论（已否决 fork 路线） |
| [research/design-system-kernel.md](research/design-system-kernel.md) | 轻量设计内核调研 |

## 仓库地图

```text
crates/
  od-core/        产物、工作区、技能、运行原语
  od-mcp/         MCP 工具面（主入口，占位）
  od-cli/         `odl` 命令入口（脚本化辅助）
  od-preview/     预览/导出边界（占位）
docs/             本文档树
skills/           内置技能（SKILL.md）
templates/        启动模板
```

## 当前状态

- **定位已定**：MCP 优先 / Agent 驱动，无独立壳层 app（见 [ADR 0001](decisions/0001-binary-shell-first.md)）
- **M0 进行中**：文档契约 + Rust 脚手架 + 最小 CLI（`init` / `new` / `preview` 占位）
- **Design kernel 路线已定**：极轻、框架无关，使用 CSS variables + recipe，不内置 UI runtime
- **下一优先级**：M1 本地产物+预览闭环，再推进 M2 的 MCP 桥接（产品主交付）

## 文档语言

产品与技术文档以中文为主。对外 README 保留简短英文摘要。
