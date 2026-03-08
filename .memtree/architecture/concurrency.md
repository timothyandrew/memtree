---
summary: 'Concurrency model: flock for writes, no lock for reads'
created: '2026-03-09T15:00:35.410649Z'
updated: '2026-03-09T15:00:35.410649Z'
tags:
- architecture
- concurrency
- locking
---

Write commands (store, move, delete) acquire an exclusive flock on <root>/.memtree.lock via the fs4 crate. Read commands (recall, ls, inspect, search) do not lock.

Writes are atomic: content is written to a temp file first, then renamed into place. This means readers never see partial content even without locking.

The lock file is at the root of the tree (.memtree/.memtree.lock).
