---
summary: Integration tests in tests/smoke.rs
created: '2026-03-09T15:01:25.328868Z'
updated: '2026-03-09T15:01:25.328868Z'
tags:
- testing
- integration-tests
- smoke
---

tests/smoke.rs — 24 integration tests using assert_cmd + predicates + tempfile.

Helper: memtree(root: &TempDir) -> Command — creates a Command with --root pointing to tempdir.

Test categories:
- store & recall (5): store_leaf_and_recall_body, recall_full_includes_frontmatter, store_directory_summary, store_updates_existing_leaf, store_reads_content_from_stdin
- ls (2): ls_shows_tree, ls_subtree
- inspect (1): inspect_shows_full_tree
- search (3): search_finds_matches, search_case_insensitive, search_no_matches
- move (3): move_leaf, move_subtree, move_to_existing_errors
- delete (3): delete_leaf, delete_nonempty_dir_requires_force, delete_nonexistent_errors
- auto-promotion (1): auto_promote_leaf_to_directory
- path validation (4): rejects_directory_traversal, rejects_absolute_path, rejects_reserved_summary_path, rejects_empty_component
- recall directory (1): recall_directory_lists_children
- end-to-end (1): full_workflow

Each test gets its own TempDir. Run with: cargo test --test smoke
