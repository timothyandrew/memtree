---
summary: Development workflow from CLAUDE.md
created: '2026-03-09T14:59:48.373688Z'
updated: '2026-03-09T14:59:48.373688Z'
tags:
- project
- workflow
---

After every change:
1. Run cargo test (runs both unit and integration/smoke tests — all must pass)
2. Update README.md if the change affects user-facing behavior, commands, or flags
3. Commit and push

Commands:
  cargo build            # dev build
  cargo build --release  # release build
  cargo test             # run all tests (unit + integration)
  cargo test --test smoke  # integration tests only
