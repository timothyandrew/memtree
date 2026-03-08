---
summary: Stdin content reading via --content - flag
created: '2026-03-09T15:00:50.138291Z'
updated: '2026-03-09T15:00:50.138291Z'
tags:
- architecture
- stdin
---

The store command supports reading content from stdin using --content - flag. This is an explicit opt-in to avoid blocking in pipelines when no stdin is available.

Usage:
  echo 'Memory content' | memtree store --path notes/idea --summary 'An idea' --content -

Design decision: stdin reading requires the explicit '--content -' flag rather than auto-detecting a TTY, to prevent accidental blocking when memtree is used in scripts/pipelines.
