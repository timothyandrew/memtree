use std::path::Path;

use crate::error::Result;
use crate::tree;

pub fn read_summary(root: &Path, logical: &str) -> Result<String> {
    let path = tree::summary_path(root, logical);
    if !path.is_file() {
        return Ok(String::new());
    }
    Ok(std::fs::read_to_string(&path)?)
}

pub fn write_summary(root: &Path, logical: &str, text: &str) -> Result<()> {
    let path = tree::summary_path(root, logical);
    tree::atomic_write(&path, text.as_bytes())
}
