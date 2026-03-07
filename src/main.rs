mod cli;
mod commands;
mod config;
mod error;
mod leaf;
mod lock;
mod summary;
mod tree;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    let result = run(cli);
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> error::Result<()> {
    let root = config::resolve_root(cli.root.as_deref())?;

    match cli.command {
        Command::Store {
            path,
            summary,
            content,
            tags,
        } => {
            commands::store::run(
                &root,
                &path,
                &summary,
                content.as_deref(),
                tags.unwrap_or_default(),
            )?;
        }
        Command::Recall { path, full } => {
            commands::recall::run(&root, &path, full)?;
        }
        Command::Ls { path, depth } => {
            commands::ls::run(&root, path.as_deref(), depth)?;
        }
        Command::Search { query } => {
            commands::search::run(&root, &query)?;
        }
        Command::Move { src, dst } => {
            commands::mv::run(&root, &src, &dst)?;
        }
        Command::Delete { path, force } => {
            commands::delete::run(&root, &path, force)?;
        }
    }

    Ok(())
}
