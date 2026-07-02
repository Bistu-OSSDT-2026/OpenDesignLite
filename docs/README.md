# Open Design Lite 文档

命令名 `odl`。轻量、本地优先的设计/内容辅助工具。

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
apps/
  shell/          原生预览壳层（占位）
  extensions/     编辑器薄适配层（占位）
crates/
  od-core/        产物、工作区、技能、运行原语
  od-cli/         `odl` 命令入口
  od-mcp/         MCP 工具面（占位）
  od-preview/     预览/导出边界（占位）
docs/             本文档树
skills/           内置技能（SKILL.md）
templates/        启动模板
```

## 当前状态

- **M0 进行中**：文档契约 + Rust 脚手架 + 最小 CLI（`init` / `new` / `preview` 占位）
- **Design kernel 路线已定**：极轻、框架无关，使用 CSS variables + recipe，不内置 UI runtime
- **下一优先级**：补齐 [specs/](specs/) 中的 artifact-workspace 与 preview 规范，再推进 M1

## 文档语言

产品与技术文档以中文为主。对外 README 保留简短英文摘要。
