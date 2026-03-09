---
summary: memtree-save skill design and location
created: '2026-03-09T14:53:54.440556Z'
updated: '2026-03-09T15:35:22.577247Z'
tags:
- skills
- memtree-save
---

Located at skills/memtree-save/SKILL.md. Slash command for Claude Code.

PROCEDURE (8 steps):
1. Check existing tree with memtree inspect
2. Deep audit — extract EVERY piece of info from conversation (12 categories: decisions, code, errors, architecture, commands, config, dependencies, tests, commits, facts, debugging, unfinished work)
3. Plan tree paths — logical branches, reuse existing, fine-grained leaves
4. Store leaves — full verbatim content, one-line summaries, tags for cross-cutting
5. Store directory summaries
6. Restructure if needed (move/delete)
7. Update CLAUDE.md with ## Memtree section using @import syntax for tree discovery
   - Import _summary.md for every branch
   - Import leaf .md files if CLAUDE.md stays under 200 lines
   - Each entry has inline summary next to the @import
   - Replace existing ## Memtree section if present
8. Confirm with memtree inspect + count

Key rules:
- Content = full verbatim data, never paraphrase
- Summaries = one-line navigational aids
- No restore mode (removed — handled by /memtree-load)

Also has README at skills/memtree-save/README.md with usage examples.
