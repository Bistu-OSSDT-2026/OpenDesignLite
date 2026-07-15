//! `odl setup`：检测编码 Agent 并写入 `open-design-lite` 的 MCP 配置，
//! 替代「手工编辑 JSON + 填绝对路径」的流程。
//!
//! 合并策略（setup.md）：JSON 深合并、只碰 `open-design-lite` 这一个 key、
//! 幂等；已存在但内容不同（典型：旧 `cargo run` 模板）默认不覆盖，
//! `--force` 覆盖。含注释的配置（如 Zed 的 JSONC）解析失败时报
//! `config_parse_failed`，不做「剥注释重写」——那会毁掉用户的注释。
//!
//! Spec: docs/specs/setup.md, cli.md（`odl setup`）

use serde_json::{json, Map, Value};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const SERVER_KEY: &str = "open-design-lite";

/// setup 专属错误（错误码对齐 setup.md）。
#[derive(Debug)]
pub enum SetupError {
    AgentNotDetected(String),
    ConfigParseFailed(String),
    ConfigWriteFailed(String),
}

impl SetupError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AgentNotDetected(_) => "agent_not_detected",
            Self::ConfigParseFailed(_) => "config_parse_failed",
            Self::ConfigWriteFailed(_) => "config_write_failed",
        }
    }
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentNotDetected(msg) => write!(f, "agent not detected: {msg}"),
            Self::ConfigParseFailed(msg) => write!(f, "config parse failed: {msg}"),
            Self::ConfigWriteFailed(msg) => write!(f, "config write failed: {msg}"),
        }
    }
}

impl std::error::Error for SetupError {}

/// 各 Agent 配置文件的 JSON 形状（setup.md「JSON 形状」）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigShape {
    /// `{"mcpServers": {name: {command, args}}}`（Claude Code / Cursor / ZCode）。
    McpServers,
    /// `{"mcp": {name: {command, args}}}`（OpenCode）。
    Mcp,
    /// `{"context_servers": {name: {command: {path, args}}}}`（Zed）。
    ContextServers,
}

impl ConfigShape {
    fn top_key(self) -> &'static str {
        match self {
            Self::McpServers => "mcpServers",
            Self::Mcp => "mcp",
            Self::ContextServers => "context_servers",
        }
    }
}

/// 支持的 Agent 清单（setup.md 检测规则表的代码化）。
pub struct AgentSpec {
    pub slug: &'static str,
    pub shape: ConfigShape,
    /// 项目级配置文件（相对当前目录）。
    pub project_config: &'static str,
}

pub const AGENTS: &[AgentSpec] = &[
    AgentSpec {
        slug: "claude-code",
        shape: ConfigShape::McpServers,
        project_config: ".mcp.json",
    },
    AgentSpec {
        slug: "cursor",
        shape: ConfigShape::McpServers,
        project_config: ".cursor/mcp.json",
    },
    AgentSpec {
        slug: "opencode",
        shape: ConfigShape::Mcp,
        project_config: "opencode.json",
    },
    AgentSpec {
        slug: "zed",
        shape: ConfigShape::ContextServers,
        project_config: ".zed/settings.json",
    },
    AgentSpec {
        slug: "zcode",
        shape: ConfigShape::McpServers,
        project_config: ".zcode/mcp.json",
    },
];

fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var_os("USERPROFILE").map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}

/// 用户级配置文件位置。
fn global_config(slug: &str) -> Option<PathBuf> {
    let home = home_dir()?;
    Some(match slug {
        "claude-code" => home.join(".claude.json"),
        "cursor" => home.join(".cursor").join("mcp.json"),
        "opencode" => home.join(".config").join("opencode").join("opencode.json"),
        "zed" => {
            #[cfg(windows)]
            {
                std::env::var_os("APPDATA")
                    .map(PathBuf::from)?
                    .join("Zed")
                    .join("settings.json")
            }
            #[cfg(not(windows))]
            {
                home.join(".config").join("zed").join("settings.json")
            }
        }
        "zcode" => home.join(".zcode").join("mcp.json"),
        _ => return None,
    })
}

/// 用户级检测标记：判断这台机器是否装了该 Agent（与写入 scope 无关）。
fn agent_installed(slug: &str) -> bool {
    let Some(home) = home_dir() else {
        return false;
    };
    match slug {
        "claude-code" => home.join(".claude").is_dir() || home.join(".claude.json").is_file(),
        "cursor" => home.join(".cursor").is_dir(),
        "opencode" => home.join(".config").join("opencode").is_dir(),
        "zed" => global_config("zed").map(|p| p.exists()).unwrap_or(false),
        "zcode" => home.join(".zcode").is_dir(),
        _ => false,
    }
}

/// 期望写入的 server 条目。`exe` 为 `odl` 二进制绝对路径（正斜杠）。
fn desired_entry(shape: ConfigShape, exe: &str) -> Value {
    match shape {
        ConfigShape::McpServers | ConfigShape::Mcp => json!({
            "command": exe,
            "args": ["mcp"],
        }),
        ConfigShape::ContextServers => json!({
            "command": {
                "path": exe,
                "args": ["mcp"],
            }
        }),
    }
}

/// 单个配置文件的合并结果。
#[derive(Debug, PartialEq, Eq)]
pub enum MergeOutcome {
    /// 已是期望内容，无需改动（幂等）。
    Unchanged,
    /// 需要写入的新文件内容。
    Updated(String),
    /// 条目已存在但内容不同；未加 `--force`，不覆盖。
    NeedsForce,
}

/// 深合并：只新增/更新 `open-design-lite` 条目，其余 key 原样保留。
/// `existing` 为现有文件内容（文件不存在传 `None`）。
pub fn merge_config(
    existing: Option<&str>,
    shape: ConfigShape,
    exe: &str,
    force: bool,
) -> Result<MergeOutcome, SetupError> {
    let mut root: Value = match existing {
        None => Value::Object(Map::new()),
        Some(text) if text.trim().is_empty() => Value::Object(Map::new()),
        Some(text) => serde_json::from_str(text).map_err(|e| {
            SetupError::ConfigParseFailed(format!(
                "existing config is not valid JSON ({e}); if it contains comments (JSONC), add the `{SERVER_KEY}` entry manually"
            ))
        })?,
    };
    let Value::Object(map) = &mut root else {
        return Err(SetupError::ConfigParseFailed(
            "existing config root is not a JSON object".to_string(),
        ));
    };

    let servers = map
        .entry(shape.top_key().to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let Value::Object(servers) = servers else {
        return Err(SetupError::ConfigParseFailed(format!(
            "`{}` is not a JSON object",
            shape.top_key()
        )));
    };

    let desired = desired_entry(shape, exe);
    match servers.get(SERVER_KEY) {
        Some(current) if *current == desired => return Ok(MergeOutcome::Unchanged),
        Some(_) if !force => return Ok(MergeOutcome::NeedsForce),
        _ => {}
    }
    servers.insert(SERVER_KEY.to_string(), desired);

    let mut text = serde_json::to_string_pretty(&root)
        .map_err(|e| SetupError::ConfigWriteFailed(e.to_string()))?;
    text.push('\n');
    Ok(MergeOutcome::Updated(text))
}

/// 单个 Agent 的处理结果（供 reporter 输出）。
pub struct SetupOutcome {
    pub agent: &'static str,
    pub path: PathBuf,
    pub status: SetupStatus,
    /// dry-run 时携带将写入的内容。
    pub preview: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SetupStatus {
    Written,
    AlreadyConfigured,
    WouldWrite,
    NeedsForce,
}

pub struct SetupOptions<'a> {
    pub agent: Option<&'a str>,
    pub dry_run: bool,
    pub global: bool,
    pub force: bool,
}

/// `odl` 自身二进制路径（正斜杠，配置里跨 Agent 可用）。
/// 与 od-mcp `odl_executable()` 同一手法：release 资产名不固定为 `odl`。
fn odl_exe() -> Result<String, SetupError> {
    let exe = std::env::current_exe()
        .map_err(|e| SetupError::ConfigWriteFailed(format!("cannot resolve odl binary: {e}")))?;
    Ok(exe.display().to_string().replace('\\', "/"))
}

pub fn run(options: &SetupOptions<'_>) -> Result<Vec<SetupOutcome>, SetupError> {
    let exe = odl_exe()?;
    let cwd = std::env::current_dir()
        .map_err(|e| SetupError::ConfigWriteFailed(format!("cannot resolve cwd: {e}")))?;

    let targets: Vec<&AgentSpec> = match options.agent {
        Some(slug) => {
            let spec = AGENTS.iter().find(|a| a.slug == slug).ok_or_else(|| {
                SetupError::AgentNotDetected(format!(
                    "unknown agent `{slug}`; expected one of: {}",
                    AGENTS
                        .iter()
                        .map(|a| a.slug)
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })?;
            // 显式指定 + 全局：Agent 必须真的装了，否则是写给空气。
            // 项目级则总是允许（把配置带进仓库是合法场景）。
            if options.global && !agent_installed(slug) {
                return Err(SetupError::AgentNotDetected(format!(
                    "`{slug}` does not look installed on this machine (missing its user config dir)"
                )));
            }
            vec![spec]
        }
        None => {
            let detected: Vec<&AgentSpec> =
                AGENTS.iter().filter(|a| agent_installed(a.slug)).collect();
            if detected.is_empty() {
                return Err(SetupError::AgentNotDetected(
                    "no supported coding agent found; rerun with --agent <name> to force one"
                        .to_string(),
                ));
            }
            detected
        }
    };

    let mut outcomes = Vec::new();
    for spec in targets {
        let path = if options.global {
            global_config(spec.slug).ok_or_else(|| {
                SetupError::ConfigWriteFailed(format!(
                    "cannot resolve global config path for `{}`",
                    spec.slug
                ))
            })?
        } else {
            cwd.join(spec.project_config)
        };
        outcomes.push(apply_one(spec, &path, &exe, options)?);
    }
    Ok(outcomes)
}

fn apply_one(
    spec: &AgentSpec,
    path: &Path,
    exe: &str,
    options: &SetupOptions<'_>,
) -> Result<SetupOutcome, SetupError> {
    let existing = match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(SetupError::ConfigWriteFailed(format!(
                "read {}: {e}",
                path.display()
            )))
        }
    };

    match merge_config(existing.as_deref(), spec.shape, exe, options.force)? {
        MergeOutcome::Unchanged => Ok(SetupOutcome {
            agent: spec.slug,
            path: path.to_path_buf(),
            status: SetupStatus::AlreadyConfigured,
            preview: None,
        }),
        MergeOutcome::NeedsForce => Ok(SetupOutcome {
            agent: spec.slug,
            path: path.to_path_buf(),
            status: SetupStatus::NeedsForce,
            preview: None,
        }),
        MergeOutcome::Updated(text) => {
            if options.dry_run {
                return Ok(SetupOutcome {
                    agent: spec.slug,
                    path: path.to_path_buf(),
                    status: SetupStatus::WouldWrite,
                    preview: Some(text),
                });
            }
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    SetupError::ConfigWriteFailed(format!("create {}: {e}", parent.display()))
                })?;
            }
            fs::write(path, &text).map_err(|e| {
                SetupError::ConfigWriteFailed(format!("write {}: {e}", path.display()))
            })?;
            Ok(SetupOutcome {
                agent: spec.slug,
                path: path.to_path_buf(),
                status: SetupStatus::Written,
                preview: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXE: &str = "C:/tools/odl.exe";

    /// 空文件/不存在 → 生成只含本条目的配置。
    #[test]
    fn merge_creates_entry_from_scratch() {
        let out = merge_config(None, ConfigShape::McpServers, EXE, false).unwrap();
        let MergeOutcome::Updated(text) = out else {
            panic!("expected Updated, got {out:?}");
        };
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["mcpServers"][SERVER_KEY]["command"], EXE);
        assert_eq!(v["mcpServers"][SERVER_KEY]["args"][0], "mcp");
    }

    /// 幂等：写出的内容再次合并 → Unchanged。
    #[test]
    fn merge_is_idempotent() {
        let MergeOutcome::Updated(text) =
            merge_config(None, ConfigShape::Mcp, EXE, false).unwrap()
        else {
            panic!("first merge must write");
        };
        let second = merge_config(Some(&text), ConfigShape::Mcp, EXE, false).unwrap();
        assert_eq!(second, MergeOutcome::Unchanged);
    }

    /// 已有其他 server 的 key 原样保留，未知字段不丢。
    #[test]
    fn merge_preserves_other_servers_and_unknown_fields() {
        let existing = r#"{
            "theme": "dark",
            "mcpServers": {
                "other-tool": {"command": "other", "args": ["run"], "env": {"X": "1"}}
            }
        }"#;
        let MergeOutcome::Updated(text) =
            merge_config(Some(existing), ConfigShape::McpServers, EXE, false).unwrap()
        else {
            panic!("expected Updated");
        };
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["theme"], "dark");
        assert_eq!(v["mcpServers"]["other-tool"]["command"], "other");
        assert_eq!(v["mcpServers"]["other-tool"]["env"]["X"], "1");
        assert_eq!(v["mcpServers"][SERVER_KEY]["command"], EXE);
    }

    /// 旧 `cargo run` 条目：默认 NeedsForce；`--force` 覆盖为二进制路径。
    #[test]
    fn stale_cargo_run_entry_requires_force() {
        let existing = r#"{
            "mcpServers": {
                "open-design-lite": {
                    "command": "cargo",
                    "args": ["run", "-p", "od-cli", "--", "mcp"],
                    "cwd": "/old/clone"
                }
            }
        }"#;
        let out = merge_config(Some(existing), ConfigShape::McpServers, EXE, false).unwrap();
        assert_eq!(out, MergeOutcome::NeedsForce);

        let MergeOutcome::Updated(text) =
            merge_config(Some(existing), ConfigShape::McpServers, EXE, true).unwrap()
        else {
            panic!("force must overwrite");
        };
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["mcpServers"][SERVER_KEY]["command"], EXE);
        assert!(v["mcpServers"][SERVER_KEY].get("cwd").is_none());
    }

    /// Zed 形状：嵌套 command 对象。
    #[test]
    fn zed_shape_nests_command_object() {
        let MergeOutcome::Updated(text) =
            merge_config(None, ConfigShape::ContextServers, EXE, false).unwrap()
        else {
            panic!("expected Updated");
        };
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["context_servers"][SERVER_KEY]["command"]["path"], EXE);
        assert_eq!(v["context_servers"][SERVER_KEY]["command"]["args"][0], "mcp");
    }

    /// 非法 JSON（含注释等）→ config_parse_failed，不落盘、不破坏原文件。
    #[test]
    fn invalid_json_reports_parse_failed() {
        let err = merge_config(
            Some("// jsonc comment\n{}"),
            ConfigShape::McpServers,
            EXE,
            false,
        )
        .unwrap_err();
        assert_eq!(err.code(), "config_parse_failed");
    }
}
