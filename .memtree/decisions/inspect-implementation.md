---
summary: Inspect implemented as ls with unlimited depth
created: '2026-03-09T15:01:47.225561Z'
updated: '2026-03-09T15:01:47.225561Z'
tags:
- decisions
- inspect
- implementation
---

The 'inspect' command was implemented by reusing the existing ls command infrastructure rather than writing new traversal code.

Implementation: Command::Inspect dispatches to commands::ls::run(&root, None, usize::MAX)

This means inspect gets all ls formatting for free (indentation, summary display, directory/leaf distinction) with zero code duplication. The only new code was the CLI variant and dispatch arm.
