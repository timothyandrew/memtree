---
summary: src/error.rs — MemtreeError enum
created: '2026-03-09T15:00:10.572895Z'
updated: '2026-03-09T15:00:10.572895Z'
tags:
- source
- errors
- thiserror
---

src/error.rs defines the error type using thiserror derive.

MemtreeError variants:
- Io(#[from] std::io::Error) — "IO error: {0}"
- Yaml(#[from] serde_yml::Error) — "YAML parse error: {0}"
- InvalidPath { reason: String } — "Invalid path: {reason}"
- NotFound(PathBuf) — "Not found: {0}"
- AlreadyExists(PathBuf) — "Already exists: {0}"
- NonEmptyDirectory { path: PathBuf } — "Directory not empty: {path} (use --force to delete)"
- LockFailure(String) — "Failed to acquire lock: {0}"

Also defines: pub type Result<T> = std::result::Result<T, MemtreeError>;

Removed variant: NoHome (was for ~/home resolution, no longer needed after CWD-local default).
