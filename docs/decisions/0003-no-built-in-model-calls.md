# ADR 0003：v1 不内置模型调用 / 仅编排外部 Agent

**状态**：已采纳（2026-07-14 修订：由「不内置模型调用」细化为「v1 不内置，规划 BYOK design agent 为后续里程碑 M4，v1 仅预留 UI 占位与接口边界」）
**日期**：2026-07
**替代**：内置 BYOK 生成链路（原 M3「内置生成」）

## 背景

原路线图 M3「内置生成」计划让 `odl` 自带模型调用：BYOK 接 OpenAI 兼容提供商，直接从提示词生成 HTML/文档/幻灯片。技术选型表把「模型调用：v1 仅编排外部 Agent vs 内置 BYOK」列为开放项。

但产品定位已收敛为 MCP 优先 / Agent 驱动（见 [ADR 0001](0001-binary-shell-first.md)）：用户已经在用带模型能力的编码 Agent，Agent 本身承担生成。再自研一套 BYOK 模型调用会重复 Agent 的能力，并把 provider 配置、密钥管理、异步 runtime 等复杂度引入本应轻量的内核。

## 决策

**v1 不内置任何模型调用，只编排外部 Agent。**

- 删除路线图 M3「内置生成」；导出顺延为 M3。
- 关闭「v1 是否内置模型调用」开放项。
- 生成由外部 Agent 经 MCP / CLI / 文件交接完成，`od-core` 只负责产物、工作区、技能与预览。
- `smoke-prompts` 从「回归测试内置生成」改为「验证外部 Agent 经 MCP 产出的产物质量」。
- （2026-07-14 修订新增）为后续 BYOK design agent 预留 UI 位置与进程边界：预览壳中放置可折叠聊天面板**占位**（空态文案），并预留 shell↔agent 的桥接接口占位（`window.__odlShellBridge` 空对象）。**本决策周期内不实现任何 LLM 调用、密钥管理或聊天逻辑。**

## 后果

**正面**

- 内核保持轻量：无 provider SDK、无密钥管理、无 async 泄漏进 `od-core`。
- 不与编码 Agent 的核心能力重复，定位更清晰。
- 减少 v1 范围，加快收敛到可发布。

**负面**

- 没有外部 Agent 时，无法「纯 CLI 从提示词直接生成产物」——用户必须自带 Agent。
- 若未来要做离线自带生成，需重开此决策并新增 ADR。
- 预览壳中的 design agent 面板占位不改变本决策的范围边界：它是纯 UI/接口预留，不引入 provider SDK、密钥管理或 async runtime。

## 后续

BYOK design agent（内置聊天框 + 用户自带 API key）规划为路线图 **M4** 里程碑（见 [roadmap](../product/roadmap.md)）。届时需新增 ADR 明确 provider 抽象、密钥存储与 shell↔agent 进程通信方案；本 ADR 预留的占位（聊天面板 DOM 位置、`__odlShellBridge` 接口名）是该 ADR 的输入约束。
