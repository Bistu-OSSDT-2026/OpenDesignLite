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
        Command::New { kind, dir, .. } => {
            let artifact = commands::new::run(kind, dir)?;
            reporter.info(&format!("created {}", artifact.primary_path().display()));
        }
        Command::Preview { dir, .. } => {
            let target = commands::preview::run(dir)?;
            reporter.info(&format!("preview target: {}", target.display()));
            reporter.info("native shell preview is not implemented yet");
        }
        Command::Handoff { dir, stdout, agent } => {
            commands::handoff::run(dir, agent, *stdout)?;
        }
        Command::Export { dir, format, .. } => {
            commands::export::run(dir, format)?;
        }
    }
    Ok(())
}
