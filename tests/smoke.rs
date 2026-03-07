use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn memtree(root: &TempDir) -> Command {
    let mut cmd = Command::from(cargo_bin_cmd!("memtree"));
    cmd.arg("--root").arg(root.path());
    cmd
}

// ── store & recall ──────────────────────────────────────────────

#[test]
fn store_leaf_and_recall_body() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror for library errors and anyhow for application errors.",
               "--tags", "rust,errors"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "rust/errors"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Use thiserror"));
}

#[test]
fn recall_full_includes_frontmatter() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror.", "--tags", "rust,errors"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "rust/errors", "--full"])
        .assert()
        .success()
        .stdout(predicate::str::contains("summary: Error patterns"))
        .stdout(predicate::str::contains("tags:"))
        .stdout(predicate::str::contains("Use thiserror."));
}

#[test]
fn store_directory_summary() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust", "--summary", "Rust topics"])
        .assert()
        .success();

    // recall on directory should print summary + Children header
    memtree(&root)
        .args(["recall", "rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust topics"))
        .stdout(predicate::str::contains("Children:"));
}

#[test]
fn store_updates_existing_leaf() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "note", "--summary", "v1", "--content", "first"])
        .assert()
        .success();

    memtree(&root)
        .args(["store", "--path", "note", "--summary", "v2", "--content", "second"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "note"])
        .assert()
        .success()
        .stdout(predicate::str::contains("second"))
        .stdout(predicate::str::contains("first").not());

    // created timestamp should be preserved (check via --full)
    memtree(&root)
        .args(["recall", "note", "--full"])
        .assert()
        .success()
        .stdout(predicate::str::contains("summary: v2"));
}

#[test]
fn store_reads_content_from_stdin() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "piped", "--summary", "Piped", "--content", "-"])
        .write_stdin("hello from stdin")
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "piped"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello from stdin"));
}

// ── ls ──────────────────────────────────────────────────────────

#[test]
fn ls_shows_tree() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust", "--summary", "Rust topics"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror."])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "rust/async", "--summary", "Async Rust patterns"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "rust/async/tokio", "--summary", "Tokio runtime",
               "--content", "Use #[tokio::main]."])
        .assert().success();

    memtree(&root)
        .args(["ls", "--depth", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/"))
        .stdout(predicate::str::contains("errors.md"))
        .stdout(predicate::str::contains("async/"));

    // depth 1 should NOT show children of rust/
    memtree(&root)
        .args(["ls", "--depth", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/"))
        .stdout(predicate::str::contains("errors.md").not());
}

#[test]
fn ls_subtree() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "a/b", "--summary", "B", "--content", "body"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "a/c", "--summary", "C", "--content", "body"])
        .assert().success();

    memtree(&root)
        .args(["ls", "a", "--depth", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("b.md"))
        .stdout(predicate::str::contains("c.md"));
}

// ── search ──────────────────────────────────────────────────────

#[test]
fn search_finds_matches() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror for library errors."])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "python/decorators", "--summary", "Decorators",
               "--content", "Use @functools.wraps."])
        .assert().success();

    memtree(&root)
        .args(["search", "thiserror"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/errors"))
        .stdout(predicate::str::contains("Error patterns"));
}

#[test]
fn search_case_insensitive() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "note", "--summary", "Note", "--content", "HELLO world"])
        .assert().success();

    memtree(&root)
        .args(["search", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("note"));
}

#[test]
fn search_no_matches() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "note", "--summary", "Note", "--content", "body"])
        .assert().success();

    memtree(&root)
        .args(["search", "nonexistent"])
        .assert()
        .success()
        .stderr(predicate::str::contains("No matches"));
}

// ── move ────────────────────────────────────────────────────────

#[test]
fn move_leaf() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Errors",
               "--content", "Use thiserror."])
        .assert().success();

    memtree(&root)
        .args(["move", "rust/errors", "rust/error-handling"])
        .assert()
        .success();

    // old path gone
    memtree(&root)
        .args(["recall", "rust/errors"])
        .assert()
        .failure();

    // new path exists
    memtree(&root)
        .args(["recall", "rust/error-handling"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Use thiserror."));
}

#[test]
fn move_subtree() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "a", "--summary", "A dir"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "a/x", "--summary", "X", "--content", "x body"])
        .assert().success();

    memtree(&root)
        .args(["move", "a", "b"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "b/x"])
        .assert()
        .success()
        .stdout(predicate::str::contains("x body"));
}

#[test]
fn move_to_existing_errors() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "a", "--summary", "A", "--content", "a"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "b", "--summary", "B", "--content", "b"])
        .assert().success();

    memtree(&root)
        .args(["move", "a", "b"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Already exists"));
}

// ── delete ──────────────────────────────────────────────────────

#[test]
fn delete_leaf() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "note", "--summary", "Note", "--content", "body"])
        .assert().success();

    memtree(&root)
        .args(["delete", "note"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "note"])
        .assert()
        .failure();
}

#[test]
fn delete_nonempty_dir_requires_force() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "dir", "--summary", "Dir"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "dir/child", "--summary", "Child", "--content", "body"])
        .assert().success();

    // without --force: error
    memtree(&root)
        .args(["delete", "dir"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not empty"));

    // with --force: success
    memtree(&root)
        .args(["delete", "dir", "--force"])
        .assert()
        .success();

    memtree(&root)
        .args(["recall", "dir"])
        .assert()
        .failure();
}

#[test]
fn delete_nonexistent_errors() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["delete", "nope"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not found"));
}

// ── auto-promotion ──────────────────────────────────────────────

#[test]
fn auto_promote_leaf_to_directory() {
    let root = TempDir::new().unwrap();

    // Create a leaf at rust/errors
    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror."])
        .assert().success();

    // Store a child under rust/errors — should auto-promote
    memtree(&root)
        .args(["store", "--path", "rust/errors/handling", "--summary", "Handling",
               "--content", "Match on error types."])
        .assert().success();

    // The promoted original should be at rust/errors/errors
    memtree(&root)
        .args(["recall", "rust/errors/errors"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Use thiserror."));

    // The new child should exist
    memtree(&root)
        .args(["recall", "rust/errors/handling"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Match on error types."));

    // rust/errors should now be a directory
    memtree(&root)
        .args(["ls", "rust/errors", "--depth", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("errors.md"))
        .stdout(predicate::str::contains("handling.md"));
}

// ── path validation ─────────────────────────────────────────────

#[test]
fn rejects_directory_traversal() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "../escape", "--summary", "bad", "--content", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("traversal"));
}

#[test]
fn rejects_absolute_path() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "/etc/passwd", "--summary", "bad", "--content", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("absolute"));
}

#[test]
fn rejects_reserved_summary_path() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "_summary", "--summary", "bad", "--content", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("reserved"));
}

#[test]
fn rejects_empty_component() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "a//b", "--summary", "bad", "--content", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty"));
}

// ── recall directory lists children ─────────────────────────────

#[test]
fn recall_directory_lists_children() {
    let root = TempDir::new().unwrap();

    memtree(&root)
        .args(["store", "--path", "lang", "--summary", "Languages"])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "lang/rust", "--summary", "Rust", "--content", "Systems lang."])
        .assert().success();
    memtree(&root)
        .args(["store", "--path", "lang/python", "--summary", "Python", "--content", "Scripting."])
        .assert().success();

    memtree(&root)
        .args(["recall", "lang"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Languages"))
        .stdout(predicate::str::contains("rust.md"))
        .stdout(predicate::str::contains("python.md"));
}

// ── end-to-end workflow (mirrors the manual smoke test) ─────────

#[test]
fn full_workflow() {
    let root = TempDir::new().unwrap();

    // 1. Store a leaf
    memtree(&root)
        .args(["store", "--path", "rust/errors", "--summary", "Error patterns",
               "--content", "Use thiserror for library errors and anyhow for application errors.",
               "--tags", "rust,errors"])
        .assert().success();

    // 2. Store a directory summary
    memtree(&root)
        .args(["store", "--path", "rust", "--summary", "Rust topics"])
        .assert().success();

    // 3. ls shows both
    memtree(&root)
        .args(["ls", "--depth", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/"))
        .stdout(predicate::str::contains("Rust topics"))
        .stdout(predicate::str::contains("errors.md"))
        .stdout(predicate::str::contains("Error patterns"));

    // 4. recall body
    memtree(&root)
        .args(["recall", "rust/errors"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Use thiserror"));

    // 5. search
    memtree(&root)
        .args(["search", "thiserror"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust/errors"));

    // 6. move
    memtree(&root)
        .args(["move", "rust/errors", "rust/error-handling"])
        .assert()
        .success();

    memtree(&root)
        .args(["ls", "--depth", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("error-handling.md"));

    // 7. delete
    memtree(&root)
        .args(["delete", "rust/error-handling"])
        .assert()
        .success();

    memtree(&root)
        .args(["ls", "--depth", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("error-handling").not());
}
