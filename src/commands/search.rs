use std::path::Path;

use walkdir::WalkDir;

use crate::error::{MemtreeError, Result};
use crate::leaf::Leaf;

pub fn run(root: &Path, query: &str) -> Result<()> {
    if !root.is_dir() {
        return Err(MemtreeError::NotFound(root.to_path_buf()));
    }

    let query_lower = query.to_lowercase();
    let mut found = false;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy();
            e.file_type().is_file()
                && name.ends_with(".md")
                && name.as_ref() != "_summary.md"
                && !name.starts_with('.')
        })
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path)?;

        if content.to_lowercase().contains(&query_lower) {
            // Compute logical path
            let rel = path.strip_prefix(root).unwrap_or(path);
            let logical = rel
                .to_string_lossy()
                .trim_end_matches(".md")
                .to_string();

            let summary = if let Ok(leaf) = Leaf::parse(&content) {
                leaf.frontmatter.summary
            } else {
                String::new()
            };

            // Find context snippet
            let snippet = find_snippet(&content, &query_lower);

            println!("{}    {}", logical, summary);
            if let Some(s) = snippet {
                println!("  ...{}...", s.trim());
            }
            println!();
            found = true;
        }
    }

    if !found {
        eprintln!("No matches found for \"{}\"", query);
    }

    Ok(())
}

fn find_snippet(content: &str, query_lower: &str) -> Option<String> {
    let content_lower = content.to_lowercase();
    let idx = content_lower.find(query_lower)?;

    let start = idx.saturating_sub(30);
    let end = (idx + query_lower.len() + 30).min(content.len());

    // Snap to char boundaries
    let start = content.floor_char_boundary(start);
    let end = content.ceil_char_boundary(end);

    Some(content[start..end].to_string())
}
