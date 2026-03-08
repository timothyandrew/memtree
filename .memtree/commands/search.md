---
summary: memtree search — case-insensitive substring search across leaves
created: '2026-03-09T15:01:07.892228Z'
updated: '2026-03-09T15:01:07.892228Z'
tags:
- commands
- search
---

Usage:
  memtree search <query>

Searches all leaves for case-insensitive substring matches in both summary and body content. Prints matching leaf paths with their summaries.

If no matches found, prints 'No matches' to stderr.
No locking (read-only).
