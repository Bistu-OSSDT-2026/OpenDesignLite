# Open Design Lite

> **通过 MCP 接入编码 Agent 的本地优先设计助手（`odl`）。**
>
> 用一句话描述你想要的效果——Agent 调用 Open Design Lite 的工具创建 HTML 页面、Markdown 文档或轻量 HTML 幻灯片，本地预览窗口会自动弹出，并在 Agent 迭代时实时刷新。

设计默认保持轻量、框架无关：**CSS 变量、静态 recipe、纯文件**，而不是捆绑 UI 运行时。无 React、无 Tailwind、无 shadcn——只有可移植、可检视的磁盘产物。

**English README**: [../README.md](../README.md)

---

## 目录

- [功能特性](#功能特性)
- [工作原理](#工作原理)
- [前置条件](#前置条件)
- [安装](#安装)
- [使用方式](#使用方式)
  - [CLI 模式（脚本化）](#cli-模式脚本化)
  - [MCP 模式（Agent 驱动）](#mcp-模式agent-驱动)
  - [Agent 配置](#agent-配置)
- [MCP 工具参考](#mcp-工具参考)
- [内置技能](#内置技能)
- [项目结构](#项目结构)
- [文档](#文档)
- [路线图](#路线图)
- [贡献](#贡献)
- [许可证](#许可证)

---

## 功能特性

- 🔧 **MCP 优先** — Agent 通过 MCP 工具驱动设计；CLI 是同一内核上的脚本化辅助。
- 🖥️ **本地优先** — 无云端、无服务器、无需注册。一切都在本机运行。
- 🪟 **实时预览** — 原生 WebView 窗口自动打开，每次变更后实时重载。
- 🎨 **框架无关** — 设计 token（`--od-*` CSS 变量）与静态 `.od-*` class recipe，而非运行时。
- 📄 **多种产物类型** — HTML 页面、Markdown 文档、轻量 HTML 幻灯片。
- 📦 **随时导出** — 导出为自包含 HTML、Markdown、ZIP 或 PDF。
- 🤝 **便于交接** — 每个 artifact 包含 `handoff.md`，让人与 Agent 保持同步。
- 🧩 **可扩展技能** — 内置技能为文件系统目录，内含可读的 `SKILL.md`。

---

## 工作原理

```
你（一句话） → 编码 Agent → MCP 工具 → od-core → 磁盘产物 → 实时预览
```

1. **配置** 编码 Agent 中的 `open-design-lite` MCP 服务器（一次性）。
2. **描述** 用自然语言说明你想要什么。
3. **Agent 调用** MCP 工具（`artifact_create`、`artifact_preview` 等）构建产物。
4. **预览自动打开** 于原生窗口，Agent 迭代时实时重载。
5. **继续对话** 微调；完成后导出。

无需单独安装 app 或 chat/preview 壳层——**Agent 就是你的 UI，预览只是一个窗口**。

---

## 前置条件

- **Windows** 为主要目标平台；macOS / Linux 已纳入 CI 构建，Linux 预览可能需额外 WebKitGTK 依赖（见 [releases/v0.1.0.md](releases/v0.1.0.md)）
- 兼容的编码 Agent（见 [Agent 配置](#agent-配置)）
- [Rust](https://rustup.rs/) 工具链——**仅从源码构建（贡献者）时需要**

---

## 安装

### 一键安装（推荐）

两条命令完成二进制下载与 Agent 接入：

```powershell
# Windows（PowerShell）
irm https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.ps1 | iex
odl setup
```

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.sh | sh
odl setup
```

安装位置：`%LOCALAPPDATA%\OpenDesignLite\bin\odl.exe`（Windows）/ `~/.local/bin/odl`（Unix）。`odl setup` 自动检测已安装的编码 Agent 并写入 MCP 配置——无需手工编辑 JSON，之后重启 Agent 即可看到四个 `artifact_*` 工具。检测规则与参数（`--agent` / `--dry-run` / `--global` / `--force`）见 [specs/setup.md](specs/setup.md)。

### 从源码构建（贡献者）

```powershell
git clone https://github.com/Bistu-OSSDT-2026/OpenDesignLite.git
cd OpenDesignLite
cargo build --release

# （Windows）若 link.exe 报错，可使用辅助脚本：
powershell -File scripts/build.ps1 build --release
```

Release 二进制位于 `target/release/odl.exe`（Windows）或 `target/release/odl`（Unix）。用 `target/release/odl --help` 验证，应列出 `init`、`new`、`preview`、`export`、`handoff`、`skill`、`setup`、`mcp` 子命令。

---

## 使用方式

### CLI 模式（脚本化）

`odl` CLI 可在终端创建并预览产物：

```powershell
# 初始化工作区
odl init my-workspace

# 创建 HTML 产物
odl new html my-workspace/hello

# 创建 Markdown 文档
odl new docs my-workspace/readme

# 创建 HTML 幻灯片
odl new slides my-workspace/deck

# 预览产物（打开原生实时重载窗口）
odl preview my-workspace/hello

# 导出产物
odl export my-workspace/hello --format html   # 自包含 HTML 目录
odl export my-workspace/readme --format md    # Markdown（仅 docs）
odl export my-workspace --format zip          # ZIP 归档
odl export my-workspace/hello --format pdf    # PDF（需本机 Chrome/Edge；slides 固定 16:9）
```

（贡献者从源码运行时可用 `cargo run -p od-cli --` 替代 `odl`。）

### MCP 模式（Agent 驱动）

为 Agent 集成启动 MCP 服务器：

```powershell
odl mcp
```

这会在 stdio 上启动 JSON-RPC 服务器，供编码 Agent 连接。**通常不需要手动运行**——`odl setup` 写好配置后由 Agent 按需拉起。

### Agent 配置

**推荐直接运行 `odl setup`**——自动检测已安装 Agent 并写入下列配置。以下模板即 `odl setup` 生成的内容，仅在故障排查时手工编辑；`<ODL>` 替换为已安装二进制的绝对路径（如 `C:/Users/me/AppData/Local/OpenDesignLite/bin/odl.exe` 或 `~/.local/bin/odl`）。

<details>
<summary><strong>Claude Code</strong></summary>

添加到项目 `.mcp.json`（或用户级 `~/.claude.json`）：

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "<ODL>",
      "args": ["mcp"]
    }
  }
}
```
</details>

<details>
<summary><strong>Codex（OpenCode）</strong></summary>

添加到 `opencode.json`：

```json
{
  "mcp": {
    "open-design-lite": {
      "command": "<ODL>",
      "args": ["mcp"]
    }
  }
}
```
</details>

<details>
<summary><strong>Cursor</strong></summary>

添加到 Cursor MCP 配置（项目 `.cursor/mcp.json` 或 `~/.cursor/mcp.json`）：

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "<ODL>",
      "args": ["mcp"]
    }
  }
}
```
</details>

<details>
<summary><strong>Zed</strong></summary>

添加到 Zed 的 `settings.json`：

```json
{
  "context_servers": {
    "open-design-lite": {
      "command": {
        "path": "<ODL>",
        "args": ["mcp"]
      }
    }
  }
}
```
</details>

<details>
<summary><strong>ZCode / 其他 MCP 兼容 Agent</strong></summary>

配置为 stdio MCP 服务器（`.zcode/mcp.json` 或等价位置）：

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "<ODL>",
      "args": ["mcp"]
    }
  }
}
```

配置完成后重启 Agent。应能看到 `artifact_create`、`artifact_preview`、`artifact_handoff`、`artifact_export` 工具。
</details>

> **不要在 Agent 配置里使用 `cargo run`**：首次调用会触发编译（慢），且编译输出可能污染 stdio 的 JSON-RPC 流。若 MCP 工具时断时续，先检查是否残留 `cargo run` 配置（`odl setup --force` 覆盖）；`scripts/mcp_proxy.py` 保留为隔离 stdio 的调试代理。

更完整的 Agent 自动安装说明见 [AGENT_SETUP.md](../AGENT_SETUP.md)。

---

## MCP 工具参考

| 工具 | 说明 |
|------|------|
| `artifact_create` | 在工作区创建新产物（HTML 页面、Markdown 文档或幻灯片）。 |
| `artifact_preview` | 打开或刷新产物的实时重载预览窗口。 |
| `artifact_handoff` | 生成或读取产物的 `handoff.md`——保持 Agent 与人的同步。 |
| `artifact_export` | 导出为便携格式：`html`、`md`、`zip` 或 `pdf`。 |

**预览默认值**：`externalBrowser: false`、`watch: true`——使用 MCP 托管的 `odl preview` 窗口并实时重载。不要自行打开系统浏览器；预览由技能处理（见 `skills/preview-via-mcp/SKILL.md`）。

---

## 内置技能

技能为文件系统目录，内含 Agent 可发现并使用的可读 `SKILL.md`：

| 技能 | 模式 | 说明 |
|------|------|------|
| `html-page` | `html` | 创建带内联 CSS 的自包含 HTML 页面（落地页、文章、微站）。 |
| `slides-html` | `slides` | 创建轻量 HTML 演示幻灯片（16:9，单文件）。 |
| `docs-polish` | `docs` | 润色与格式化 Markdown 文档。 |
| `preview-via-mcp` | `workflow` | 指导 Agent 通过 MCP 预览（切勿打开系统浏览器）。 |

**视觉 brief**（`skills/visual-briefs.md`）定义三种被技能引用的共享美学：

- **Editorial** — 温暖纸质感杂志风格（文档、落地页、叙事内容）。
- **Studio** — 画廊墙 / 设计评审板（作品集、展示、幻灯片）。
- **Workbench** — 清晰信息层级（工具、仪表盘、工作区）。

---

## 项目结构

```text
OpenDesignLite/
├── crates/
│   ├── od-core/        # 内核：产物、设计 token、工作区、技能
│   ├── od-cli/         # CLI：init、new、preview、export、handoff、skill、mcp
│   ├── od-mcp/         # MCP 实现库（由 od-cli 的 `mcp` 子命令调用）
│   └── od-preview/     # 原生 WebView 预览窗口与实时重载
├── docs/               # 文档（中文为主）：架构、规范、ADR、路线图
├── skills/             # 内置 Agent 技能（每目录一个 SKILL.md）
├── templates/          # 各产物类型的启动模板
├── scripts/            # 构建辅助（Windows MSVC、MCP 代理）
├── Cargo.toml          # Rust workspace 清单
└── AGENTS.md           # 在本仓库工作的编码 Agent 指引
```

---

## 文档

详细文档位于 [`docs/`](.) 目录（以中文为主）：

| 文档 | 用途 |
|------|------|
| [docs/README.md](README.md) | 本文档（中文 README） |
| [product/prd.md](product/prd.md) | 产品范围、排除项、MVP 定义 |
| [product/roadmap.md](product/roadmap.md) | 里程碑 M0–M3 与发布计划 |
| [architecture/overview.md](architecture/overview.md) | 系统分层、模块边界、数据模型 |
| [decisions/](decisions/) | 架构决策记录（ADR） |
| [specs/](specs/) | 各模块实现规范与接口契约 |
| [releases/](releases/) | 发布说明与已知限制 |
| [team/plan.md](team/plan.md) | 团队分工与接口变更规则 |

### 推荐阅读顺序

| 文档 | 用途 |
|------|------|
| [product/prd.md](product/prd.md) | 做什么、不做什么、MVP 模式 |
| [architecture/overview.md](architecture/overview.md) | 系统分层、模块边界、数据模型 |
| [product/roadmap.md](product/roadmap.md) | 里程碑 M0–M3 |
| [releases/](releases/) | 发布说明与已知限制 |
| [decisions/](decisions/) | 已采纳的架构决策（ADR） |
| [specs/](specs/) | 实现规范与接口契约 |
| [team/plan.md](team/plan.md) | 5 人分工与接口变更规则 |

---

## 路线图

| 里程碑 | 状态 | 重点 |
|--------|------|------|
| **M0** – 仓库契约 | ✅ 已完成 | 文档、脚手架、最小 CLI、技能目录、启动模板 |
| **M1** – 本地产物闭环 | ✅ 基本完成 | `init`、`new html\|docs\|slides`、原生预览与自动刷新、handoff |
| **M2** – Agent 桥接（MCP） | 🚧 收尾中 | MCP 服务器已可启动；`odl setup` + 安装脚本与稳定性加固进行中 |
| **M3** – 导出 | ✅ 基本完成 | HTML/MD/ZIP/PDF 已接入（slides PDF 固定 16:9）；HTML 单文件内联（`--single-file`）计划中 |
| **M4** – BYOK design agent | 📝 规划中 | 预览壳内置聊天面板 + 自带 API key；v1 仅预留 UI 占位 |

详见 [product/roadmap.md](product/roadmap.md) 与 [releases/v0.1.0.md](releases/v0.1.0.md)。

---

## 贡献

欢迎贡献！请先阅读 [AGENTS.md](../AGENTS.md) 中的工程规则与产品原则。

1. **Fork** 本仓库
2. **创建** 功能分支（`git checkout -b feature/amazing-feature`）
3. **提交** 变更（`git commit -m 'Add amazing feature'`）
4. **推送** 到分支（`git push origin feature/amazing-feature`）
5. **发起** Pull Request

除非产品目标需要，否则保持变更小而快、朴实无华。未经 ADR 不要引入重型框架。

---

## 许可证

本项目采用 **MIT License** — 详见 [LICENSE](../LICENSE)。
