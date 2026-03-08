---
summary: memtree move — move a leaf or subtree
created: '2026-03-09T15:01:10.045856Z'
updated: '2026-03-09T15:01:10.045856Z'
tags:
- commands
- move
---

Usage:
  memtree move <src> <dst>

Moves a leaf or entire subtree from src to dst path. Both paths are validated. Errors if destination already exists (AlreadyExists error). Acquires write lock.

Works for both individual leaves (.md files) and directories (entire subtrees).
