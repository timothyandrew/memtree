use std::fs::{File, OpenOptions};
use std::path::Path;

use fs4::fs_std::FileExt;

use crate::error::{MemtreeError, Result};

pub struct TreeLock {
    _file: File,
}

impl TreeLock {
    pub fn acquire(root: &Path) -> Result<Self> {
        let lock_path = root.join(".memtree.lock");
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)?;

        file.lock_exclusive()
            .map_err(|e| MemtreeError::LockFailure(e.to_string()))?;

        Ok(TreeLock { _file: file })
    }
}
