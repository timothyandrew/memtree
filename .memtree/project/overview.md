---
summary: memtree project overview and purpose
created: '2026-03-09T14:59:42.006482Z'
updated: '2026-03-09T14:59:42.006482Z'
tags:
- project
- overview
---

memtree is a filesystem-based memory tree CLI for AI agents, written in Rust. It persists memories on disk in a tree structure grouped by topic. Agents keep top-level summaries in context and load deeper levels on demand.

Key metaphor: directories = branches, .md files = leaves. _summary.md files hold directory descriptions. Leaves have YAML frontmatter (summary, created, updated, tags) plus a markdown body.

Install: cargo install --path .
Binary name: memtree
Repository: github.com/timothyandrew/memtree
Crate version: 0.1.0
