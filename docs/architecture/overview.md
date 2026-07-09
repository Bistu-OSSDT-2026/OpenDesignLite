# 架构总览

## 核心决策

**MCP 优先，Agent 驱动。** 主产品面是 MCP 工具面：编码 Agent 经 MCP 驱动 `od-core`，`artifact_preview` 自动拉起常驻、实时刷新的预览窗口。CLI 是脚本化辅助，与 MCP 复用同一内核。不做独立壳层 app，也不做编辑器专用插件。

**Design kernel 极轻且框架无关。** `od-core` 表达 token、layout primitive、component recipe 与 pattern guideline，不内置 React、Tailwind、Radix、shadcn/ui 或 Web Components runtime。

详见 [ADR 0001](../decisions/0001-binary-shell-first.md) 与 [ADR 0002](../decisions/0002-lightweight-design-kernel.md)。

## 设计原则

| 原则 | 含义 |
|------|------|
| 本地优先 | 默认无网络；产物在磁盘 |
| 文件即接口 | 人类与 Agent 都能直接读写 |
| 内核独立 | `od-core` 不依赖任何编辑器 SDK |
| Agent 即界面 | 交互发生在编码 Agent 里，经 MCP 驱动内核 |
| 预览统一 | 一种 WebView 实现，多种产物类型 |
| 技能即目录 | `SKILL.md` + 可选模板，无注册表 |
| 设计内核极轻 | `--od-*` token + recipe，不绑定 UI 框架 |
| 小步验证 | 先跑通闭环，再扩生态 |

## 系统分层

```text
              编码 Agent（外部，经 MCP 连接）
                             │
┌────────────────────────────v─────────────────────────────┐
│  Surfaces（入口层）                                       │
│  od-mcp（主入口）      od-cli（脚本化辅助）              │
└────────────────────────────┬─────────────────────────────┘
                             │ 调用
┌────────────────────────────v─────────────────────────────┐
│  Kernel（内核） crates/od-core                             │
│  Artifact · Workspace · Skill · Design Kernel · Run      │
└────────────┬───────────────────────────────┬───────────────┘
             │                               │
┌────────────v──────────┐         ┌──────────v───────────────┐
│  Skills + Templates   │         │  Preview / Export        │
│  skills/  templates/  │         │  crates/od-preview       │
└────────────┬──────────┘         └──────────┬───────────────┘
             │                               │
             └───────────────┬───────────────┘
                             v
                    磁盘上的纯文件产物
```

## Crate 职责

| Crate | 职责 | 状态 |
|-------|------|------|
| `od-core` | 产物类型、工作区路径、manifest 路径、领域原语 | 脚手架 |
| `od-mcp` | MCP tool 注册与调用转发到内核（主入口） | 占位 |
| `od-cli` | `odl` 子命令：`init` `new` `preview` `export` `handoff`（脚本化辅助） | 部分实现 |
| `od-preview` | WebView 预览、文件监视、Markdown 渲染边界 | 占位 |

**依赖方向**：`od-mcp` / `od-cli` → `od-core` → 标准库 + 最小第三方依赖。`od-preview` 被 MCP 与 CLI 共用，由 `artifact_preview` / `odl preview` 拉起。

## Design Kernel

Design kernel 是内核中的设计语言层（token + layout primitive + component recipe + pattern + visual brief），不是组件库，也不内置任何 UI runtime。它随 artifact 写出静态 CSS 与设计说明，让 HTML / Slides 默认更好看，同时保持产物可离线打开。

四层结构、class 命名、边界规则与 visual brief 的**唯一详细来源**是 [specs/design-kernel.md](../specs/design-kernel.md)（镜像 `crates/od-core/src/design/catalog.rs`），此处不再重复展开。决策背景见 [ADR 0002](../decisions/0002-lightweight-design-kernel.md)。

## 产物模型（Artifact）

产物是**目录**，不是数据库记录。

```text
<artifact-dir>/
  index.html | doc.md | slides.html   # 主文件，类型决定扩展名
  assets/                             # 可选静态资源
  manifest.json                       # 元数据（类型、版本、创建时间）
  handoff.md                          # Agent 交接说明
```

### 产物类型

| Slug | 枚举 | 主文件 |
|------|------|--------|
| `html` | `ArtifactKind::Html` | `index.html` |
| `docs` / `md` | `ArtifactKind::Markdown` | `doc.md` |
| `slides` | `ArtifactKind::Slides` | `slides.html` |

实现见 `crates/od-core/src/lib.rs`。完整字段规范待写：[specs/artifact-workspace.md](../specs/README.md#artifact-workspace)。

### 工作区（Workspace）

用户级容器，可含多个产物：

```text
<workspace>/
  manifest.json       # schemaVersion, name
  artifacts/          # 产物目录集合
  skills/             # 工作区覆盖技能（可选）
```

`odl init <dir>` 创建上述结构。

## 技能模型（Skill）

技能是**目录** + `SKILL.md`：

```text
skills/html-page/
  SKILL.md            # front matter + 指令正文
  templates/          # 可选
```

Front matter 最小字段：`name`、`mode`、`description`。完整协议待写：[specs/built-in-skills.md](../specs/README.md#built-in-skills)。

内置技能：

| 目录 | 模式 | 产物 |
|------|------|------|
| `html-page` | html | `index.html` |
| `docs-polish` | docs | `doc.md` |
| `slides-html` | slides | `slides.html` |

## 预览管线

```text
odl preview <dir>
    → 检测主文件（index.html | slides.html | doc.md）
    → od-preview 打开系统 WebView
    → 文件系统 watcher 触发刷新
    → 错误时展示内联错误页（缺文件、加载失败、资源 404）
```

| 类型 | 渲染方式 |
|------|----------|
| HTML / Slides | 系统 WebView，`file://` 加载 |
| Markdown | 原生渲染 或 转静态 HTML 后 WebView |
| PDF（M4） | WebView 打印/导出 |

**明确不做**：为产物启动 dev server；产物应自包含可离线打开。

## Agent 集成三层

| 层级 | 机制 | 适用场景 |
|------|------|----------|
| **L3 MCP（主路径）** | tool: create / preview / export / handoff | opencode、Codex、Claude Code、Cursor 等 MCP 客户端 |
| L2 CLI（脚本化辅助） | `odl new` `preview` `export` `handoff` | 脚本、自动化、无 MCP 场景 |
| L1 文件（兜底） | `handoff.md` + 产物目录 | 任意 Agent，零集成 |

MCP 是产品的主交付（M2）。M1 先跑通本地产物+预览闭环（L1+L2 的基础），MCP 在其上暴露同一套能力。内置模型调用在 M3，非 MVP 阻塞项。

## CLI 命令（目标契约）

当前已实现标 ✓。

| 命令 | 说明 | 状态 |
|------|------|------|
| `odl init [dir]` | 创建工作区 | ✓ |
| `odl new <kind> <dir>` | 创建产物 + starter + handoff | ✓ |
| `odl preview <dir>` | 打开预览 | 占位 |
| `odl export <dir> --format …` | 导出 | 未实现 |
| `odl handoff <dir>` | 输出/刷新 handoff | 未实现 |

完整 flags 与退出码：[specs/cli.md](../specs/README.md#cli)。

## MCP 工具（目标契约）

| Tool | 对应 CLI | 里程碑 |
|------|----------|--------|
| `artifact_create` | `new` | M2 |
| `artifact_preview` | `preview` | M2 |
| `artifact_export` | `export` | M2 |
| `artifact_handoff` | `handoff` | M2 |

详情：[specs/mcp.md](../specs/README.md#mcp)。

## 为何用 MCP 优先，而非壳层或插件优先

- 用户已经在用带 MCP 能力的编码 Agent，Agent 本身就是交互界面——无需自研并维护一套对话/预览 UI。
- 编辑器插件 API（Cursor / VS Code / Zed / Codex）各不相同，预览面板行为不一致，核心产品会被第一个插件绑架。
- MCP 是跨 Agent 的稳定契约：配置一次即用，产物存储与预览行为一致。

预览只是一个由 `artifact_preview` 拉起的常驻窗口，不获得 MCP/shell/文件系统 IPC。

## 技术选型（开放）

| 项 | 候选 | 决定时机 |
|----|------|----------|
| 预览 WebView | Wry + Tao（M1 已采用），其他 Rust WebView 备选 | M1 preview spike |
| Markdown 渲染 | 内置 crate vs 转 HTML | M1 |
| 模型调用 | v1 仅编排外部 Agent vs 内置 BYOK | M3 前 ADR |

## 相关文档

- [boundaries.md](boundaries.md) — 模块边界与变更规则
- [product/prd.md](../product/prd.md) — 产品范围
- [research/design-system-kernel.md](../research/design-system-kernel.md) — 轻量设计内核调研
- [specs/](../specs/) — 实现级契约（待编写）
