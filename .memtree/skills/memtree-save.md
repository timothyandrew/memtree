---
summary: memtree-save skill design and location
created: '2026-03-09T14:53:54.440556Z'
updated: '2026-03-09T14:54:05.763458Z'
tags:
- skills
- memtree-save
---

Created at skills/memtree-save/SKILL.md (commit c7211a4). Slash command for Claude Code that has two modes:

SAVE MODE (/memtree-save):
1. Audit current conversation context for distinct topics
2. Plan tree paths, reusing existing branches
3. Store leaves with full verbatim content (not abridged) via memtree store
4. Store directory summaries for new parent dirs
5. Optionally restructure with move/delete
6. Confirm with memtree ls --depth 1

RESTORE MODE (start of new conversation):
1. Run memtree ls --depth 1 to load top-level nodes
2. Print tree to user
3. Drill deeper on demand

All commands use --root .memtree for CWD-local storage.
