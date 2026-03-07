use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::{MemtreeError, Result};

/// Validate a logical path (e.g. "rust/errors").
pub fn validate_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(MemtreeError::InvalidPath {
            reason: "path is empty".into(),
        });
    }
    if path.starts_with('/') {
        return Err(MemtreeError::InvalidPath {
            reason: "absolute paths not allowed".into(),
        });
    }
    if path.contains("..") {
        return Err(MemtreeError::InvalidPath {
            reason: "directory traversal not allowed".into(),
        });
    }
    for component in path.split('/') {
        if component.is_empty() {
            return Err(MemtreeError::InvalidPath {
                reason: "empty path component".into(),
            });
        }
        if component == "_summary" {
            return Err(MemtreeError::InvalidPath {
                reason: "_summary is a reserved name".into(),
            });
        }
    }
    Ok(())
}

/// Returns the filesystem path for a leaf: `<root>/<path>.md`
pub fn leaf_path(root: &Path, logical: &str) -> PathBuf {
    root.join(format!("{}.md", logical))
}

/// Returns the filesystem path for a directory node: `<root>/<path>/`
pub fn dir_path(root: &Path, logical: &str) -> PathBuf {
    root.join(logical)
}

/// Returns the summary path for a directory node: `<root>/<path>/_summary.md`
pub fn summary_path(root: &Path, logical: &str) -> PathBuf {
    root.join(logical).join("_summary.md")
}

/// Write content atomically: write to temp file then rename.
pub fn atomic_write(target: &Path, content: &[u8]) -> Result<()> {
    let parent = target.parent().ok_or_else(|| MemtreeError::InvalidPath {
        reason: "no parent directory".into(),
    })?;
    fs::create_dir_all(parent)?;

    let temp_path = parent.join(format!(".tmp.{}", std::process::id()));
    {
        let mut f = fs::File::create(&temp_path)?;
        f.write_all(content)?;
        f.sync_all()?;
    }
    fs::rename(&temp_path, target)?;
    Ok(())
}

/// Auto-promote a leaf to a directory.
/// If `rust/errors.md` exists and we need `rust/errors/` as a directory,
/// move `rust/errors.md` → `rust/errors/errors.md`.
pub fn auto_promote(root: &Path, logical: &str) -> Result<()> {
    let leaf = leaf_path(root, logical);
    if leaf.is_file() {
        let dir = dir_path(root, logical);
        let basename = Path::new(logical)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        fs::create_dir_all(&dir)?;
        let new_leaf = dir.join(format!("{}.md", basename));
        fs::rename(&leaf, &new_leaf)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_paths() {
        assert!(validate_path("rust/errors").is_ok());
        assert!(validate_path("python").is_ok());
        assert!(validate_path("a/b/c/d").is_ok());
    }

    #[test]
    fn invalid_paths() {
        assert!(validate_path("").is_err());
        assert!(validate_path("/absolute").is_err());
        assert!(validate_path("a/../b").is_err());
        assert!(validate_path("a//b").is_err());
        assert!(validate_path("a/_summary/b").is_err());
        assert!(validate_path("_summary").is_err());
    }

    #[test]
    fn test_leaf_path() {
        let root = Path::new("/tmp/memtree");
        assert_eq!(leaf_path(root, "rust/errors"), PathBuf::from("/tmp/memtree/rust/errors.md"));
    }

    #[test]
    fn test_auto_promote() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        // Create a leaf at rust/errors.md
        let rust_dir = root.join("rust");
        fs::create_dir_all(&rust_dir).unwrap();
        fs::write(rust_dir.join("errors.md"), "content").unwrap();

        // Promote it
        auto_promote(root, "rust/errors").unwrap();

        // errors.md should now be at rust/errors/errors.md
        assert!(root.join("rust/errors/errors.md").is_file());
        assert!(!root.join("rust/errors.md").is_file());
    }
}
