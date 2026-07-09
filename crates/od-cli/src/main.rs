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
use serde_json::json;

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
            reporter.json_value(json!({"ok": true, "workspace": {"root": dir}}));
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
            let result = commands::new::run(
                kind,
                dir,
                commands::new::NewOptions {
                    title: title.as_deref(),
                    brief,
                    embed_css: *embed_css,
                    force: *force,
                },
            )?;
            for warning in &result.warnings {
                reporter.warn(warning);
            }
            let artifact = result.artifact;
            reporter.json_value(json!({
                "ok": true,
                "artifact": {
                    "kind": artifact.kind.slug(),
                    "root": artifact.root,
                    "primaryFile": artifact.kind.primary_file(),
                }
            }));
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
            if let Some(rendered) = commands::handoff::run(dir, agent, *stdout)? {
                println!("{rendered}");
            } else {
                reporter.info(&format!("updated {}", dir.join("handoff.md").display()));
            }
        }
        Command::Export { dir, format, .. } => {
            commands::export::run(dir, format)?;
        }
        Command::Skill => {
            let cwd = std::env::current_dir()?;
            let skills = commands::skill::list(&cwd);
            reporter.json_value(json!(skills
                .iter()
                .map(|skill| json!({
                    "name": skill.front.name,
                    "mode": skill.front.mode,
                    "description": skill.front.description,
                }))
                .collect::<Vec<_>>()));
            if !reporter.json {
                for skill in skills {
                    reporter.info(&format!(
                        "{} | {} | {}",
                        skill.front.name, skill.front.mode, skill.front.description
                    ));
                }
            }
        }
    }
    Ok(())
}
