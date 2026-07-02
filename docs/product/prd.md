# 产品需求：Open Design Lite

## 一句话

本地二进制设计助手，快速创建、润色、预览、交接小型产物（HTML 页面、Markdown 文档、HTML 幻灯片）。

不是 Web 平台，不是 Figma 替代品。体验应像打开一个小工具，而非启动一套平台。

## 问题

重型 AI 设计工具启动慢、配置重，对日常轻量任务过度设计：

- 快速 HTML 原型
- 润色文档
- 起草幻灯片
- 预览并微调
- 交给编码 Agent 继续实现

用户要的是**速度**和**低门槛**。

## 目标

| 目标 | 验收直觉 |
|------|----------|
| 本地二进制，冷启动快 | 小工具级启动感 |
| 一分钟内可预览产物 | `odl new` → 有文件 → 能预览 |
| 产物即普通文件 | 可直接用编辑器打开、git 管理 |
| Agent 友好 | CLI / MCP / handoff.md 三通道 |
| 原生预览 | 无需浏览器标签页或 dev server |
| 三种模式做精 | HTML、文档、幻灯片 |
| 默认更好看 | 内置轻量 design kernel，而不是裸 HTML |

## 非目标

- 完整替代 Figma / 协作设计套件
- 多用户协作、云账号、同步
- 插件市场
- 重型项目管理
- 通用编码 Agent 替代品
- v1 完整可视化编辑器
- 内置 React/Tailwind/shadcn/ui/Radix/Web Components 运行时
- 需要 dev server 或前端构建链才能预览的设计系统

## 用户

- 使用 Codex、Claude Code、OpenCode、Cursor、Zed 的构建者
- 需要快速文档/原型/演示的 PM、创始人
- 希望实现前有可预览产物的工程师

## 产品形态

一个内核，多个入口：

```text
                    ┌─────────────┐
                    │   od-core   │
                    └──────┬──────┘
           ┌───────────────┼───────────────┐
           v               v               v
    Native Shell        CLI            MCP
    （主 UI/预览）    （脚本化）      （Agent 集成）
           │               │               │
           └───────────────┴───────────────┘
                           │
                    Editor Extensions
                    （薄适配，M5）
```

**一等公民集成契约**：CLI 命令、MCP 工具、磁盘文件、handoff 提示词。插件只做转发，不持有状态。

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

1. 打开壳层或运行 `odl`
2. 选择模式：HTML / 文档 / 幻灯片
3. 输入提示词或提供文件
4. 内核创建产物工作区（纯文件）
5. 应用轻量 design kernel 的默认视觉约束
6. 首个文件生成后立即预览
7. 迭代编辑
8. 导出或生成交接文件给外部 Agent

## 成功指标

- 冷启动对小体量本地工具近乎即时
- 一条命令创建产物工作区
- HTML / 幻灯片预览无需项目配置
- HTML / 幻灯片默认观感明显优于裸模板
- 文档润色无需浏览器
- 仅凭 `handoff.md` 即可理解如何交给 Codex / OpenCode / Claude Code

## 待决问题

见 [decisions/README.md](../decisions/README.md) 开放项；实现前需在对应 spec 中关闭。
