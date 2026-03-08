---
summary: src/commands/mod.rs — command module declarations
created: '2026-03-09T15:00:17.300091Z'
updated: '2026-03-09T15:00:17.300091Z'
tags:
- source
- commands
---

src/commands/mod.rs declares all command modules:

pub mod store;
pub mod recall;
pub mod ls;
pub mod mv;
pub mod delete;
pub mod search;

Each module has a pub fn run() as its entry point. One file per command.
