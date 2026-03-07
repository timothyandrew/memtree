use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeafFrontmatter {
    pub summary: String,
    #[serde(with = "chrono::serde::ts_seconds_option", default, skip_serializing_if = "Option::is_none")]
    #[serde(skip)]
    _created_ts: Option<i64>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// A parsed leaf node: frontmatter + body.
pub struct Leaf {
    pub frontmatter: LeafFrontmatter,
    pub body: String,
}

const FRONTMATTER_DELIM: &str = "---";

impl Leaf {
    pub fn new(summary: String, body: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Leaf {
            frontmatter: LeafFrontmatter {
                summary,
                created: now,
                updated: now,
                tags,
                _created_ts: None,
            },
            body,
        }
    }

    /// Parse a leaf from markdown with YAML frontmatter.
    pub fn parse(content: &str) -> Result<Self> {
        let content = content.trim_start();
        if !content.starts_with(FRONTMATTER_DELIM) {
            return Err(crate::error::MemtreeError::InvalidPath {
                reason: "missing frontmatter delimiter".into(),
            });
        }

        let after_first = &content[FRONTMATTER_DELIM.len()..];
        let end = after_first
            .find(&format!("\n{}", FRONTMATTER_DELIM))
            .ok_or_else(|| crate::error::MemtreeError::InvalidPath {
                reason: "missing closing frontmatter delimiter".into(),
            })?;

        let yaml_str = &after_first[..end];
        let frontmatter: LeafFrontmatter = serde_yml::from_str(yaml_str.trim())?;

        let body_start = FRONTMATTER_DELIM.len() + end + 1 + FRONTMATTER_DELIM.len();
        let body = content[body_start..].trim().to_string();

        Ok(Leaf { frontmatter, body })
    }

    /// Serialize to markdown with YAML frontmatter.
    pub fn serialize(&self) -> Result<String> {
        let yaml = serde_yml::to_string(&self.frontmatter)?;
        Ok(format!("---\n{}---\n\n{}\n", yaml, self.body.trim_end()))
    }

    /// Update an existing leaf with new content, preserving `created`.
    pub fn update(&mut self, summary: String, body: String, tags: Vec<String>) {
        self.frontmatter.summary = summary;
        self.frontmatter.updated = Utc::now();
        self.body = body;
        if !tags.is_empty() {
            self.frontmatter.tags = tags;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let leaf = Leaf::new(
            "Test summary".into(),
            "Some body content".into(),
            vec!["tag1".into(), "tag2".into()],
        );
        let serialized = leaf.serialize().unwrap();
        let parsed = Leaf::parse(&serialized).unwrap();
        assert_eq!(parsed.frontmatter.summary, "Test summary");
        assert_eq!(parsed.body, "Some body content");
        assert_eq!(parsed.frontmatter.tags, vec!["tag1", "tag2"]);
    }

    #[test]
    fn update_preserves_created() {
        let mut leaf = Leaf::new("Old".into(), "Old body".into(), vec![]);
        let created = leaf.frontmatter.created;
        std::thread::sleep(std::time::Duration::from_millis(10));
        leaf.update("New".into(), "New body".into(), vec!["new-tag".into()]);
        assert_eq!(leaf.frontmatter.created, created);
        assert_eq!(leaf.frontmatter.summary, "New");
        assert_eq!(leaf.body, "New body");
    }
}
