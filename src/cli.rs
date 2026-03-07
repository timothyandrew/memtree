use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "memtree", about = "Filesystem-based memory tree for AI agents")]
pub struct Cli {
    /// Root directory for the memory tree
    #[arg(long, global = true)]
    pub root: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Store a memory (leaf or directory summary)
    Store {
        /// Logical path (e.g. rust/errors)
        #[arg(long)]
        path: String,

        /// Summary text
        #[arg(long)]
        summary: String,

        /// Content body (omit for directory summary; reads stdin if not a TTY)
        #[arg(long)]
        content: Option<String>,

        /// Comma-separated tags
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Recall a memory by path
    Recall {
        /// Logical path
        path: String,

        /// Include YAML frontmatter in output
        #[arg(long)]
        full: bool,
    },

    /// List the memory tree
    Ls {
        /// Subtree path (default: root)
        path: Option<String>,

        /// Max depth to display
        #[arg(long, default_value = "1")]
        depth: usize,
    },

    /// Search across all leaves
    Search {
        /// Case-insensitive substring query
        query: String,
    },

    /// Move a leaf or subtree
    Move {
        /// Source path
        src: String,
        /// Destination path
        dst: String,
    },

    /// Print the entire tree with summaries (no leaf content)
    Inspect,

    /// Delete a leaf or subtree
    Delete {
        /// Path to delete
        path: String,

        /// Force-delete non-empty directories
        #[arg(long)]
        force: bool,
    },
}
