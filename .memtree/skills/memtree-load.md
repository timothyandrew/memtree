---
summary: memtree-load skill design and location
created: '2026-03-09T14:54:11.075202Z'
updated: '2026-03-09T15:35:27.502375Z'
tags:
- skills
- memtree-load
---

Located at skills/memtree-load/SKILL.md. Slash command that takes a user prompt and loads relevant context from the local memtree.

Usage: /memtree-load <description of what you need context for>

PROCEDURE:
1. Get tree overview with memtree ls --depth 2
2. Extract key terms from user prompt, run memtree search for each
3. Identify all relevant paths (be inclusive)
4. Recall full content for each relevant path
5. Present structured summary grouped by topic, highlight most relevant first, note gaps

GUIDELINES:
- Cast wide net — load anything partially relevant
- If tree empty, suggest /memtree-save first
- If nothing relevant found, show top-level tree for manual browsing

Also has README at skills/memtree-load/README.md with usage examples.
