# 架构决策记录（ADR）

| ADR | 标题 | 状态 |
|-----|------|------|
| [0001](0001-binary-shell-first.md) | MCP 优先 / Agent 驱动 | 已采纳 |
| [0002](0002-lightweight-design-kernel.md) | 轻量 Design Kernel | 已采纳 |

## 开放项（待新 ADR）

| 主题 | 阻塞里程碑 | 说明 |
|------|------------|------|
| v1 是否内置模型调用 | M3 | 或仅编排外部 Agent |
| Markdown 预览实现 | M1 | 原生 vs HTML 中转 |
| 最小导出集合 | M4 | 是否 M1 就要 ZIP |

开放项关闭时：新增 `decisions/000N-*.md`，并更新 [architecture/overview.md](../architecture/overview.md) 技术选型表。
