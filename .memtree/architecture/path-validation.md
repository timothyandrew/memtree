---
summary: Path validation rules for logical paths
created: '2026-03-09T15:00:45.141167Z'
updated: '2026-03-09T15:00:45.141167Z'
tags:
- architecture
- validation
---

Path validation in src/tree.rs validate_path():

Rejects:
- Paths containing '..' (directory traversal)
- Paths starting with '/' (absolute paths)
- Paths containing '_summary' as a component (reserved for directory summaries)
- Paths with empty components (e.g. 'a//b')

Paths are logical (e.g. 'rust/errors') and resolved to filesystem paths by leaf_path() and dir_path() helpers. Leaf paths get .md extension appended. All path-based commands validate before operating.
