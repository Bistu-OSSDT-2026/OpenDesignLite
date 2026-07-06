# Preview

**状态**：草案  
**里程碑**：M1  
**实现位置**：`crates/od-preview`、`apps/shell`

## 目的

定义本地预览行为：如何打开 HTML、Markdown、Slides artifact，如何监听文件变化，如何刷新 WebView，如何展示错误页。M1 默认技术为 `wry` + 系统 WebView，外部浏览器作为 fallback。

## 范围

- 包含 artifact 类型检测。
- 包含 WebView 加载策略。
- 包含 Markdown 渲染路径。
- 包含文件监听与刷新策略。
- 包含错误页与 fallback。
- 不包含 PDF 导出，见 [export.md](export.md)。
- 不包含 shell 完整产品 UI。

## 技术栈

| 能力 | 默认技术 |
|------|----------|
| WebView | `wry` |
| Window/event loop | `tao` 或 `winit`，以 `wry` 当前推荐为准 |
| 文件监听 | `notify` |
| Markdown 渲染 | `comrak` |
| Markdown HTML 清洗 | `ammonia` |
| 模板包装 | `minijinja` + `include_str!` |
| 日志 | `tracing` |
| 外部浏览器 | `open` crate 或等价平台调用 |

## 输入

```text
odl preview <artifact-dir>
```

内部参数草案：

| 参数 | 类型 | 默认 | 说明 |
|------|------|------|------|
| `artifact_root` | path | 必填 | artifact 目录。 |
| `external_browser` | bool | false | 使用系统浏览器打开。 |
| `watch` | bool | true | 监听文件变化。 |
| `devtools` | bool | false | 开启 WebView devtools，平台支持时有效。 |

## 加载策略

| Artifact | M1 加载方式 | 后续 |
|----------|-------------|------|
| HTML | `file:///<artifact>/index.html` | custom protocol `odl://artifact/...` |
| Slides | `file:///<artifact>/slides.html` | custom protocol + print CSS |
| Markdown | `comrak` 渲染、`ammonia` 清洗、`minijinja` 包装为临时 HTML 后加载 | 虚拟协议、TOC、代码高亮 |

M1 可以使用 `file://`，但不能向 artifact 页面暴露 native IPC。custom protocol 是后续安全增强，不阻塞 M1。

## Markdown 渲染

流程：

```text
doc.md
  -> comrak render HTML
  -> ammonia clean generated HTML
  -> minijinja wrap markdown-preview.html
  -> write temp file or load HTML string
  -> WebView load
```

规则：

- Markdown 中原始 HTML 默认清洗。
- HTML artifact 与 slides 不清洗。
- 代码块 M1 不要求语法高亮。
- Markdown wrapper 应引用 `od-design.css` 或内嵌等效样式。

## 文件监听

监听范围：artifact root。

忽略：

- `.git/`
- `.log/`
- 系统临时文件
- 编辑器 swap 文件
- preview 生成的临时 HTML 文件

刷新规则：

| 场景 | 行为 |
|------|------|
| 主文件变更 | debounce 后 reload。 |
| `assets/` 变更 | debounce 后 reload。 |
| `handoff.md` 变更 | 默认不 reload，除非是 Markdown artifact 的主文件。 |
| `manifest.json` 变更 | 重新检测主文件，必要时 reload。 |
| 主文件被删除 | 显示错误页，不关闭窗口。 |

Debounce 默认 100ms，可在实现中调整到 50-200ms。

## 错误页

错误页必须是内联 HTML，不依赖 artifact assets。

必须展示：

- artifact 路径。
- 错误类型。
- 用户可执行的下一步。
- 如果是 Markdown 渲染错误，展示文件名和简短上下文。

错误类型：

| 错误 | 场景 |
|------|------|
| `artifact_not_found` | 目录不存在。 |
| `primary_file_missing` | 主文件不存在。 |
| `render_failed` | Markdown 渲染或模板包装失败。 |
| `webview_failed` | WebView 初始化或加载失败。 |
| `watch_failed` | watcher 初始化失败；允许继续无 watch 预览。 |

## 外部浏览器 fallback

触发方式：

- 用户传入 `--external-browser`。
- WebView 初始化失败且平台可打开系统浏览器。

fallback 行为：

- HTML / Slides：打开主文件。
- Markdown：打开渲染后的临时 HTML。
- watcher 不保证自动刷新。

## 安全规则

- 预览页面不得获得 shell、文件系统、命令执行 IPC。
- `file://` 预览只用于 artifact 自身文件和相对 assets。
- 外部链接默认由 WebView 处理；后续 shell 可选择用系统浏览器打开。
- 不注入可执行 native bridge。

## 测试

- `index.html` 能打开并显示。
- `slides.html` 能打开并显示。
- `doc.md` 能渲染为 HTML 并显示。
- 修改主文件后窗口刷新。
- 删除主文件后显示错误页。
- WebView 不可用时 fallback 到外部浏览器或给出明确错误。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
