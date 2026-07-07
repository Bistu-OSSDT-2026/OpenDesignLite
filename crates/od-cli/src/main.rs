//! `odl` 入口：解析 → 分发 → 映射退出码。
//!
//! Spec: docs/specs/cli.md

mod cli;
mod commands;
mod error;
mod exit;
mod output;

use clap::Parser;
use cli::{Cli, Command};
use error::Result;
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

fn dispatch(command: &Command, reporter: &Reporter) -> Result<()> {
    match command {
        Command::Init { dir, name, force } => {
            commands::init::run(dir, name.as_deref(), *force)?;
            reporter.info(&format!("initialized {}", dir.display()));
        }
        Command::New { kind, dir, .. } => {
            let artifact = commands::new::run(kind, dir)?;
            reporter.info(&format!("created {}", artifact.primary_path().display()));
        }
        Command::Preview {
            dir,
            external_browser,
            no_watch,
            devtools,
        } => {
            // no_watch 是用户视角否定式，翻转成 watch 肯定式。
            commands::preview::run(dir, *external_browser, !*no_watch, *devtools)?;
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
