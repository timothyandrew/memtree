---
summary: src/config.rs — tree root resolution
created: '2026-03-09T15:00:03.157649Z'
updated: '2026-03-09T15:00:03.157649Z'
tags:
- source
- config
---

src/config.rs contains a single function: resolve_root(cli_root: Option<&str>) -> Result<PathBuf>

Resolution order:
1. cli_root argument (from --root flag) — returns PathBuf::from(root)
2. MEMTREE_ROOT env var — returns PathBuf::from(root)
3. Default: PathBuf::from(".memtree") — CWD-local

Previously defaulted to ~/.memtree/ (HOME env + .memtree join) with a NoHome error variant. Changed in commit 1d57e57 to use CWD-local .memtree/ directory. The NoHome error variant and MemtreeError import were removed.
