---
summary: src/cli.rs — clap derive definitions
created: '2026-03-09T15:00:01.349001Z'
updated: '2026-03-09T15:00:01.349001Z'
tags:
- source
- cli
- clap
---

src/cli.rs defines the CLI interface using clap derive macros.

Cli struct:
- #[arg(long, global = true)] root: Option<String>  — tree root override
- #[command(subcommand)] command: Command

Command enum variants:
- Store { path: String, summary: String, content: Option<String>, tags: Option<Vec<String>> }
  - path and summary are --long args
  - content is optional --long, reads stdin with '-'
  - tags uses value_delimiter = ','
- Recall { path: String (positional), full: bool (--long) }
- Ls { path: Option<String> (positional), depth: usize (--long, default 1) }
- Inspect (no args)
- Search { query: String (positional) }
- Move { src: String (positional), dst: String (positional) }
- Delete { path: String (positional), force: bool (--long) }
