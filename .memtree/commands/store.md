---
summary: memtree store — create/update leaves or directory summaries
created: '2026-03-09T15:00:56.234503Z'
updated: '2026-03-09T15:00:56.234503Z'
tags:
- commands
- store
---

Usage:
  memtree store --path <path> --summary <text> [--content <text>] [--tags t1,t2]

With --content: creates/updates a leaf (.md file with frontmatter + body)
Without --content: creates/updates a directory summary (_summary.md plain text)

Behavior:
- Creates parent directories as needed
- Acquires write lock
- Uses atomic writes (temp file + rename)
- Triggers auto-promotion if storing under an existing leaf
- On update: preserves original 'created' timestamp, updates 'updated'
- Tags are comma-delimited via --tags flag
- --content - reads body from stdin
