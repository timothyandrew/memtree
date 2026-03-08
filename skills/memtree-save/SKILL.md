---
name: memtree-save
description: Persist and restore conversation context using the memtree CLI. Use when the user runs /memtree-save to save current conversation context to disk, or at the start of a new conversation to restore prior context from the memtree tree.
---

# memtree-save

Save conversation context to a memtree, or restore it in a new session. The `memtree` binary must be in PATH. Tree root defaults to `.memtree/` in the current working directory. Override with `--root <path>` or `MEMTREE_ROOT`.

## Save mode (on `/memtree-save`)

### 1. Check existing tree

Run `memtree inspect` to see what's already stored. Update or extend existing leaves rather than duplicating.

### 2. Deep audit — extract everything

Go through the entire conversation and extract **every** piece of information, no matter how small. Be exhaustive. Categories to look for:

- **Decisions made** — every choice, with full rationale and alternatives considered
- **Code written or modified** — what changed, in which files, and why
- **Errors encountered** — full error messages, what caused them, how they were resolved
- **Architecture and design** — patterns, data flows, module responsibilities, formats
- **Commands and APIs** — exact syntax, flags, behavior, edge cases
- **Configuration** — settings, env vars, file paths, defaults
- **Dependencies** — libraries, versions, what they're used for
- **Test details** — what's tested, test names, how to run them, coverage gaps
- **Commits** — hashes, messages, what each commit included
- **Facts and context** — project structure, naming conventions, constraints, user preferences
- **Debugging insights** — what was tried, what worked, what didn't
- **Unfinished work** — TODOs, known issues, next steps discussed

Do NOT summarize or abridge. If something was discussed, it goes in the tree.

### 3. Plan tree paths

Organize into logical branches (e.g. `project/`, `architecture/`, `commands/`, `debugging/`). Reuse existing branches. Aim for fine-grained leaves — one topic per leaf, not mega-leaves that cover everything.

### 4. Store leaves

For each topic:
```bash
memtree store --path <path> --summary "<one-line summary>" \
  --content "<full verbatim detail>" --tags <comma-separated>
```

Rules:
- **Content = full verbatim data.** Include exact code, exact error messages, exact command output. Never paraphrase when you can quote.
- **Summaries = one-line navigational aids.** Keep them short and scannable.
- **Tags** for cross-cutting concerns (e.g. `--tags auth,debugging,rust`).

### 5. Store directory summaries

For every directory (new or existing):
```bash
memtree store --path <dir> --summary "<short navigational description>"
```

### 6. Restructure if needed

Use `memtree move <src> <dst>` or `memtree delete --force <path>` to reorganize for clarity.

### 7. Confirm

Run `memtree inspect` and print the full tree to the user with a count of leaves and branches.

## Restore mode (start of new conversation)

1. Run `memtree ls --depth 1` to load all top-level nodes with summaries.
2. Print the tree so the user can see what's available.
3. Drill deeper on demand with `memtree ls <path> --depth 1` or `memtree recall <path>`.
4. Level 1 load is mandatory; deeper levels are on-demand only.
