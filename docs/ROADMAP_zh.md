# 路线图

## M0：仓库契约

- PRD 与架构文档。
- Rust 工作区脚手架。
- 内置技能占位。
- 最小 CLI 命令集。

## M1：本地产物闭环

- `odl init`
- `odl new html/docs/slides`
- 原生预览壳层打开产物文件夹
- 文件监视器刷新预览
- 生成交接文件

## M2：Agent 桥接

- MCP 服务暴露创建/预览/导出/交接能力。
- 面向 Codex、Claude Code、OpenCode、Cursor 和 Zed 的交接提示词。
- 可选的外部命令运行器，用于 Agent 创建的产物。

## M3：内置生成

- BYOK OpenAI 兼容提供商。
- 直接生成 HTML、文档和幻灯片。
- 模式专用提示词模板与质量检查。

## M4：导出

- 自包含 HTML。
- Markdown。
- 从预览导出 PDF。
- ZIP 产物包。

## M5：薄插件

- Cursor 命令集成。
- Zed 命令/面板集成。
- 如有必要，VS Code 预览面板。
- Codex/Claude Code MCP 安装助手。
