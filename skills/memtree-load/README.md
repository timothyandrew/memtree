# memtree-load

Claude Code skill that loads relevant context from the local `.memtree/` directory based on a user prompt.

## Usage

```
/memtree-load <description of what you need context for>
```

When invoked, the agent:

1. Gets the tree overview with `memtree ls --depth 2`
2. Extracts keywords from your prompt and searches with `memtree search`
3. Recalls full content for all matching leaves
4. Presents a structured summary grouped by topic, with the most relevant pieces first

## Examples

```
> /memtree-load working on the OAuth integration

Loaded 5 leaves:
- architecture/auth-flow — OAuth 2.0 flow with PKCE...
- decisions/token-storage — Store refresh tokens in httpOnly cookies...
- debugging/cors-issue — Fixed by adding credentials: include...
```

```
> /memtree-load what error handling patterns do we use

Loaded 3 leaves:
- architecture/error-types — MemtreeError enum with thiserror...
- decisions/error-strategy — Use thiserror for libraries, anyhow for apps...
- commands/store — Error cases: InvalidPath, AlreadyExists...
```

## Install

Copy or symlink this directory into your project's `skills/` folder. Requires the `memtree` binary in PATH (`cargo install --path .` from the repo root).
