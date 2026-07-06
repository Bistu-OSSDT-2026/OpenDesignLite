# 实现规范（Specs）

**产品文档说「做什么」；架构文档说「怎么分层」；本目录说「接口长什么样」。**

Specs 是实现前的一份真相。每份 spec 必须可测试、无歧义，并说明哪些内容仍是草案。

## 规范清单

| Spec | 负责人 | 里程碑 | 状态 |
|------|--------|--------|------|
| [artifact-workspace](artifact-workspace.md) | Product & Kernel Lead | M1 | 草案 |
| [preview](preview.md) | Preview Shell | M1 | 草案 |
| [handoff](handoff.md) | Product & Kernel Lead | M1 | 草案 |
| [cli](cli.md) | Product & Kernel Lead | M1 | 草案 |
| [built-in-skills](built-in-skills.md) | Skills & Templates | M1 | 草案 |
| [design-kernel](design-kernel.md) | Product & Kernel Lead + Skills | M1 | 草案 |
| [release-checklist](release-checklist.md) | Product & Kernel Lead | M1 末 | 草案 |
| [mcp](mcp.md) | Product & Kernel Lead | M2 | 草案 |
| [smoke-prompts](smoke-prompts.md) | Skills & Templates | M3 | 草案 |
| [export](export.md) | Export & Packaging | M4 | 草案 |

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
| MCP | M2 使用 `rmcp` + `tokio` | 不把 async 泄漏进 `od-core` |
| 发布 | `cargo-dist` | CLI 二进制发布；GUI 打包后置 |

## 明确禁止作为默认路径

- Electron、Node/Vite/Next dev server。
- React、Tailwind、Radix、shadcn/ui、Lit/Web Components runtime 进入 `od-core`。
- Playwright bundled browser、wkhtmltopdf、Pandoc、WeasyPrint 作为默认导出依赖。
- 每个 template 自己发明 token、颜色、圆角、间距命名。

## 编写与变更规则

1. spec 字段变化必须同步相关架构文档或 ADR。
2. M1 实现前，`artifact-workspace`、`preview`、`cli`、`handoff`、`built-in-skills`、`design-kernel` 必须至少评审一次。
3. `mcp`、`export`、`smoke-prompts` 可先保持草案，但不得与 M1 已定字段冲突。
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
