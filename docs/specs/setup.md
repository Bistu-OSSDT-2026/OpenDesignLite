# Setup（安装与 Agent 配置）

**状态**：草案（2026-07-14 新增，随「简单好部署」定位落地）  
**里程碑**：M2 收尾  
**实现位置**：`crates/od-cli/src/commands/setup.rs`（`odl setup`）、`scripts/install.sh`、`scripts/install.ps1`

## 目的

把「clone + cargo build + 手工编辑各 Agent 的 MCP JSON + 重启」的多步流程收敛为两步：

```text
1. 安装脚本下载 release 二进制到固定位置
2. odl setup 自动检测 Agent 并写入 MCP 配置
```

本 spec 是 `odl setup` 检测规则、各 Agent 配置文件形状、合并策略与安装脚本行为的**唯一事实来源**；README / AGENT_SETUP.md 中的配置模板均由此派生。

## 安装脚本

| 平台 | 入口 | 安装路径 |
|------|------|----------|
| Windows | `powershell -c "irm <raw-url>/scripts/install.ps1 | iex"` | `%LOCALAPPDATA%\OpenDesignLite\bin\odl.exe` |
| macOS / Linux | `curl -fsSL <raw-url>/scripts/install.sh | sh` | `~/.local/bin/odl` |

行为：

- 按平台/架构选择 GitHub Release 资产，命名固定（与 CI 一致）：`odl-linux-x64`、`odl-macos-x64`、`odl-macos-arm64`、`odl-windows-x64.exe`。
- 下载到安装路径，Unix 下 `chmod +x`。
- 不修改用户 shell rc / 注册表；PATH 未包含安装目录时**打印指引**由用户自行添加。
- 结束时提示运行 `odl setup`。
- 源码构建（`cargo build --release`）保留为贡献者路径，不再是用户默认路径。

## `odl setup` 检测规则

按 Agent 检测配置文件位置；`--global` 写用户级，默认写项目级（当前目录）：

| Agent | `--agent` 名 | 项目级 | 用户级（`--global`） | JSON 形状 |
|-------|--------------|--------|----------------------|-----------|
| Claude Code | `claude-code` | `.mcp.json` | `~/.claude.json`（`mcpServers` 顶层键） | A |
| Cursor | `cursor` | `.cursor/mcp.json` | `~/.cursor/mcp.json` | A |
| OpenCode / Codex | `opencode` | `opencode.json` | `~/.config/opencode/opencode.json` | B |
| Zed | `zed` | `.zed/settings.json` | `~/.config/zed/settings.json`（Windows：`%APPDATA%\Zed\settings.json`） | C |
| ZCode / 其他 | `zcode` | `.zcode/mcp.json` | `~/.zcode/mcp.json` | A |

- 不带 `--agent` 时：检测上述位置（含其父目录已存在即视为「装了该 Agent」），对每个检测到的 Agent 写入；一个都没检测到 → `agent_not_detected`。
- 带 `--agent` 时：只处理指定 Agent；对应位置不存在则**创建**（项目级）或报 `agent_not_detected`（用户级但 Agent 未安装）。

### JSON 形状

写入的 server 条目统一命名 `open-design-lite`，`command` 为已安装 `odl` 二进制的**绝对路径**（`current_exe()` 解析，不使用 `cargo run`）：

形状 A（Claude Code / Cursor / ZCode）：

```json
{
  "mcpServers": {
    "open-design-lite": {
      "command": "C:/Users/me/AppData/Local/OpenDesignLite/bin/odl.exe",
      "args": ["mcp"]
    }
  }
}
```

形状 B（OpenCode，字段名 `mcp` + `workdir`）：

```json
{
  "mcp": {
    "open-design-lite": {
      "command": "/home/me/.local/bin/odl",
      "args": ["mcp"]
    }
  }
}
```

形状 C（Zed，`context_servers` 嵌套 `command` 对象）：

```json
{
  "context_servers": {
    "open-design-lite": {
      "command": {
        "path": "/home/me/.local/bin/odl",
        "args": ["mcp"]
      }
    }
  }
}
```

> 二进制自带内核与技能，`odl mcp` 不依赖工作目录，因此默认不写 `cwd`/`workdir`/`work_dir` 字段。

## 合并策略

- **深合并，最小触碰**：只新增/更新 `open-design-lite` 这一个 key；同文件内其余 key（其他 MCP server、Agent 自身设置）原样保留，包括未知字段。
- **幂等**：目标条目已存在且内容相同 → 跳过（输出「已配置」）；重复运行结果一致。
- **冲突**：目标条目已存在但内容不同（典型：指向旧的 `cargo run` 模板）→ 默认不覆盖并提示；`--force` 覆盖。
- **`--dry-run`**：打印将写入的文件路径与合并后的 JSON，不落盘。
- 写入前保留原文件格式合法性：整文件解析 → 修改内存对象 → 序列化写回（2 空格缩进）。

## 错误

| Code | 场景 |
|------|------|
| `agent_not_detected` | 未检测到任何 Agent，或 `--agent` 指定的 Agent 未安装。 |
| `config_parse_failed` | 已存在的目标配置文件不是合法 JSON。含注释的 JSONC（如 Zed settings.json）v1 不做「剥注释重写」——那会毁掉用户注释——直接报错并提示手工添加条目。 |
| `config_write_failed` | 目标文件不可写。 |

## 与仓库内 `.mcp.json` / `CLAUDE.md` 的关系

- 仓库根目录的 `.mcp.json` + `CLAUDE.md` 只服务于「在 OpenDesignLite 仓库自身里工作/演示（dogfood）」的场景。
- 用户在**自己项目**中使用 odl，一律通过 `odl setup` 配置；两者不可互相替代。

## 测试

- mock 各 Agent 配置目录：`odl setup --agent cursor --dry-run` 输出正确形状且不落盘。
- 已配置其他 MCP server 的文件跑 setup 后，其他 server key 原样保留。
- 幂等：连续两次运行，第二次跳过且文件内容不变。
- 旧 `cargo run` 条目：默认提示不覆盖；`--force` 覆盖为二进制路径。
- 非法 JSON 目标文件 → `config_parse_failed`，原文件不被破坏。

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-07-14 | 初版：安装脚本 + `odl setup` 检测/形状/合并契约。 |
