---
summary: Commits made in this session
created: '2026-03-09T15:01:37.894451Z'
updated: '2026-03-09T15:01:37.894451Z'
tags:
- commits
- history
---

Commits made during this conversation session:

1. c7211a4 — Add memtree-save skill for persisting/restoring conversation context
   - Created skills/memtree-save/SKILL.md
   - Two modes: save (audit + store) and restore (ls + drill)
   - Initially used --root .memtree on all commands

2. c6922ab — Add memtree-load skill for prompt-driven context retrieval
   - Created skills/memtree-load/SKILL.md
   - Takes user prompt, searches tree, recalls relevant leaves, synthesizes

3. 1d57e57 — Change default tree root to CWD-local .memtree/ and add inspect command
   - src/config.rs: default from ~/.memtree/ to .memtree/
   - src/error.rs: removed NoHome variant
   - src/cli.rs + main.rs: added Inspect command (ls with usize::MAX depth)
   - tests/smoke.rs: added inspect_shows_full_tree test
   - Updated README, CLAUDE.md, both skills to reflect new default
