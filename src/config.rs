use std::path::PathBuf;

use crate::error::{MemtreeError, Result};

pub fn resolve_root(cli_root: Option<&str>) -> Result<PathBuf> {
    if let Some(root) = cli_root {
        return Ok(PathBuf::from(root));
    }

    if let Ok(root) = std::env::var("MEMTREE_ROOT") {
        return Ok(PathBuf::from(root));
    }

    let home = std::env::var("HOME").map_err(|_| MemtreeError::NoHome)?;
    Ok(PathBuf::from(home).join(".memtree"))
}
