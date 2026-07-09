# Export

**状态**：已实现（M3：html 目录 / md / zip / pdf 本机浏览器）  
**里程碑**：M3  
**实现位置**：`crates/od-core/src/export.rs`（CLI / MCP 共用）

## 目的

定义 `odl export` 行为。导出必须保持本地优先和轻量原则，默认不依赖 Playwright、Pandoc、wkhtmltopdf、WeasyPrint 或 bundled browser。

## 命令

```text
odl export <artifact-dir> --format html|md|zip|pdf [--out <path>]
```

参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `--format` | 必填 | `html`、`md`、`zip`、`pdf`。 |
| `--out` | 当前目录 | 输出路径。 |
| `--single-file` | false | HTML 单文件导出，后续实现。 |
| `--backend` | `auto` | PDF 后端选择，M3 定。 |

## 格式行为

| Format | 行为 | 默认技术 |
|--------|------|----------|
| `html` | HTML/slides 输出目录；docs 渲染为 HTML 目录。 | 文件复制 + `comrak`/`minijinja` |
| `md` | docs 输出 `doc.md`；HTML/slides 不支持或只导出 handoff。 | 文件复制 |
| `zip` | 打包 artifact 目录。 | `zip` crate |
| `pdf` | 浏览器打印路径。 | 先 spike WebView print，必要时外部 Chrome/Edge |

## ZIP 规则

包含：

- `manifest.json`
- 主文件
- `assets/`
- `handoff.md`

排除：

- `.git/`
- `.log/`
- 系统临时文件
- preview 临时文件

ZIP 内路径必须使用 `/`。

## HTML 规则

M3 初始只要求目录导出：

```text
export/
  index.html | slides.html | doc.html
  assets/
```

单文件 HTML 后续可用 `lol_html` 做资源内联，但不作为 M3 的硬性入口条件。

## PDF 规则

推荐顺序：

1. WebView / 系统浏览器打印能力。
2. 检测本机 Chrome/Edge headless print。
3. 用户显式选择外部 backend。

默认不下载 Chromium，不打包 Playwright browser。

## 错误

| Code | 场景 |
|------|------|
| `format_unsupported` | artifact kind 不支持目标格式。 |
| `export_failed` | 一般导出失败。 |
| `pdf_backend_missing` | 找不到 PDF 后端。 |
| `resource_missing` | 导出所需资源缺失。 |

## 测试

- ZIP 导出可解压并保留相对路径。
- HTML 目录导出可离线打开。
- docs 导出 HTML 使用 `od-design.css`。
- PDF backend 不存在时返回可读错误。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-09 | 实现落地到 `od-core::export`；pdf 走本机 Chrome/Edge headless。 |
