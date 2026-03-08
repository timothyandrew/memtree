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

### architecture/ — Architecture and design patterns (format, concurrency, validation)
@.memtree/architecture/_summary.md
- auto-promotion — Leaf becomes directory when storing nested path @.memtree/architecture/auto-promotion.md
- concurrency — Flock for writes, no lock for reads @.memtree/architecture/concurrency.md
- on-disk-format — On-disk format for leaves and directories @.memtree/architecture/on-disk-format.md
- path-validation — Path validation rules for logical paths @.memtree/architecture/path-validation.md
- stdin-content — Stdin content reading via --content - flag @.memtree/architecture/stdin-content.md

### commands/ — CLI command details (store, recall, ls, inspect, search, move, delete)
@.memtree/commands/_summary.md
- delete — Remove a leaf or subtree @.memtree/commands/delete.md
- ls-and-inspect — Tree listing commands @.memtree/commands/ls-and-inspect.md
- move — Move a leaf or subtree @.memtree/commands/move.md
- recall — Print leaf body or directory info @.memtree/commands/recall.md
- search — Case-insensitive substring search across leaves @.memtree/commands/search.md
- store — Create/update leaves or directory summaries @.memtree/commands/store.md

### commits/ — Git commit history and session logs
@.memtree/commits/_summary.md
- session-history — Commits made in this session @.memtree/commits/session-history.md

### decisions/ — Key design and architecture decisions
@.memtree/decisions/_summary.md
- cwd-local-root — Tree root is .memtree/ in CWD, not ~/.memtree/ @.memtree/decisions/cwd-local-root.md
- inspect-implementation — Inspect implemented as ls with unlimited depth @.memtree/decisions/inspect-implementation.md
- skill-format — Skill file format and conventions used @.memtree/decisions/skill-format.md

### project/ — Project overview, dependencies, and development workflow
@.memtree/project/_summary.md
- dependencies — Rust crate dependencies @.memtree/project/dependencies.md
- overview — memtree project overview and purpose @.memtree/project/overview.md
- workflow — Development workflow from CLAUDE.md @.memtree/project/workflow.md

### skills/ — Claude Code skills created for this project
@.memtree/skills/_summary.md
- memtree-load — memtree-load skill design and location @.memtree/skills/memtree-load.md
- memtree-save — memtree-save skill design and location @.memtree/skills/memtree-save.md

### source/ — Source code details for each module in src/
@.memtree/source/_summary.md
- cli-rs — src/cli.rs clap derive definitions @.memtree/source/cli-rs.md
- commands-mod — src/commands/mod.rs module declarations @.memtree/source/commands-mod.md
- config-rs — src/config.rs tree root resolution @.memtree/source/config-rs.md
- error-rs — src/error.rs MemtreeError enum @.memtree/source/error-rs.md
- ls-rs — src/commands/ls.rs tree listing with depth control @.memtree/source/ls-rs.md
- main-rs — src/main.rs entrypoint and clap dispatch @.memtree/source/main-rs.md
- other-modules — leaf.rs, tree.rs, lock.rs, summary.rs module purposes @.memtree/source/other-modules.md

### testing/ — Unit and integration test details
@.memtree/testing/_summary.md
- integration-tests — Integration tests in tests/smoke.rs @.memtree/testing/integration-tests.md
- unit-tests — Unit tests in src/ modules @.memtree/testing/unit-tests.md
