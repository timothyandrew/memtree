# CLAUDE.md

## Workflow

After every change:
1. Run `cargo test` (runs both unit and integration/smoke tests — all must pass)
2. Update `README.md` if the change affects user-facing behavior, commands, or flags
3. Commit and push

```sh
cargo build            # dev build
cargo build --release  # release build
cargo test             # run all tests (unit + integration)
cargo test --test smoke  # integration tests only
```

## Project Structure

- `src/main.rs` — entrypoint, clap dispatch
- `src/cli.rs` — clap derive definitions
- `src/config.rs` — tree root resolution (--root flag > MEMTREE_ROOT env > .memtree/)
- `src/error.rs` — MemtreeError enum (thiserror)
- `src/lock.rs` — global lockfile via fs4 flock
- `src/tree.rs` — path validation, leaf_path/dir_path helpers, atomic_write, auto_promote
- `src/leaf.rs` — LeafFrontmatter serde, parse/serialize frontmatter+body
- `src/summary.rs` — _summary.md read/write
- `src/commands/` — one file per command: store, recall, ls, search, mv, delete
- `tests/smoke.rs` — integration tests using assert_cmd, runs the actual binary against tempdirs

## Key Design Decisions

- Dirs = branches, `.md` files = leaves. `_summary.md` holds directory descriptions.
- YAML frontmatter (serde_yml) on leaves: summary, created, updated, tags.
- Write commands acquire a global flock; read commands don't lock.
- Atomic writes: temp file + rename.
- Auto-promotion: storing under an existing leaf converts it to a directory (e.g., `rust/errors.md` → `rust/errors/errors.md`).
- Stdin reading requires explicit `--content -` flag to avoid blocking in pipelines.

## Conventions

- Keep error variants in `MemtreeError`; use `thiserror` derive.
- Path validation rejects `..`, leading `/`, `_summary`, and empty components.
- Integration tests use `assert_cmd` + `predicates` + `tempfile`. Each test gets its own tempdir.

## Memtree

Saved conversation context — use `memtree inspect` for full tree, `memtree recall <path>` for leaf content.

@.memtree/architecture/_summary.md
@.memtree/architecture/auto-promotion.md
@.memtree/architecture/concurrency.md
@.memtree/architecture/on-disk-format.md
@.memtree/architecture/path-validation.md
@.memtree/architecture/stdin-content.md
@.memtree/commands/_summary.md
@.memtree/commands/delete.md
@.memtree/commands/ls-and-inspect.md
@.memtree/commands/move.md
@.memtree/commands/recall.md
@.memtree/commands/search.md
@.memtree/commands/store.md
@.memtree/commits/_summary.md
@.memtree/commits/session-history.md
@.memtree/decisions/_summary.md
@.memtree/decisions/cwd-local-root.md
@.memtree/decisions/inspect-implementation.md
@.memtree/decisions/skill-format.md
@.memtree/project/_summary.md
@.memtree/project/dependencies.md
@.memtree/project/overview.md
@.memtree/project/workflow.md
@.memtree/skills/_summary.md
@.memtree/skills/memtree-load.md
@.memtree/skills/memtree-save.md
@.memtree/source/_summary.md
@.memtree/source/cli-rs.md
@.memtree/source/commands-mod.md
@.memtree/source/config-rs.md
@.memtree/source/error-rs.md
@.memtree/source/ls-rs.md
@.memtree/source/main-rs.md
@.memtree/source/other-modules.md
@.memtree/testing/_summary.md
@.memtree/testing/integration-tests.md
@.memtree/testing/unit-tests.md
