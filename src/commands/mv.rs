use std::path::Path;

use crate::error::{MemtreeError, Result};
use crate::lock::TreeLock;
use crate::tree;

pub fn run(root: &Path, src: &str, dst: &str) -> Result<()> {
    tree::validate_path(src)?;
    tree::validate_path(dst)?;

    if !root.is_dir() {
        return Err(MemtreeError::NotFound(root.to_path_buf()));
    }

    let _lock = TreeLock::acquire(root)?;

    let src_leaf = tree::leaf_path(root, src);
    let src_dir = tree::dir_path(root, src);

    if src_leaf.is_file() {
        let dst_leaf = tree::leaf_path(root, dst);
        if dst_leaf.exists() {
            return Err(MemtreeError::AlreadyExists(dst_leaf));
        }
        // Create parent directories for dst
        if let Some(parent) = dst_leaf.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::rename(&src_leaf, &dst_leaf)?;
    } else if src_dir.is_dir() {
        let dst_dir = tree::dir_path(root, dst);
        if dst_dir.exists() {
            return Err(MemtreeError::AlreadyExists(dst_dir));
        }
        if let Some(parent) = dst_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::rename(&src_dir, &dst_dir)?;
    } else {
        return Err(MemtreeError::NotFound(src_leaf));
    }

    Ok(())
}
