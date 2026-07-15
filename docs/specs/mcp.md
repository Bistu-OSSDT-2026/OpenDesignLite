# MCP

**状态**：部分实现（stdio server + create/preview/handoff/export handler 已接入；客户端配置说明与真实 Agent 联调仍待收尾）  
**里程碑**：M2  
**实现位置**：`crates/od-mcp`（入口：`odl mcp`）

## 目的

定义 Open Design Lite 的 MCP 工具面。MCP 是 Agent 集成层，必须调用 `od-core` 的同一套 artifact/workspace 规则，不得重复实现路径、manifest 或 preview 语义。

## 技术栈

| 能力 | 当前实现 | 说明 |
|------|----------|------|
| Transport | 手写 JSON-RPC over stdio（`Content-Length` framing） | `od_mcp::serve_stdio()`；由 `odl mcp` 启动 |
| JSON | `serde`、`serde_json` | 请求/响应与 DTO |
| Schema | `schemars` | `tools/list` 的 `inputSchema` |
| 日志 | `tracing`（CLI 侧） | 不把 async 泄漏进 `od-core` |

原计划默认 SDK 为 `rmcp` + `tokio`。当前 M2 用同步手写 stdio server 先打通可连接路径；若后续迁到 `rmcp`，不得破坏现有 tool 名与 JSON 字段。

已支持的 JSON-RPC methods：`initialize`、`ping`、`tools/list`、`tools/call`。

## Tools

| Tool | 对应 CLI | 说明 |
|------|----------|------|
| `artifact_create` | `odl new` | 已接入 stdio `tools/call`。 |
| `artifact_preview` | `odl preview` | spawn `odl preview` 子进程并立即返回；已接入。 |
| `artifact_handoff` | `odl handoff` | 生成或读取 handoff；已接入。 |
| `artifact_export` | `odl export` | 已接入；转发 `od-core::export`（html / md / zip / pdf）。 |

## `artifact_create`

输入：

```json
{
  "kind": "html",
  "dir": "D:/work/demo",
  "title": "Demo",
  "visualBrief": "editorial",
  "overwrite": false,
  "autoPreview": true
}
```

`autoPreview` 可选，默认 `true`：创建成功后自动 spawn 预览窗口（等价随后调一次 `artifact_preview`）。无 GUI 环境或 spawn 失败时**降级不报错**——create 本身仍成功，仅在响应中注明预览未启动。CI/自动化场景传 `false` 关闭。

输出：

```json
{
  "artifact": {
    "kind": "html",
    "root": "D:/work/demo",
    "primaryFile": "index.html",
    "handoff": "handoff.md"
  },
  "nextStep": "Preview window opened. Edit files under D:/work/demo and it live-reloads. If no window appeared, call artifact_preview with dir=\"D:/work/demo\"."
}
```

`nextStep` 可选（additive，向后兼容）：给 agent 的下一步提示文案；autoPreview 关闭或降级时提示改为「调用 `artifact_preview`」。

## `artifact_preview`

Coding agent 的**必选预览路径**：调用本工具打开持久、可 live-reload 的 `odl preview` 窗口。Agent 不得自行用系统浏览器打开产物（`start` / `xdg-open` / `open` / Playwright 等）。默认 `externalBrowser: false`（webview 窗口）、`watch: true`。

输入：

```json
{
  "dir": "D:/work/demo",
  "externalBrowser": false,
  "watch": true
}
```

输出：

```json
{
  "started": true,
  "mode": "webview",
  "alreadyRunning": false
}
```

`alreadyRunning` 可选（additive，向后兼容）：命中单实例锁（同一 artifact 目录已有存活预览窗口）时为 `true`，此时不重复弹窗，仍返回 `started: true`。

如果当前进程环境不能打开 GUI，应返回明确错误，不应阻塞 MCP server。

**stdio 隔离（硬性规则）**：spawn `odl preview` 子进程时必须重定向 stdio——stdin 置空、stdout/stderr 重定向到 `<dir>/.odl/preview.log`，**禁止 `Stdio::inherit()`**。server 的 stdin/stdout 是 JSON-RPC 通道，子进程任何输出都会污染协议流，这是历史上「预览一弹 server 就崩」的根因，不得回退。spawn 后须 `try_wait()` 检测子进程是否立即退出，已退出返回 `preview_crashed`（附 `preview.log` 尾部内容），不得假装成功。

## `artifact_handoff`

输入：

```json
{
  "dir": "D:/work/demo",
  "agent": "opencode",
  "write": true
}
```

输出：

```json
{
  "path": "D:/work/demo/handoff.md",
  "content": "# Handoff: Demo\n..."
}
```

## `artifact_export`

输入：

```json
{
  "dir": "D:/work/demo",
  "format": "zip",
  "out": "D:/work/demo.zip"
}
```

`format`：`html` | `md` | `zip` | `pdf`。`out` 可选。

输出：

```json
{
  "out": "D:/work/demo.zip",
  "format": "zip"
}
```

## 错误格式

MCP SDK 的错误对象必须包含稳定 code：

| Code | 场景 |
|------|------|
| `invalid_args` | 参数不合法。 |
| `artifact_not_found` | artifact 目录不存在。 |
| `manifest_invalid` | manifest 无法解析。 |
| `preview_unavailable` | 无法打开 WebView 或 fallback。 |
| `preview_crashed` | 预览子进程 spawn 成功但短时间内退出。 |
| `format_unsupported` | artifact kind 不支持目标格式，或未知 format。 |
| `export_failed` | 一般导出失败。 |
| `pdf_backend_missing` | 找不到本机 Chrome/Edge PDF 后端。 |
| `resource_missing` | 导出所需资源缺失。 |
| `not_implemented` | 功能尚未实现。 |

## 安全

- MCP 不提供任意 shell 执行 tool。
- 所有路径操作必须通过 `od-core` 校验。
- preview 页面不得获得 MCP 权限。
- MCP 不默认联网。

## 测试

- stdio 初始化成功。
- tools/list 包含四个 tool。
- `artifact_create` 结果与 CLI `new` 一致。
- 无效 kind 返回 `invalid_args`。
- `artifact_preview` 在无 GUI 环境返回明确错误。
- spawn 的预览子进程不继承 server stdio（stdin null、stdout/stderr 落 `preview.log`）。
- 子进程 spawn 后立即退出 → `preview_crashed`，不返回 `started: true`。
- 同一 dir 重复 preview → 第二次返回 `alreadyRunning: true`，不弹第二个窗口。
- `artifact_create` 默认自动弹预览；`autoPreview: false` 不弹；无 GUI 降级时 create 仍成功。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-08 | 对齐当前实现状态：DTO/schema/tool run 已有，stdio server 待接。 |
| 2026-07-09 | 同步代码：`serve_stdio()` + `odl mcp` 已接入 create/preview/handoff；记录手写 JSON-RPC 与原 `rmcp` 计划的偏差。 |
| 2026-07-09 | `artifact_export` 接入 html / md / zip / pdf。 |
| 2026-07-09 | `artifact_preview` 说明：Agent 必选预览路径，默认不开系统浏览器。 |
| 2026-07-14 | `artifact_create` 新增 `autoPreview`（默认 true）与 `nextStep`；`artifact_preview` 新增 `alreadyRunning`；错误码新增 `preview_crashed`；写入 stdio 隔离硬性规则。 |
