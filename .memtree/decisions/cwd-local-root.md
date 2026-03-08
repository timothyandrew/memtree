---
summary: Tree root is .memtree/ in CWD, not ~/.memtree/
created: '2026-03-09T14:54:14.814624Z'
updated: '2026-03-09T14:54:14.814624Z'
tags:
- decisions
- config
---

DECISION: The memtree skills use --root .memtree on every command to store the tree in the current working directory rather than the binary's default of ~/.memtree/.

RATIONALE: User explicitly requested CWD-local storage so each project gets its own memtree. The binary's resolve_root() in src/config.rs defaults to ~/.memtree/ (via HOME env var), so --root .memtree must be passed explicitly.

OVERRIDE: Set MEMTREE_ROOT env var if a different root is preferred.
