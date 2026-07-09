# Release Checklist

**状态**：M1 发布前检查清单  
**里程碑**：M1 末  
**实现位置**：N/A（流程文档）

## 目的

定义 M1 演示和二进制发布前的检查项。M1 目标是本地产物闭环，不是完整商业发行。

## 技术栈检查

- CLI 使用 `clap`。
- 日志使用 `tracing`。
- 预览默认使用 `wry` 或已记录替代原因。
- 文件监听使用 `notify`。
- Markdown 使用 `comrak`。
- 模板使用 `minijinja` 或等价 Rust-native 方案。
- 未引入 Electron、Node dev server、React/Tailwind runtime 作为默认路径。

## 功能检查

- `odl init <dir>` 创建 workspace。
- `odl new html <dir>` 创建 HTML artifact。
- `odl new docs <dir>` 创建 Markdown artifact。
- `odl new slides <dir>` 创建 slides artifact。
- 每个 artifact 有 `manifest.json`、主文件、`handoff.md`。
- HTML / slides 有 `assets/od-design.css` 或 inline design CSS。
- `odl preview <dir>` 能打开预览窗口。
- 修改主文件后预览刷新。
- 主文件缺失时显示错误页。
- `--external-browser` fallback 可用或给出明确错误。

## 平台检查

最低 M1 要求：

| 平台 | 要求 |
|------|------|
| Windows | 必测。WebView2 不可用时给出提示。 |
| macOS | 尽量测试。WKWebView 应可用。 |
| Linux | best effort。WebKitGTK 缺失时给出提示或 fallback。 |

## 文档检查

- README 指向中文 docs。
- specs 状态与实现状态一致。
- ADR 0001、0002 与实现不冲突。
- 已知限制写入 release notes。

## 发布检查

- `cargo check` 通过。
- CLI help 可读。
- 示例命令可复制运行。
- GitHub Release 二进制可由 `cargo-dist` 或手动流程生成。
- 不包含 `external/` 上游调研克隆。

## 已知可接受限制

- PDF 导出未实现。
- MCP stdio server 已可启动（`odl mcp`），create/preview/handoff 已接；各 Agent 配置说明与真实客户端联调仍待收尾。
- 当前 MCP transport 为手写 JSON-RPC，尚未迁到 `rmcp`。
- Linux WebView 依赖可能需要用户安装。
- Markdown 代码高亮可暂缺。
- 自动更新暂缺。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-08 | 对齐当前限制：MCP 不是完全未实现，而是 stdio server 待接。 |
| 2026-07-09 | 同步代码：stdio server 与 create/preview/handoff 已接入；剩余为客户端配置/联调。 |
