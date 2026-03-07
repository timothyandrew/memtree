use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum MemtreeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yml::Error),

    #[error("Invalid path: {reason}")]
    InvalidPath { reason: String },

    #[error("Not found: {0}")]
    NotFound(PathBuf),

    #[error("Already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("Directory not empty: {path} (use --force to delete)")]
    NonEmptyDirectory { path: PathBuf },

    #[error("Could not determine home directory")]
    NoHome,

    #[error("Failed to acquire lock: {0}")]
    LockFailure(String),
}

pub type Result<T> = std::result::Result<T, MemtreeError>;
