---
summary: memtree delete — remove a leaf or subtree
created: '2026-03-09T15:01:11.424240Z'
updated: '2026-03-09T15:01:11.424240Z'
tags:
- commands
- delete
---

Usage:
  memtree delete <path> [--force]

Deletes a leaf or directory. Without --force, refuses to delete non-empty directories (NonEmptyDirectory error with message 'Directory not empty: {path} (use --force to delete)'). With --force, recursively deletes.

Errors if path not found (NotFound error). Acquires write lock.
