---
summary: src/leaf.rs, tree.rs, lock.rs, summary.rs module purposes
created: '2026-03-09T15:00:26.975333Z'
updated: '2026-03-09T15:00:26.975333Z'
tags:
- source
- modules
---

src/leaf.rs — LeafFrontmatter serde struct and Leaf parser
- LeafFrontmatter: summary, created, updated, tags fields
- Leaf::parse(content) — splits frontmatter from body via --- delimiters
- Leaf::serialize() — renders frontmatter + body back to string
- Unit tests: roundtrip, update_preserves_created

src/tree.rs — path validation and filesystem helpers
- validate_path(path) — rejects: .., leading /, _summary, empty components
- leaf_path(root, path) — resolves logical path to .md file path
- dir_path(root, path) — resolves logical path to directory path
- atomic_write(path, content) — writes to temp file then renames
- auto_promote(root, path) — converts existing leaf to directory when storing nested path
  e.g. rust/errors.md -> rust/errors/errors.md
- Unit tests: valid_paths, invalid_paths, test_leaf_path, test_auto_promote

src/lock.rs — global lockfile via fs4 flock
- Acquires exclusive flock on <root>/.memtree.lock
- Only write commands lock; read commands skip

src/summary.rs — _summary.md read/write
- Reads/writes plain text directory summaries
