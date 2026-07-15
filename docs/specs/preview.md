# Preview

**状态**：已接入 CLI；2026-07-14 修订：新增深色壳、固定视口、稳定性规则（实现进行中）  
**里程碑**：M1（壳与稳定性属 M2 收尾）  
**实现位置**：`crates/od-preview`（由 `odl preview` 与 MCP `artifact_preview` 共用拉起）

## 目的

定义本地预览行为：如何打开 HTML、Markdown、Slides artifact，如何监听文件变化，如何刷新 WebView，如何展示错误页。默认技术为 `wry` + 系统 WebView，外部浏览器作为 fallback。

## 范围

- 包含 artifact 类型检测。
- 包含 WebView 加载策略（custom protocol + 壳页面 iframe）。
- 包含窗口尺寸与视口规则。
- 包含 Markdown 渲染路径。
- 包含文件监听与刷新策略。
- 包含错误页与 fallback。
- 包含稳定性规则（stdio、崩溃日志、单实例锁）。
- 不包含 PDF 导出，见 [export.md](export.md)。
- 预览窗口由**深色壳（shell）**包裹产物内容区；壳只承载展示性信息（产物名、类型、reload 状态）与 design agent 面板**占位**，不含任何会改动产物的交互 UI（改动发生在编码 Agent 里）。

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

2026-07-14 修订：M1 的 `file://` 直接加载被 **custom protocol + 壳页面 iframe** 替换（原表中的「后续」列已实现）：

| Artifact | 加载方式 |
|----------|----------|
| HTML | 壳页面 `odl-shell://shell/index.html`，内容区 iframe 加载 `odl-shell://shell/artifact/index.html` |
| Slides | 同上，iframe 加载 `odl-shell://shell/artifact/slides.html` |
| Markdown | `comrak` 渲染、`ammonia` 清洗后写入 `.odl/preview.html`；iframe 的 artifact 入口映射到该文件 |

规则：

- custom protocol handler 把 `odl-shell://shell/artifact/<rel>` 映射到 `artifact_root/<rel>`；`<rel>` 规范化后必须仍在 `artifact_root` 内，越权返回 404（复用 `od-core` export 的 `strip_prefix` 校验模式）。
- MIME 按扩展名给出（`text/html` / `text/css` / `image/*` 等），未知扩展名 `application/octet-stream`。
- 壳页面本身由内嵌资源提供（`include_str!`），不落盘、不依赖 artifact 文件。
- 不向 artifact 页面暴露 native IPC。
- **降级路径**：若 custom protocol + iframe 在目标平台 WebView 上不可行（spike 验证不通过），退回 `file://` 直接加载 + 初始化脚本注入壳样式的方案，本 spec 相应回改。

## 窗口与视口

窗口按 artifact 类型使用**固定尺寸、不可缩放**（`with_resizable(false)` + min/max inner size 双保险）。内容区（iframe 视口）尺寸即部署后浏览器视口尺寸——**所见即部署后所得**，这是「锁 16:9」产品要求在预览层的体现（导出层的对应保证见 [export.md](export.md) Slides PDF 规则）：

| Artifact | 内容区（CSS 像素） | 说明 |
|----------|--------------------|------|
| Slides | 1280 × 720 | 精确 16:9 |
| HTML / Markdown | 1366 × 768 | 桌面标准视口 |

- 窗口外尺寸 = 内容区尺寸 + 壳的固定边距（顶栏高度等设计常量），保证内容区是精确目标尺寸而不是近似值。
- 壳采用深色 chrome：顶栏显示产物名（取自 `manifest.json`，缺省用目录名）、kind、reload 状态指示点。

## 稳定性

预览与 MCP server 的进程边界规则（修复「预览一弹 server 就崩」类问题的硬性约束）：

- **stdio 隔离（硬性规则）**：MCP spawn `odl preview` 子进程时**必须**重定向 stdio——stdin 置空，stdout/stderr 重定向到 `<artifact>/.odl/preview.log`。**禁止继承 server 自身的 stdio**（server 的 stdin/stdout 是 JSON-RPC 通道，子进程任何输出都会污染协议流）。
- **崩溃检测**：spawn 后短暂等待并 `try_wait()`；子进程已退出 → 读取 `preview.log` 尾部并返回 `preview_crashed`，不得假装成功。
- **崩溃日志**：预览进程入口安装 panic hook，panic 信息与 backtrace 写入 `<artifact>/.odl/preview.log`。
- **WebView2 user data folder**：固定指向每用户目录（如 `%LOCALAPPDATA%/OpenDesignLite/webview2-data`），全部预览实例共享持久复用；避免默认临时目录被多进程抢占。
- **自动 fallback**：WebView 初始化失败时自动尝试外部浏览器 fallback，两者都失败才报 `fallback_failed`。
- **单实例锁**：`<artifact>/.odl/preview.lock`（含心跳 mtime）。同一 artifact 目录重复 preview 时命中有效锁 → 不弹第二个窗口，MCP 返回 `alreadyRunning: true`。锁过期（心跳超时）视为无效，正常拉起。清理不依赖 graceful shutdown，由下次启动按心跳过期兜底。

## 预留位置（不实现）

为后续 M4 BYOK design agent 预留（见 [ADR 0003](../decisions/0003-no-built-in-model-calls.md) 2026-07-14 修订）：

- 壳页面含一个**可折叠聊天面板容器**（初始折叠），内部仅有「Design agent (coming soon)」空态文案。
- 壳页面全局暴露 `window.__odlShellBridge` **空对象**作为 shell↔agent 桥接接口占位。
- 仅占位，无任何实现；不引入 LLM 调用、密钥管理或 IPC 逻辑。

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

reload 只刷新内容区 iframe（重设 `src` 并带时间戳绕过缓存），壳与其状态（面板折叠等）不重置。

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
| `fallback_failed` | 外部浏览器 fallback 失败。 |
| `preview_crashed` | 预览子进程启动后短时间内退出（由 MCP 层经 `try_wait` + `preview.log` 检测）。 |

## 外部浏览器 fallback

触发方式：

- 用户传入 `--external-browser`。
- WebView 初始化失败且平台可打开系统浏览器。

fallback 行为：

- HTML / Slides：打开主文件。
- Markdown：打开渲染后的临时 HTML。
- watcher 不保证自动刷新。

## 安全规则

- 预览页面不得获得 MCP、文件系统、命令执行 IPC。
- `file://` 预览只用于 artifact 自身文件和相对 assets。
- 外部链接默认由 WebView 处理；也可选择用系统浏览器打开。
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
| 2026-07-08 | 对齐当前实现：`odl preview` 已委托 `od-preview`，Markdown 写入 `.odl/preview.html`，新增 `fallback_failed` 错误码。 |
| 2026-07-14 | 大修：加载策略改为 custom protocol + 深色壳 iframe；新增「窗口与视口」（按 kind 固定尺寸不可缩放）、「稳定性」（stdio 隔离/崩溃检测/panic 日志/WebView2 data dir/自动 fallback/单实例锁）、「预留位置」（design agent 面板占位 + `__odlShellBridge`）；错误表新增 `preview_crashed`。 |
