use std::path::Path;

use crate::error::{MemtreeError, Result};
use crate::lock::TreeLock;
use crate::tree;

pub fn run(root: &Path, path: &str, force: bool) -> Result<()> {
    tree::validate_path(path)?;

    if !root.is_dir() {
        return Err(MemtreeError::NotFound(root.to_path_buf()));
    }

    let _lock = TreeLock::acquire(root)?;

    let leaf_p = tree::leaf_path(root, path);
    let dir_p = tree::dir_path(root, path);

    if leaf_p.is_file() {
        std::fs::remove_file(&leaf_p)?;
    } else if dir_p.is_dir() {
        // Check if non-empty (besides _summary.md)
        let has_children = std::fs::read_dir(&dir_p)?
            .filter_map(|e| e.ok())
            .any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name != "_summary.md" && !name.starts_with('.')
            });

        if has_children && !force {
            return Err(MemtreeError::NonEmptyDirectory { path: dir_p });
        }

        std::fs::remove_dir_all(&dir_p)?;
    } else {
        return Err(MemtreeError::NotFound(leaf_p));
    }

    Ok(())
}
