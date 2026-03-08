---
summary: memtree ls and inspect — tree listing commands
created: '2026-03-09T15:01:02.654764Z'
updated: '2026-03-09T15:01:02.654764Z'
tags:
- commands
- ls
- inspect
---

memtree ls [path] [--depth N]:
- Lists tree with summaries at each level
- Default depth: 1 (top-level only)
- Optional path to list a subtree
- Format: indented name + summary, dirs end with /
- No locking (read-only)

memtree inspect:
- Prints entire tree with all summaries, no depth limit
- Equivalent to ls with usize::MAX depth
- No arguments, no path filtering
- Added in commit 1d57e57
- Implementation: dispatches to commands::ls::run(&root, None, usize::MAX)
