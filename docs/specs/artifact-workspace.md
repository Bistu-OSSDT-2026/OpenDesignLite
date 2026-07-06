# Artifact Workspace

**状态**：草案  
**里程碑**：M1  
**实现位置**：`crates/od-core`

## 目的

定义 Open Design Lite 的磁盘文件模型：工作区、产物目录、manifest、主文件、assets、handoff 与 design kernel 资源的布局。所有入口层必须复用该规则，不能各自推断路径。

## 范围

- 包含工作区目录布局。
- 包含 artifact 目录布局。
- 包含 `manifest.json` 最小 schema。
- 包含主文件检测顺序与 artifact kind 映射。
- 包含 design kernel 静态资源的推荐位置。
- 不包含 CLI flags，见 [cli.md](cli.md)。
- 不包含预览加载细节，见 [preview.md](preview.md)。

## 术语

| 术语 | 含义 |
|------|------|
| workspace | 用户级容器，可包含多个 artifact。 |
| artifact | 一个可预览、可交接、可导出的产物目录。 |
| primary file | artifact 的主文件，例如 `index.html`。 |
| handoff | 面向外部 Agent 的交接说明。 |
| design kernel | `--od-*` token、静态 CSS recipe、visual brief。 |

## 工作区布局

```text
<workspace>/
  manifest.json
  artifacts/
  skills/               # 可选，工作区覆盖技能
```

`manifest.json` 最小示例：

```json
{
  "schemaVersion": 1,
  "type": "workspace",
  "name": "my-workspace",
  "createdBy": "odl",
  "createdAt": "2026-07-01T00:00:00Z"
}
```

字段规则：

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `schemaVersion` | number | 是 | M1 固定为 `1`。 |
| `type` | string | 是 | 工作区固定为 `workspace`。 |
| `name` | string | 是 | 默认取目录名。 |
| `createdBy` | string | 是 | 固定为 `odl`。 |
| `createdAt` | string | 是 | UTC ISO 8601。 |

## Artifact 布局

HTML：

```text
<artifact>/
  manifest.json
  index.html
  handoff.md
  assets/
    od-design.css       # M1 推荐，允许 inline 模式省略
```

Markdown：

```text
<artifact>/
  manifest.json
  doc.md
  handoff.md
  assets/
    od-design.css       # 预览包装可使用
```

Slides：

```text
<artifact>/
  manifest.json
  slides.html
  handoff.md
  assets/
    od-design.css
```

Artifact `manifest.json` 最小示例：

```json
{
  "schemaVersion": 1,
  "type": "artifact",
  "kind": "html",
  "title": "Landing Page Draft",
  "primaryFile": "index.html",
  "createdBy": "odl",
  "createdAt": "2026-07-01T00:00:00Z",
  "design": {
    "kernelVersion": 1,
    "stylesheet": "assets/od-design.css",
    "visualBrief": "editorial"
  }
}
```

字段规则：

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `schemaVersion` | number | 是 | M1 固定为 `1`。 |
| `type` | string | 是 | artifact 固定为 `artifact`。 |
| `kind` | string | 是 | `html`、`docs`、`slides`。 |
| `title` | string | 是 | 人类可读标题。 |
| `primaryFile` | string | 是 | 必须与 kind 匹配。 |
| `createdBy` | string | 是 | 固定为 `odl`。 |
| `createdAt` | string | 是 | UTC ISO 8601。 |
| `design` | object | 否 | design kernel 元数据。 |

`design` 字段规则：

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `kernelVersion` | number | 是 | M1 固定为 `1`。 |
| `stylesheet` | string | 否 | 相对 artifact root 的 CSS 路径。 |
| `visualBrief` | string | 否 | `editorial`、`studio`、`workbench` 等。 |

## Kind 与主文件

| Kind | slug 输入 | 主文件 | 预览方式 |
|------|-----------|--------|----------|
| HTML | `html`、`html-page` | `index.html` | WebView 直接加载 |
| Docs | `docs`、`markdown`、`md` | `doc.md` | Markdown 渲染后 WebView 加载 |
| Slides | `slides`、`ppt`、`deck` | `slides.html` | WebView 直接加载 |

主文件检测顺序固定为：

```text
index.html -> slides.html -> doc.md
```

如果 `manifest.json` 存在，优先使用 `manifest.primaryFile`，但必须验证文件存在且 kind 匹配。若 manifest 缺失，允许按检测顺序推断 artifact kind，并在 CLI 输出 warning。

## 路径规则

- 所有 manifest 中的路径必须相对 artifact root。
- manifest 路径分隔符使用 `/`。
- 不允许 `..` 越出 artifact root。
- `assets/` 为普通静态资源目录，不应存数据库或缓存。
- `.log/`、`.git/`、系统临时文件不属于 artifact 语义内容。

## 错误

| 场景 | 错误码建议 | 行为 |
|------|------------|------|
| workspace manifest 缺失 | `workspace_not_found` | `odl init` 可创建；其他命令报错。 |
| artifact 主文件缺失 | `primary_file_missing` | preview 显示错误页；CLI 退出非 0。 |
| manifest JSON 无效 | `manifest_invalid` | 显示解析错误和路径。 |
| kind 未知 | `artifact_kind_unknown` | 列出支持值。 |
| 路径越界 | `path_escape` | 拒绝操作。 |

## 测试

- 创建三类 artifact，确认主文件名正确。
- manifest 缺失时按检测顺序推断。
- manifest 指向不存在文件时报错。
- Windows 路径写入 manifest 时统一为 `/`。
- `assets/od-design.css` 相对路径可被 HTML 使用。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-01 | 初版草案。 |
