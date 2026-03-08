---
summary: Rust crate dependencies
created: '2026-03-09T14:59:45.736034Z'
updated: '2026-03-09T14:59:45.736034Z'
tags:
- project
- dependencies
- rust
---

Runtime dependencies:
- clap (with derive feature) — CLI argument parsing
- serde_yml — YAML frontmatter parsing/serialization
- thiserror — ergonomic error type derivation
- fs4 — file locking (flock) for concurrent write safety

Dev/test dependencies:
- assert_cmd — integration testing of CLI binaries
- predicates — assertion matchers for stdout/stderr
- tempfile — temporary directories for test isolation

Cargo.toml is at project root. Build targets: single binary 'memtree'.
