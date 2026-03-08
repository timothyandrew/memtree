---
summary: Skill file format and conventions used
created: '2026-03-09T15:01:43.821923Z'
updated: '2026-03-09T15:01:43.821923Z'
tags:
- decisions
- skills
- format
---

Skills are stored in skills/<name>/SKILL.md in the project root.

SKILL.md format:
- YAML frontmatter with 'name' and 'description' fields (required)
- description is the primary trigger — must explain both what the skill does AND when to use it
- Markdown body with procedural instructions

The skills appear in Claude Code's available skills list and are invoked via /<name> slash command.

Design choice: single SKILL.md file per skill, no scripts/references/assets needed — both memtree-save and memtree-load are purely procedural instructions for Claude.

Skill creator reference at: /Users/tim/.claude/skills/skill-creator/SKILL.md
Example skills at: /Users/tim/.claude/skills/
