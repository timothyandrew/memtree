---
summary: src/main.rs — entrypoint and clap dispatch
created: '2026-03-09T14:59:55.282218Z'
updated: '2026-03-09T14:59:55.282218Z'
tags:
- source
- entrypoint
---

src/main.rs is the binary entrypoint. Structure:

- Declares modules: cli, commands, config, error, leaf, lock, summary, tree
- main() parses CLI with Cli::parse(), calls run(), prints errors to stderr and exits with code 1
- run(cli) resolves root via config::resolve_root(), then matches on cli.command:
  - Command::Store { path, summary, content, tags } => commands::store::run()
  - Command::Recall { path, full } => commands::recall::run()
  - Command::Ls { path, depth } => commands::ls::run()
  - Command::Inspect => commands::ls::run(&root, None, usize::MAX)
  - Command::Search { query } => commands::search::run()
  - Command::Move { src, dst } => commands::mv::run()
  - Command::Delete { path, force } => commands::delete::run()
