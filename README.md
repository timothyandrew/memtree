# memtree

Filesystem-based memory tree for AI agents. Persist memories on disk in a tree structure grouped by topic — agents keep top-level summaries in context and load deeper levels on demand.

## How It Looks

```mermaid
graph TD
    root["🌳 ~/.memtree/"]
    root --- rust["📂 rust/"]
    root --- python["📂 python/"]
    root --- tools["📂 tools/"]

    rust --- errors["📝 errors.md<br/><i>Error handling patterns</i>"]
    rust --- async["📂 async/"]
    rust --- lifetimes["📝 lifetimes.md<br/><i>Ownership &amp; borrowing rules</i>"]

    async --- tokio["📝 tokio.md<br/><i>Tokio runtime usage</i>"]
    async --- streams["📝 streams.md<br/><i>Async iterator patterns</i>"]

    python --- decorators["📝 decorators.md<br/><i>Decorator patterns</i>"]
    python --- typing["📝 typing.md<br/><i>Type hint cheatsheet</i>"]

    tools --- git["📂 git/"]
    tools --- docker["📝 docker.md<br/><i>Dockerfile best practices</i>"]

    git --- rebase["📝 rebase.md<br/><i>Interactive rebase workflow</i>"]
    git --- hooks["📝 hooks.md<br/><i>Useful git hooks</i>"]

    style root fill:#7c3aed,stroke:#5b21b6,color:#fff,font-weight:bold
    style rust fill:#2563eb,stroke:#1d4ed8,color:#fff,font-weight:bold
    style python fill:#2563eb,stroke:#1d4ed8,color:#fff,font-weight:bold
    style tools fill:#2563eb,stroke:#1d4ed8,color:#fff,font-weight:bold
    style async fill:#0891b2,stroke:#0e7490,color:#fff,font-weight:bold
    style git fill:#0891b2,stroke:#0e7490,color:#fff,font-weight:bold
    style errors fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style lifetimes fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style tokio fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style streams fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style decorators fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style typing fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style docker fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style rebase fill:#ecfdf5,stroke:#10b981,color:#064e3b
    style hooks fill:#ecfdf5,stroke:#10b981,color:#064e3b

    linkStyle default stroke:#94a3b8,stroke-width:2px
```

> **Purple** = tree root · **Blue** = topic directories · **Teal** = subdirectories · **Green** = memory leaves

## Install

```sh
cargo install --path .
```

## Quick Start

```sh
# Store a memory
memtree store --path rust/errors --summary "Error handling patterns" \
  --content "Use thiserror for library errors, anyhow for applications." \
  --tags rust,errors

# Store a directory summary
memtree store --path rust --summary "Rust programming topics"

# List the tree
memtree ls --depth 2
# rust/                    Rust programming topics
#   errors.md              Error handling patterns

# Recall a memory
memtree recall rust/errors

# Search across all memories
memtree search thiserror

# Move
memtree move rust/errors rust/error-handling

# Delete
memtree delete rust/error-handling
```

## Tree Root

Resolved in order:
1. `--root <path>` flag
2. `MEMTREE_ROOT` environment variable
3. `~/.memtree/` (default)

The root is auto-created on first write.

## On-Disk Format

```
<root>/
├── _summary.md              # root summary (plain text)
├── rust/
│   ├── _summary.md          # "Rust programming topics"
│   ├── errors.md            # leaf with YAML frontmatter + body
│   └── async/
│       ├── _summary.md
│       └── tokio.md
└── python/
    ├── _summary.md
    └── decorators.md
```

Leaf files have YAML frontmatter:

```markdown
---
summary: "Rust error handling patterns"
created: 2026-03-09T12:00:00Z
updated: 2026-03-09T12:00:00Z
tags: [rust, errors]
---

Use `thiserror` for library errors
and `anyhow` for application errors.
```

Directory summaries (`_summary.md`) are plain text.

## Commands

| Command | Description | Locks |
|---|---|---|
| `store --path <path> --summary <text> [--content <text>] [--tags t1,t2]` | Create/update a leaf (with content) or directory summary (without) | Yes |
| `recall <path> [--full]` | Print leaf body or directory summary + children | No |
| `ls [path] [--depth N]` | Tree listing with summaries | No |
| `search <query>` | Case-insensitive substring search across all leaves | No |
| `move <src> <dst>` | Move a leaf or subtree | Yes |
| `delete <path> [--force]` | Remove a leaf or subtree | Yes |

## Concurrency

Write commands acquire an exclusive flock on `<root>/.memtree.lock`. Read commands don't lock. Writes are atomic (temp file + rename), so readers never see partial content.

## Auto-Promotion

Storing a nested path under an existing leaf automatically promotes it to a directory. For example, storing at `rust/errors/handling` when `rust/errors.md` exists moves it to `rust/errors/errors.md` and creates the `rust/errors/` directory.

## Piping Content

Use `--content -` to read content from stdin:

```sh
echo "Memory content" | memtree store --path notes/idea --summary "An idea" --content -
```
