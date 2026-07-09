# ADR 0001：MCP 优先 / Agent 驱动

**状态**：已采纳（2026-07 修订，取代原「二进制壳层优先」决策）
**日期**：2026-06 初定；2026-07 修订
**替代**：壳层优先（原决策）、插件优先、Web 平台优先

## 背景

产品需轻量、启动快、开箱即用，适用于日常小型设计产物。曾考虑过的入口形态：

1. 独立二进制壳层 app（左输入栏 + 右预览栏的双栏界面）
2. 编辑器专用插件（Codex/Cursor/Zed 侧边栏）
3. **通过 MCP 协议接入既有编码 Agent**（opencode、Codex、Claude Code 等）

原决策为「先做独立二进制壳层」。但实践方向已收敛为：不再自研任何对话/预览界面——用户已经在用带 MCP 能力的编码 Agent，Agent 本身就是交互界面。

## 决策

**主产品面是 MCP 工具面。** 编码 Agent 通过 MCP 驱动 `od-core` 的同一套 artifact/workspace 规则；`artifact_preview` 自动拉起一个**常驻、随文件变化实时刷新**的预览窗口（`od-preview`，系统 WebView）。

- **不做**独立壳层 app（左对话 + 右预览的双栏界面）。
- **不做**编辑器专用插件——Agent 用自己的 MCP 配置连接即可。
- `odl` CLI 保留为**脚本化辅助工具**，与 MCP 复用同一内核。

典型闭环：用户在 Agent 里一句话描述需求 → Agent 调 MCP tool 生成产物并弹出预览 → 用户在 Agent 交互界面继续微调 → Agent 经 MCP 改文件 → 预览窗口实时刷新。

## 后果

**正面**

- 配置极简：Agent 里配好一次 MCP server 即可用，无复杂安装。
- 原生集成任意支持 MCP 的 Agent，不被单一编辑器/插件 API 绑架。
- 无需自研并长期维护一套对话/预览 UI。
- 预览与 Agent 解耦：预览只是一个常驻窗口，靠文件监听刷新。
- 单一内核，产物存储与预览行为一致；CLI 与 MCP 契约稳定。

**负面**

- 依赖目标 Agent 具备 MCP 客户端能力。
- 需要可打开 GUI 的本地环境来显示预览窗口（无 GUI 时须返回明确错误，不阻塞 MCP server）。

## 备注

- 使用系统 WebView 预览 **可接受**（`wry` + `tao`）。
- 带 dev server 的重型 Web 平台 **拒绝**。
- 调研曾建议 fork 大型 `open-design` 项目 → **未采纳**，见 [kernel-candidates.md](../research/kernel-candidates.md)。
