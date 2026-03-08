---
summary: Unit tests in src/ modules
created: '2026-03-09T15:01:19.837736Z'
updated: '2026-03-09T15:01:19.837736Z'
tags:
- testing
- unit-tests
---

Unit tests are inline in source modules (mod tests):

src/leaf.rs tests:
- roundtrip: parse -> serialize -> parse produces same result
- update_preserves_created: updating a leaf preserves original created timestamp

src/tree.rs tests:
- valid_paths: tests that valid path strings pass validation
- invalid_paths: tests that .., leading /, _summary, empty components are rejected
- test_leaf_path: verifies logical path -> filesystem path resolution
- test_auto_promote: verifies leaf-to-directory promotion logic

Total: 6 unit tests
Run with: cargo test (runs both unit + integration)
