use std::path::Path;

use crate::error::{MemtreeError, Result};
use crate::leaf::Leaf;

pub fn run(root: &Path, path: Option<&str>, depth: usize) -> Result<()> {
    if !root.is_dir() {
        return Err(MemtreeError::NotFound(root.to_path_buf()));
    }

    let base = match path {
        Some(p) => {
            crate::tree::validate_path(p)?;
            root.join(p)
        }
        None => root.to_path_buf(),
    };

    if !base.is_dir() {
        return Err(MemtreeError::NotFound(base));
    }

    list_dir(&base, root, 0, depth)?;
    Ok(())
}

fn list_dir(dir: &Path, root: &Path, current_depth: usize, max_depth: usize) -> Result<()> {
    if current_depth >= max_depth {
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name != "_summary.md"
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let indent = "  ".repeat(current_depth);

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if path.is_dir() {
            // Read summary for this directory
            let summary_path = path.join("_summary.md");
            let summary = if summary_path.is_file() {
                let s = std::fs::read_to_string(&summary_path)?;
                s.trim().lines().next().unwrap_or("").to_string()
            } else {
                String::new()
            };

            let display = format!("{}/", name);
            println!("{}{:<25}{}", indent, display, summary);

            list_dir(&path, root, current_depth + 1, max_depth)?;
        } else if name.ends_with(".md") {
            // Read frontmatter summary
            let content = std::fs::read_to_string(&path)?;
            let summary = if let Ok(leaf) = Leaf::parse(&content) {
                leaf.frontmatter.summary
            } else {
                String::new()
            };

            println!("{}{:<25}{}", indent, name, summary);
        }
    }

    Ok(())
}
