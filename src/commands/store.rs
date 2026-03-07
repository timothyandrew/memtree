use std::io::Read;
use std::path::Path;

use crate::error::Result;
use crate::leaf::Leaf;
use crate::lock::TreeLock;
use crate::tree;

pub fn run(
    root: &Path,
    path: &str,
    summary: &str,
    content: Option<&str>,
    tags: Vec<String>,
) -> Result<()> {
    tree::validate_path(path)?;
    std::fs::create_dir_all(root)?;
    let _lock = TreeLock::acquire(root)?;

    // Determine if content comes from --content flag, stdin, or is absent
    let body = match content {
        Some(c) if c == "-" => {
            // Explicit stdin read via --content -
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            if buf.is_empty() { None } else { Some(buf) }
        }
        Some(c) => Some(c.to_string()),
        None => None,
    };

    match body {
        Some(body) => store_leaf(root, path, summary, &body, tags)?,
        None => store_summary(root, path, summary)?,
    }

    Ok(())
}

fn store_leaf(
    root: &Path,
    logical: &str,
    summary: &str,
    body: &str,
    tags: Vec<String>,
) -> Result<()> {
    // Check if any ancestor is an existing leaf that needs promotion
    let components: Vec<&str> = logical.split('/').collect();
    for i in 1..components.len() {
        let ancestor = components[..i].join("/");
        let ancestor_leaf = tree::leaf_path(root, &ancestor);
        if ancestor_leaf.is_file() {
            tree::auto_promote(root, &ancestor)?;
        }
    }

    let target = tree::leaf_path(root, logical);
    let leaf = if target.is_file() {
        // Update existing leaf
        let content = std::fs::read_to_string(&target)?;
        let mut leaf = Leaf::parse(&content)?;
        leaf.update(summary.to_string(), body.to_string(), tags);
        leaf
    } else {
        Leaf::new(summary.to_string(), body.to_string(), tags)
    };

    let serialized = leaf.serialize()?;
    tree::atomic_write(&target, serialized.as_bytes())?;
    Ok(())
}

fn store_summary(root: &Path, logical: &str, summary: &str) -> Result<()> {
    // Check if any ancestor is an existing leaf that needs promotion
    let components: Vec<&str> = logical.split('/').collect();
    for i in 1..components.len() {
        let ancestor = components[..i].join("/");
        let ancestor_leaf = tree::leaf_path(root, &ancestor);
        if ancestor_leaf.is_file() {
            tree::auto_promote(root, &ancestor)?;
        }
    }

    // Also promote self if it's a leaf
    let self_leaf = tree::leaf_path(root, logical);
    if self_leaf.is_file() {
        tree::auto_promote(root, logical)?;
    }

    let dir = tree::dir_path(root, logical);
    std::fs::create_dir_all(&dir)?;
    crate::summary::write_summary(root, logical, summary)?;
    Ok(())
}
