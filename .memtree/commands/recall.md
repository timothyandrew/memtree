---
summary: memtree recall — print leaf body or directory info
created: '2026-03-09T15:00:59.093038Z'
updated: '2026-03-09T15:00:59.093038Z'
tags:
- commands
- recall
---

Usage:
  memtree recall <path> [--full]

For leaves:
- Default: prints just the body (no frontmatter)
- --full: includes YAML frontmatter (summary, created, updated, tags) in output

For directories:
- Prints the directory summary from _summary.md
- Prints 'Children:' header followed by list of child entries

No locking (read-only operation).
