---
summary: Tree root is .memtree/ in CWD, not ~/.memtree/
created: '2026-03-09T14:54:14.814624Z'
updated: '2026-03-09T15:35:14.125725Z'
tags:
- decisions
- config
---

DECISION: Changed the binary default in src/config.rs from ~/.memtree/ to .memtree/ (CWD-local). Previously the skills had to pass --root .memtree on every command; now it's the default.

resolve_root() priority:
1. --root flag (cli_root argument)
2. MEMTREE_ROOT env var
3. PathBuf::from(".memtree") — CWD-local default

Commit: 85e9a72 (Change default tree root to CWD-local .memtree/ and add inspect command)

Changes made:
- src/config.rs: removed HOME env var lookup, now returns PathBuf::from(".memtree")
- src/error.rs: removed NoHome variant (no longer needed)
- Both skills: removed all --root .memtree flags
- README: updated Tree Root section and mermaid diagram
- CLAUDE.md: updated config.rs description
