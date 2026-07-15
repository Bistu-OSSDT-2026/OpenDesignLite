# 产品需求：Open Design Lite

## 一句话

本地设计助手，**通过 MCP 接入编码 Agent**，让用户一句话就能创建、润色、预览、交接小型产物（HTML 页面、Markdown 文档、HTML 幻灯片）。Agent 是交互界面，预览是自动弹出的常驻窗口。

**定位关键词：简单好部署、快速安装、快速使用。** 不是 Web 平台，不是 Figma 替代品，也不自带对话/预览 app。体验应像给 Agent 加一个能力，而非启动一套平台。

## 问题

重型 AI 设计工具启动慢、配置重，对日常小型任务过度设计：

- 快速 HTML 原型
- 润色文档
- 起草幻灯片
- 预览并微调
- 交给编码 Agent 继续实现

用户要的是**速度**和**低门槛**。低门槛不止运行时轻，更指**从零到可用的路径短**：一条命令安装、一条命令接入 Agent，不要求用户理解 MCP 协议或手工编辑 JSON 配置。

## 目标

| 目标 | 验收直觉 |
|------|----------|
| 快速安装 | 一条命令安装 release 二进制；从下载到 `odl setup` 完成 < 1 分钟 |
| 配置极简 | 运行 `odl setup` 自动写入 Agent 的 MCP 配置，无需手工编辑 JSON |
| 一分钟内可预览产物 | 一句话 → 有文件 → 自动弹出预览 |
| 产物即普通文件 | 可直接用编辑器打开、git 管理 |
| Agent 友好 | MCP 为主通道，CLI / handoff.md 为辅 |
| 原生预览 | 无需浏览器标签页或 dev server；所见即部署后所得（固定视口） |
| 三种模式做精 | HTML、文档、幻灯片 |
| 默认更好看 | 内置轻量 design kernel + 深色预览壳，而不是裸 HTML |
| 本地运行，冷启动快 | 小工具级启动感 |

## 非目标

- 完整替代 Figma / 协作设计套件
- 多用户协作、云账号、同步
- 插件市场
- 重型项目管理
- 通用编码 Agent 替代品
- v1 完整可视化编辑器
- v1 内置 design agent 对话（BYOK design agent 规划为后续里程碑 M4，v1 仅在预览壳中预留聊天面板占位与接口，见 [ADR 0003](../decisions/0003-no-built-in-model-calls.md)）
- 内置 React/Tailwind/shadcn/ui/Radix/Web Components 运行时
- 需要 dev server 或前端构建链才能预览的设计系统

## 用户

- 使用 Codex、Claude Code、OpenCode、Cursor、Zed 的构建者
- 需要快速文档/原型/演示的 PM、创始人
- 希望实现前有可预览产物的工程师

## 产品形态

一个内核，两个入口，一个预览层：

```text
                    编码 Agent（opencode / Codex / Claude Code …）
                           │ MCP
                    ┌──────v──────┐
                    │   od-core   │
                    └──────┬──────┘
              ┌────────────┴────────────┐
              v                         v
         MCP（主入口）              CLI（脚本化辅助）
         od-mcp                     od-cli
              └────────────┬────────────┘
                           v
                    od-preview（自动弹出、常驻、实时刷新）
```

**一等公民集成契约**：MCP 工具、CLI 命令、磁盘文件、handoff 提示词。MCP 是主通道；CLI 供脚本化使用。

## MVP 模式

### HTML 页面

- **输入**：提示词、可选参考文件、可选风格说明
- **输出**：`index.html`、`assets/`（可选）、轻量 design token/recipe 样式、`handoff.md`
- **预览**：系统 WebView

### 文档润色

- **输入**：Markdown 或纯文本
- **输出**：`doc.md`、`handoff.md`（可选）
- **预览**：原生 Markdown 渲染

### 幻灯片

- **输入**：主题、大纲、笔记或文档
- **输出**：`slides.html`、轻量 design token/recipe 样式、`handoff.md`（可选）
- **预览**：系统 WebView

## Design Kernel 原则

`odl` 的默认视觉质量来自轻量 design kernel，而不是绑定现成 UI kit。

- core 只定义 `--od-*` token、layout primitive、component recipe、pattern guideline。
- templates 使用静态 CSS 提升默认观感。
- skills 使用 visual brief 和 pattern recipe，避免 AI 生成统一 SaaS 模板。
- Tailwind、shadcn/ui、React、Web Components 只能作为后续 adapter 或导出目标。
- 任何需要 dev server 的方案都不能成为 MVP 默认路径。

## 核心工作流

1. 运行安装脚本获取二进制 → `odl setup` 自动检测并写入编码 Agent 的 MCP 配置（一次性）
2. 一句话向 Agent 描述需求（HTML / 文档 / 幻灯片）
3. Agent 调 MCP tool（`artifact_create`）创建产物工作区（纯文件）
4. 应用轻量 design kernel 的默认视觉约束
5. `artifact_create` 成功后默认自动弹出常驻预览窗口（`autoPreview`，可关；也可显式调 `artifact_preview`）
6. 用户在 Agent 交互界面继续微调，Agent 经 MCP 改文件
7. 预览窗口随文件变化实时刷新
8. 导出或生成 `handoff.md` 交接给其他 Agent

> CLI（`odl new` / `preview` …）是上述流程的脚本化等价物，供无 MCP 场景或自动化使用。

## 成功指标

- 冷启动对小体量本地工具近乎即时
- 一条命令创建产物工作区
- HTML / 幻灯片预览无需项目配置
- HTML / 幻灯片默认观感明显优于裸模板
- 文档润色无需浏览器
- 仅凭 `handoff.md` 即可理解如何交给 Codex / OpenCode / Claude Code

## 待决问题

见 [decisions/README.md](../decisions/README.md) 开放项；实现前需在对应 spec 中关闭。
