# MCP

**状态**：草案  
**里程碑**：M2  
**实现位置**：`crates/od-mcp`

## 目的

定义 Open Design Lite 的 MCP 工具面。MCP 是 Agent 集成层，必须调用 `od-core` 的同一套 artifact/workspace 规则，不得重复实现路径、manifest 或 preview 语义。

## 技术栈

| 能力 | 默认技术 |
|------|----------|
| MCP SDK | `rmcp` |
| 异步 runtime | `tokio`，限制在 `od-mcp` 内 |
| JSON | `serde`、`serde_json` |
| Schema | `schemars`，如 SDK 需要 |
| 日志 | `tracing` |

M2 默认 transport：stdio。

## Tools

| Tool | 对应 CLI | 说明 |
|------|----------|------|
| `artifact_create` | `odl new` | 创建 artifact。 |
| `artifact_preview` | `odl preview` | 打开本地预览。 |
| `artifact_handoff` | `odl handoff` | 生成或读取 handoff。 |
| `artifact_export` | `odl export` | M4 前可返回未实现。 |

## `artifact_create`

输入：

```json
{
  "kind": "html",
  "dir": "D:/work/demo",
  "title": "Demo",
  "visualBrief": "editorial",
  "overwrite": false
}
```

输出：

```json
{
  "artifact": {
    "kind": "html",
    "root": "D:/work/demo",
    "primaryFile": "index.html",
    "handoff": "handoff.md"
  }
}
```

## `artifact_preview`

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
  "mode": "webview"
}
```

如果当前进程环境不能打开 GUI，应返回明确错误，不应阻塞 MCP server。

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

## 错误格式

MCP SDK 的错误对象必须包含稳定 code：

| Code | 场景 |
|------|------|
| `invalid_args` | 参数不合法。 |
| `artifact_not_found` | artifact 目录不存在。 |
| `manifest_invalid` | manifest 无法解析。 |
| `preview_unavailable` | 无法打开 WebView 或 fallback。 |
| `not_implemented` | M4 功能尚未实现。 |

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

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
