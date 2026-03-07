use std::path::Path;

use crate::error::{MemtreeError, Result};
use crate::leaf::Leaf;
use crate::tree;

pub fn run(root: &Path, path: &str, full: bool) -> Result<()> {
    tree::validate_path(path)?;

    if !root.is_dir() {
        return Err(MemtreeError::NotFound(root.to_path_buf()));
    }

    let leaf_p = tree::leaf_path(root, path);
    let dir_p = tree::dir_path(root, path);

    if leaf_p.is_file() {
        let content = std::fs::read_to_string(&leaf_p)?;
        if full {
            print!("{}", content);
        } else {
            let leaf = Leaf::parse(&content)?;
            println!("{}", leaf.body);
        }
    } else if dir_p.is_dir() {
        let summary = crate::summary::read_summary(root, path)?;
        if !summary.is_empty() {
            println!("{}", summary.trim());
        }
        println!();
        println!("Children:");
        let mut entries: Vec<_> = std::fs::read_dir(&dir_p)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !name.starts_with('.') && name != "_summary.md"
            })
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() {
                let child_summary = crate::summary::read_summary(root, &format!("{}/{}", path, name))?;
                let s = child_summary.trim().lines().next().unwrap_or("").to_string();
                println!("  {}/    {}", name, s);
            } else if name.ends_with(".md") {
                let content = std::fs::read_to_string(entry.path())?;
                if let Ok(leaf) = Leaf::parse(&content) {
                    println!("  {}    {}", name, leaf.frontmatter.summary);
                } else {
                    println!("  {}", name);
                }
            }
        }
    } else {
        return Err(MemtreeError::NotFound(leaf_p));
    }

    Ok(())
}
