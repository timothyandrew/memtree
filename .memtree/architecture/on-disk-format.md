---
summary: On-disk format for leaves and directories
created: '2026-03-09T15:00:32.764184Z'
updated: '2026-03-09T15:00:32.764184Z'
tags:
- architecture
- format
---

Tree root directory (default: .memtree/ in CWD):
<root>/
├── _summary.md              # root summary (plain text)
├── rust/
│   ├── _summary.md          # directory summary, plain text
│   ├── errors.md            # leaf with YAML frontmatter + body
│   └── async/
│       ├── _summary.md
│       └── tokio.md
└── python/
    ├── _summary.md
    └── decorators.md

Leaf file format (YAML frontmatter + markdown body):
---
summary: "Rust error handling patterns"
created: 2026-03-09T12:00:00Z
updated: 2026-03-09T12:00:00Z
tags: [rust, errors]
---

Use thiserror for library errors
and anyhow for application errors.

Directory summaries (_summary.md) are plain text, first line used in ls output.
Hidden files (dotfiles) and _summary.md are excluded from ls/inspect listings.
