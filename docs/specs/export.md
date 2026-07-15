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

| Artifact | 纸张 |
|----------|------|
| `html` / `docs` | 浏览器默认纸张（A4/Letter），不受 Slides 规则影响。 |
| `slides` | 固定 16:9（13.333in × 7.5in），见下节。 |

### Slides PDF 规则（16:9）

**机制**：Chrome/Edge 的 headless CLI **没有**稳定的纸张尺寸开关（`--print-to-pdf-page-size` 不存在；那是 DevTools Protocol `Page.printToPDF` 的参数）。纸张尺寸唯一可靠的控制方式是 CSS `@page` 规则，Chrome 与 Edge 共享 Blink 打印管线，行为一致：

```css
@page { size: 13.333in 7.5in; margin: 0; }
@media print {
  .od-slide { min-height: unset; width: 13.333in; height: 7.5in; overflow: hidden; break-after: page; }
  .od-slide:last-of-type { break-after: auto; }
}
```

- `@page` 16:9 规则**仅对 slides 生效**；html/docs 维持默认纸张。
- 每个 `.od-slide` 在打印时固定为整页尺寸并 `overflow: hidden`，杜绝内容溢出导致 Blink 自动分页产生错乱/空白页；末页取消 `break-after` 避免尾部空白页。
- `export_pdf` 的浏览器命令行参数不变（`--headless=new --disable-gpu --no-pdf-header-footer --print-to-pdf=...`）。

**staging 内样式升级**：`assets/od-design.css` 是产物创建时写入的静态文件，旧产物不会自动获得新打印规则。因此 PDF 导出在**临时 staging 目录**（`prepare_pdf_source` 的副本）内**总是**用当前内核版本重写 `od-design.css`；产物目录内的原文件一概不动（用户手改的 CSS 不会被覆盖，但手改部分也不会出现在 PDF 里——PDF 始终按当前内核样式渲染）。

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
- slides PDF：每页尺寸 13.333in × 7.5in（±容差），无内容溢出产生的额外空白页；旧产物（staging 前 CSS 无 `@page` 规则）导出同样满足。
- html/docs PDF 纸张不受 slides 规则影响。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-09 | 实现落地到 `od-core::export`；pdf 走本机 Chrome/Edge headless。 |
| 2026-07-14 | 新增 Slides PDF 16:9 规则（CSS `@page`，非 CLI flag）；PDF staging 内总是重写 `od-design.css`，不动产物原文件。 |
