---
summary: src/commands/ls.rs — tree listing with depth control
created: '2026-03-09T15:00:15.506021Z'
updated: '2026-03-09T15:00:15.506021Z'
tags:
- source
- commands
- ls
---

src/commands/ls.rs implements both 'ls' and 'inspect' (via usize::MAX depth).

pub fn run(root, path, depth):
- Validates root is a dir (else NotFound)
- If path provided, validates it and joins with root
- Calls list_dir(base, root, 0, depth)

fn list_dir(dir, root, current_depth, max_depth):
- Returns early if current_depth >= max_depth
- Reads dir entries, filters out dotfiles and _summary.md
- Sorts entries by filename
- For directories: reads _summary.md first line, prints 'name/' + summary, recurses
- For .md files: parses Leaf frontmatter, prints filename + summary
- Indentation: 2 spaces per depth level
- Format: {indent}{name:<25}{summary}
