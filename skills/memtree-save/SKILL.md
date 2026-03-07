---
name: memtree-save
description: Persist and restore conversation context using the memtree CLI. Use when the user runs /memtree-save to save current conversation context to disk, or at the start of a new conversation to restore prior context from the memtree tree.
---

# memtree-save

Save conversation context to a memtree, or restore it in a new session. The `memtree` binary must be in PATH. Tree root defaults to `.memtree/` in the current working directory. Override with `--root <path>` or `MEMTREE_ROOT`.

## Save mode (on `/memtree-save`)

1. **Audit context** — identify distinct topics worth persisting: decisions, code snippets, error messages, facts, architectural choices, debugging findings.

2. **Plan tree paths** — choose a logical path for each topic (e.g. `project/auth/oauth-flow`, `debugging/cors-issue`). Reuse existing branches when possible; run `memtree ls --depth 2` first to see what already exists.

3. **Store leaves** — for each topic:
   ```bash
   memtree store --path <path> --summary "<one-line summary>" \
     --content "<full verbatim detail>" --tags <comma-separated>
   ```
   - Content must be the **full verbatim data** (code, error messages, decisions with rationale) — not an abridged summary.
   - Use tags for cross-cutting concerns (e.g. `--tags auth,debugging`).

4. **Store directory summaries** — for any new parent directories:
   ```bash
   memtree store --path <dir> --summary "<short navigational description>"
   ```

5. **Restructure if needed** — freely use `memtree move <src> <dst>` or `memtree delete --force <path>` to reorganize.

6. **Confirm** — run `memtree ls --depth 1` and print the top-level tree to the user.

## Restore mode (start of new conversation)

1. Run `memtree ls --depth 1` to load all top-level nodes with summaries.
2. Print the tree so the user can see what's available.
3. Drill deeper on demand with `memtree ls <path> --depth 1` or `memtree recall <path>`.
4. Level 1 load is mandatory; deeper levels are on-demand only.
