# 组队招募（4/5）

**Open Design Lite**（`odl`）— 极简、本地优先的 AI 设计/内容辅助工具。

## 我们在做什么

三类高频场景：HTML 原型、Markdown 润色、轻量 HTML 幻灯片。

**不是**重型 Web 平台，**不是** Figma 替代品。目标：**小、快、能跑通、能展示**。

```text
输入需求 → 生成普通文件 → 本地预览 → handoff 给 Agent 继续改
```

## 技术方向

- 核心：**Rust** 本地二进制
- 预览：Tauri / Wry 等轻量 WebView（M1 前定 ADR）
- 设计：极轻 design kernel，CSS variables + recipe，不绑 React/Tailwind
- 集成：CLI、MCP、纯文件交接；插件后置
- 模型：不内置模型调用，只编排外部 Agent（见 [ADR 0003](../decisions/0003-no-built-in-model-calls.md)）

详见 [docs/README.md](../README.md)。

## 可能分工

| 方向 | 内容 |
|------|------|
| 产品/文档 | PRD、演示、specs |
| Rust/CLI | `odl` 命令与工作区 |
| 预览界面 | HTML/MD/Slides 本地窗口 |
| Skills/Templates | 内置提示词与模板 |
| 导出打包 | ZIP/HTML/PDF |
| Agent 集成 | MCP、编辑器薄适配 |

每人一块能讲清楚、能答辩的成果；不要求独扛核心。

## 队友期望

- 对方向有兴趣，愿意做小工具
- Rust 可边学边做
- 能用 AI 辅助，但理解自己负责的代码
- 每周有一点可见进展即可

**不做硬性要求**：不必精通 Rust/桌面端/天天在线。

## 当前状态

仓库有 PRD、架构、路线图、Rust 脚手架；**M0 尾声，M1 待启动**。现在加入可参与 MVP 形态与技术选型。

产品详情：[product/prd.md](../product/prd.md)  
分工参考：[plan.md](plan.md)
