# User Story: Entry Files Argument

## Story

**As a** developer,
**I want to** specify which files are the entry points of my project,
**So that** Grove starts its analysis from the correct roots and doesn't miss any used code paths.

## Command

```
grove unused --entry "src/app.ts" --entry "src/worker.ts"
```

Or with glob patterns:

```
grove unused --entry "src/pages/**/*.tsx"
```

## Why This Is Needed

Not every project uses `index.ts` or `main.ts` as entry points. A Next.js app has `pages/**/*.tsx`, a CLI tool might have `bin/cli.ts`, an Express server might have `server.ts`. Without custom entry points, Grove would miss these files and incorrectly report their dependencies as unused.

## Expected Behavior

1. When `--entry` is provided, it **replaces** the default entry patterns (it does not add to them)
2. Multiple `--entry` flags can be used to specify multiple patterns
3. Each entry value can be a specific file path or a glob pattern
4. Entry files are the starting points — all their imports are followed recursively
5. Exports from entry files are NOT reported as unused by default (they are considered the "public API")

## Examples

```bash
# Single entry point
grove unused --entry "src/server.ts"

# Multiple entry points
grove unused --entry "src/index.ts" --entry "src/worker.ts"

# Glob pattern
grove unused --entry "src/pages/**/*.tsx"

# Mix of specific and glob
grove unused --entry "src/main.ts" --entry "src/routes/**/*.ts"
```

## Acceptance Criteria

- [ ] `--entry` flag accepts a file path or glob pattern
- [ ] `--entry` can be specified multiple times
- [ ] When `--entry` is provided, default entry patterns are not used
- [ ] Glob patterns expand correctly to match actual files
- [ ] If an `--entry` path doesn't match any files, a warning is printed
- [ ] Exports from entry files are not reported as unused by default
- [ ] The entry files themselves are never reported as unused files

## How to Test

1. Create a project where `src/app.ts` is the actual entry (not `index.ts`)
2. `src/app.ts` imports `src/helper.ts`
3. `src/index.ts` exists but is not the real entry
4. Run `grove unused --entry "src/app.ts"`
5. Verify `src/helper.ts` is NOT reported as unused
6. Verify `src/index.ts` IS reported as unused (since it's not reachable from `src/app.ts`)

## Edge Cases

- Entry file that doesn't exist → warning message, continue with other entries
- Glob that matches zero files → warning message
- Entry file with no imports → valid, just means the graph has one node
- Overlapping entry patterns → deduplicate, each file parsed only once
