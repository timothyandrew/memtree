---
summary: 'Auto-promotion: leaf becomes directory when storing nested path'
created: '2026-03-09T15:00:41.496990Z'
updated: '2026-03-09T15:00:41.496990Z'
tags:
- architecture
- auto-promotion
---

When storing a nested path under an existing leaf, the leaf is automatically promoted to a directory. 

Example: storing at rust/errors/handling when rust/errors.md exists:
1. Creates rust/errors/ directory
2. Moves rust/errors.md to rust/errors/errors.md (leaf name = parent dir name)
3. Creates the new rust/errors/handling.md leaf

Implementation in src/tree.rs auto_promote() function. Tested in tests/smoke.rs auto_promote_leaf_to_directory test.
