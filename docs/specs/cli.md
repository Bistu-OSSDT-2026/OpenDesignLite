# CLI

**状态**：部分实现（`export` 已接入 M3：html / md / zip / pdf）  
**里程碑**：M1  
**实现位置**：`crates/od-cli`

## 目的

定义 `odl` 命令行接口。CLI 是一等集成面，必须稳定、脚本友好、错误可读，并映射到 `od-core` 的同一套 artifact/workspace 规则。

## 技术栈

| 能力 | 默认技术 |
|------|----------|
| 参数解析 | `clap` derive |
| 应用错误 | `anyhow` |
| 日志 | `tracing` + `tracing-subscriber` |
| JSON/TOML | `serde`、`serde_json`、`toml` |
| 用户目录 | `directories` |

## 全局约定

```text
odl [GLOBAL_FLAGS] <COMMAND> [ARGS]
```

全局 flags：

| Flag | 默认 | 说明 |
|------|------|------|
| `--quiet` | false | 只输出错误。 |
| `--verbose` | false | 输出调试日志，可重复。 |
| `--json` | false | 对支持的命令输出 JSON。 |
| `--help` | false | 显示帮助。 |
| `--version` | false | 显示版本。 |

日志规则：

- 普通用户信息输出到 stdout。
- warning/error 输出到 stderr。
- `--json` 模式 stdout 必须只输出 JSON。
- `RUST_LOG` 可控制 `tracing` 级别。

## 命令

### `odl init [dir]`

创建 workspace。

参数：

| 参数 | 必填 | 默认 | 说明 |
|------|------|------|------|
| `dir` | 否 | `.` | workspace 目录。 |
| `--name <name>` | 否 | 目录名 | manifest name。 |
| `--force` | 否 | false | 允许在已有目录中补齐结构。 |

行为：

- 创建目录、`manifest.json`、`artifacts/`、可选 `skills/`。
- 如果目录已有 manifest 且不是 workspace，报错。
- 默认不覆盖已有文件。

### `odl new <kind> <dir>`

创建 artifact。

参数：

| 参数 | 必填 | 默认 | 说明 |
|------|------|------|------|
| `kind` | 是 | N/A | `html`、`docs`、`slides`。 |
| `dir` | 是 | N/A | artifact 目录。 |
| `--title <title>` | 否 | 目录名 | artifact 标题。 |
| `--brief <brief>` | 否 | `editorial` | design kernel visual brief。 |
| `--embed-css` | 否 | false | 将 `od-design.css` inline 到主文件。 |
| `--force` | 否 | false | 允许在空目录或已有 artifact 中补齐缺失文件。 |

行为：

- 写 `manifest.json`。
- 写对应主文件。
- 写 `handoff.md`。
- 默认写 `assets/od-design.css`。
- 不启动预览。

### `odl preview <dir>`

打开 artifact 预览。

参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `dir` | 必填 | artifact 目录。 |
| `--external-browser` | false | 使用系统浏览器 fallback。 |
| `--no-watch` | false | 禁用 watcher。 |
| `--devtools` | false | 平台支持时打开 devtools。 |

行为见 [preview.md](preview.md)。

### `odl handoff <dir>`

生成或刷新 `handoff.md`。

参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `dir` | 必填 | artifact 目录。 |
| `--stdout` | false | 不写文件，只输出到 stdout。 |
| `--agent <name>` | `generic` | `generic`、`opencode`、`claude-code`、`codex`。 |

### `odl export <dir> --format <format>`

导出 artifact。实现位于 `od-core::export`，CLI 与 MCP 共用。

参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `dir` | 必填 | artifact 目录。 |
| `--format` | 必填 | `html`、`md`、`zip`、`pdf`。 |
| `--out` | 按格式默认 | 输出路径（目录或文件）。 |

行为摘要见 [export.md](export.md)。`md` 仅支持 docs；`pdf` 依赖本机 Chrome/Edge headless（可用 `ODL_PDF_BROWSER` 覆盖）。

### `odl skill [show <name>]`

列出可用 skill，或输出某个 skill 的正文。

参数：

| 参数 | 默认 | 说明 |
|------|------|------|
| `show <name>` | 无 | 输出指定 skill 的 `SKILL.md` 正文（front matter 之后）。 |
| `--json` | false | 列表或 show 输出 JSON。也可使用全局 `--json`。 |

行为：

- 无 action 时列出 `name | mode | description`。
- `show` 输出 skill 正文；JSON 模式包含 `name`、`mode`、`description`、`body`、`root`。
- skill 发现复用 `od-core::skill::discover`，工作区同名 skill 覆盖内置 skill。

### `odl mcp`

启动 MCP stdio server（产品主入口的本地进程形态）。

行为：

- 调用 `od_mcp::serve_stdio()`，在 stdin/stdout 上提供 JSON-RPC（`Content-Length` framing）。
- 支持 `initialize`、`ping`、`tools/list`、`tools/call`。
- `tools/call` 已接 `artifact_create` / `artifact_preview` / `artifact_handoff` / `artifact_export`。
- 详情见 [mcp.md](mcp.md)。

## JSON 输出

`--json` 成功示例：

```json
{
  "ok": true,
  "artifact": {
    "kind": "html",
    "root": "D:/work/demo",
    "primaryFile": "index.html"
  }
}
```

错误示例：

```json
{
  "ok": false,
  "error": {
    "code": "primary_file_missing",
    "message": "Primary file index.html does not exist",
    "path": "D:/work/demo/index.html"
  }
}
```

## 退出码

| Code | 含义 |
|------|------|
| 0 | 成功。 |
| 1 | 一般错误。 |
| 2 | 参数错误。 |
| 3 | artifact/workspace 无效。 |
| 4 | 预览启动失败。 |
| 5 | 导出失败。 |
| 10 | 功能未实现。 |

## 测试

- `odl --help` 包含所有 M1 命令。
- `odl init tmp` 创建 workspace manifest。
- `odl new html tmp/artifact` 创建 `index.html`、`manifest.json`、`handoff.md`、`assets/od-design.css`。
- `--json` 模式 stdout 可被 `serde_json` 解析。
- 参数错误返回退出码 2。
- `odl skill` 输出内置 skill，`odl skill show html-page` 输出正文。
- `odl export --format zip|html` 写出可解压/可离线打开的产物；未知格式返回 `format_unsupported`。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
| 2026-07-08 | 对齐当前 CLI：preview/handoff/skill 已实现，export 仍为 M3 占位。 |
| 2026-07-09 | 补充 `odl mcp` 命令契约，与现有 stdio server 对齐。 |
| 2026-07-09 | `odl export` 接入：html / md / zip / pdf（本机浏览器）。 |
