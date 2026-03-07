---
name: memtree-load
description: Load relevant context from the memtree based on a user prompt. Use when the user runs /memtree-load followed by a description of what they're working on or need context for. Searches and retrieves matching memories from the local .memtree directory.
---

# memtree-load

Load context from the local memtree that is relevant to the user's prompt. The `memtree` binary must be in PATH. All commands use `--root .memtree` (CWD-local tree).

## Usage

The user provides a prompt describing what they need context for:
```
/memtree-load working on the OAuth integration
```

## Procedure

1. **Get the tree overview** — run `memtree --root .memtree ls --depth 2` to see the full structure with summaries.

2. **Search by keywords** — extract key terms from the user's prompt and run `memtree --root .memtree search <term>` for each. This finds leaves whose content or summary matches.

3. **Identify relevant paths** — from the tree overview and search results, identify all paths that are relevant to the user's prompt. Be inclusive — load anything that might be useful, not just exact matches.

4. **Recall full content** — for each relevant path, run `memtree --root .memtree recall <path>` to load the full leaf content.

5. **Synthesize and present** — present the loaded context to the user in a structured summary:
   - Group by topic/branch
   - Highlight the most relevant pieces first
   - Note any gaps where context might be missing
   - List the paths that were loaded so the user knows what was pulled in

## Guidelines

- Cast a wide net: if a branch looks partially relevant, load it. The cost of loading extra context is low compared to missing something important.
- If the tree is empty or `--root .memtree` doesn't exist, tell the user and suggest using `/memtree-save` first.
- If nothing relevant is found, say so and show the top-level tree so the user can browse manually.
