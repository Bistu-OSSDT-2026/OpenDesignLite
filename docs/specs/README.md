# 实现规范（Specs）

**产品文档说「做什么」；架构文档说「怎么分层」；本目录说「接口长什么样」。**

Specs 是实现契约。每份 spec 必须可测试、无歧义，并说明哪些内容仍是草案或已落地。

## 规范清单

| Spec | 负责人 | 里程碑 | 状态 |
|------|--------|--------|------|
| [artifact-workspace](artifact-workspace.md) | Product & Kernel Lead | M1 | 部分实现 |
| [preview](preview.md) | Preview | M1 | 已接入 CLI |
| [handoff](handoff.md) | Product & Kernel Lead | M1 | 已接入 CLI/MCP run |
| [cli](cli.md) | Product & Kernel Lead | M1 | 部分实现 |
| [built-in-skills](built-in-skills.md) | Skills & Templates | M1 | 部分实现 |
| [design-kernel](design-kernel.md) | Product & Kernel Lead + Skills | M1 | 部分实现 |
| [release-checklist](release-checklist.md) | Product & Kernel Lead | M1 末 | M1 发布前检查清单 |
| [mcp](mcp.md) | Product & Kernel Lead | M2 | stdio server + create/preview/handoff/export 已接入；客户端配置/联调待收尾 |
| [smoke-prompts](smoke-prompts.md) | Skills & Templates | M2 | 草案 |
| [setup](setup.md) | Product & Kernel Lead | M2 收尾 | 草案（`odl setup` + 安装脚本契约） |
| [export](export.md) | Export & Packaging | M3 | 已实现（2026-07-14 新增 Slides PDF 16:9 规则） |

## 已采纳技术栈边界

| 层 | 默认技术 | 说明 |
|----|----------|------|
| CLI | `clap` | 子命令、help、退出码稳定优先 |
| 错误 | app 层 `anyhow`，库层 `thiserror` | CLI 快速上下文 + core 稳定错误类型 |
| 日志 | `tracing` | CLI、preview、MCP 共用结构化日志 |
| 配置/manifest | `serde`、`serde_json`、`toml` | manifest 使用 JSON；用户配置可用 TOML |
| 预览 | `wry` + 系统 WebView | M1 默认；外部浏览器是 fallback |
| 文件监听 | `notify` + debounce | watcher 行为由 preview spec 约束 |
| Markdown | `comrak` | Markdown → HTML |
| Markdown 清洗 | `ammonia` | 只清洗 Markdown 渲染得到的 HTML |
| 模板 | `minijinja` + `include_str!` | 预览包装、错误页、handoff、starter |
| Design kernel | 手写 `od-design.css` + `--od-*` token | 不内置 UI runtime |
| MCP | 手写 JSON-RPC stdio（`odl mcp`）；原计划可迁 `rmcp` | 不把 async 泄漏进 `od-core` |
| 发布 | `cargo-dist` | CLI 二进制发布；GUI 打包后置 |

## 明确禁止作为默认路径

- Electron、Node/Vite/Next dev server。
- React、Tailwind、Radix、shadcn/ui、Lit/Web Components runtime 进入 `od-core`。
- Playwright bundled browser、wkhtmltopdf、Pandoc、WeasyPrint 作为默认导出依赖。
- 每个 template 自己发明 token、颜色、圆角、间距命名。

## 编写与变更规则

1. spec 字段变化必须同步相关架构文档或 ADR。
2. 改动已实现字段时，必须同步相关 crate 注释或测试。
3. `smoke-prompts` 可先保持草案；`mcp` / `export` 已实现字段变更不得破坏当前工具名与 JSON 字段。
4. 新增 runtime 依赖前必须能映射到本目录某份 spec。

## 建议实现顺序

```text
1. artifact-workspace
2. design-kernel
3. built-in-skills
4. cli
5. preview
6. handoff
7. release-checklist
8. mcp
9. smoke-prompts
10. export
```

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 根据技术栈调研补齐 specs 草案，并新增 design-kernel spec。 |
| 2026-07-14 | 新增 setup spec；preview（壳/固定视口/稳定性）、mcp（autoPreview/stdio 隔离）、export（Slides PDF 16:9）、cli（`odl setup`）同步修订。 |
