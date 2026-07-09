//! `odl` 入口：解析 → 分发 → 映射退出码。
//!
//! Spec: docs/specs/cli.md

mod cli;
mod commands;
mod exit;
mod output;

use clap::Parser;
use cli::{Cli, Command};
use output::Reporter;

fn main() {
    let args = Cli::parse();
    let reporter = Reporter {
        json: args.json,
        quiet: args.quiet,
    };

    if let Err(err) = dispatch(&args.command, &reporter) {
        reporter.error(err.code(), &err.to_string());
        std::process::exit(exit::code_for(&err));
    }
}

fn dispatch(command: &Command, reporter: &Reporter) -> od_core::Result<()> {
    match command {
        Command::Init { dir, name, force } => {
            commands::init::run(dir, name.as_deref(), *force)?;
            reporter.info(&format!("initialized {}", dir.display()));
        }
        Command::New {
            kind,
            dir,
            title,
            brief,
            embed_css,
            force,
        } => {
            let artifact = commands::new::run(
                kind,
                dir,
                title.as_deref(),
                brief.as_deref(),
                *embed_css,
                *force,
                reporter,
            )?;
            if reporter.json {
                let payload = serde_json::json!({
                    "kind": artifact.kind.slug(),
                    "root": artifact.root.display().to_string(),
                    "primaryFile": artifact.kind.primary_file(),
                });
                reporter.success_artifact(&payload);
            } else {
                reporter.info(&format!("created {}", artifact.primary_path().display()));
            }
        }
        Command::Preview {
            dir,
            external_browser,
            no_watch,
            devtools,
        } => {
            // 预览错误独立于 OdError（od-core 不依赖 od-preview）。
            match commands::preview::run(dir, *external_browser, *no_watch, *devtools) {
                Ok(()) => reporter.info(&format!("preview closed: {}", dir.display())),
                Err(e) => {
                    reporter.error(e.code(), &e.to_string());
                    std::process::exit(exit::code_for_preview(&e));
                }
            }
        }
        Command::Handoff { dir, stdout, agent } => {
            commands::handoff::run(dir, agent, *stdout, reporter)?;
        }
        Command::Export { dir, format, .. } => {
            commands::export::run(dir, format)?;
        }
        Command::Skill { action, json } => {
            commands::skill::run(action.as_ref(), *json, reporter)?;
        }
    }
    Ok(())
}
