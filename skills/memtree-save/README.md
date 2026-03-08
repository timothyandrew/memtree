# memtree-save

Claude Code skill that persists conversation context to the local `.memtree/` directory.

## Usage

```
/memtree-save
```

When invoked, the agent:

1. Checks the existing tree with `memtree inspect`
2. Audits the entire conversation for every piece of information — decisions, code changes, errors, architecture, commands, config, dependencies, tests, commits, debugging insights, and unfinished work
3. Stores each topic as a leaf with full verbatim content (not summarized)
4. Adds directory summaries for navigability
5. Restructures the tree if needed
6. Writes a `## Memtree` section to `CLAUDE.md` with `@import` syntax so future sessions auto-discover the tree
7. Prints the final tree

## Example

```
> /memtree-save

Checking existing tree...
architecture/            Architecture and design patterns
  concurrency.md           Flock for writes, no lock for reads
  on-disk-format.md        On-disk format for leaves and directories
commands/                CLI command details
  store.md                 Create/update leaves or directory summaries
  recall.md                Print leaf body or directory info
decisions/               Key design decisions
  cwd-local-root.md        Tree root is .memtree/ in CWD

28 leaves across 8 branches.
```

## Install

Copy or symlink this directory into your project's `skills/` folder. Requires the `memtree` binary in PATH (`cargo install --path .` from the repo root).
