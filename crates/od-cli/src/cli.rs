//! CLI 参数模型（clap derive）。子命令名、flags 与退出码是一等集成契约。
//!
//! Spec: docs/specs/cli.md

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "odl", version, about = "Open Design Lite")]
pub struct Cli {
    /// 只输出错误。
    #[arg(long, global = true)]
    pub quiet: bool,

    /// 输出调试日志，可重复。
    #[arg(long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// 对支持的命令输出 JSON。
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 创建 workspace。
    Init {
        #[arg(default_value = ".")]
        dir: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// 创建 artifact。
    New {
        /// `html` | `docs` | `slides`。
        kind: String,
        dir: PathBuf,
        #[arg(long)]
        title: Option<String>,
        #[arg(long, default_value = "editorial")]
        brief: String,
        #[arg(long)]
        embed_css: bool,
        #[arg(long)]
        force: bool,
    },
    /// 打开 artifact 预览。
    Preview {
        dir: PathBuf,
        #[arg(long)]
        external_browser: bool,
        #[arg(long)]
        no_watch: bool,
        #[arg(long)]
        devtools: bool,
    },
    /// 生成或刷新 handoff.md。
    Handoff {
        dir: PathBuf,
        #[arg(long)]
        stdout: bool,
        #[arg(long, default_value = "generic")]
        agent: String,
    },
    /// 导出 artifact（M4）。
    Export {
        dir: PathBuf,
        #[arg(long)]
        format: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// 列出可用 skill。
    Skill,
}
