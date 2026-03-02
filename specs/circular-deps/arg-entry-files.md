# User Story: Circular Dependency Entry Files

## Story

**As a** developer,
**I want to** specify which files to start scanning from,
**So that** I can analyze a specific part of my project or use non-standard entry points.

## Command

```
grove circular ./src/index.ts
grove circular "./src/**/*.ts"
grove circular ./src/app.ts ./src/worker.ts
```

## Expected Behavior

1. Entry files are specified as positional arguments after the `circular` subcommand
2. Each argument can be a file path or a glob pattern
3. Multiple arguments are supported — all matched files become entry points
4. Glob patterns are expanded against the file system
5. From each entry file, the tool recursively follows imports to discover the full dependency graph
6. Circular detection runs on the combined graph of all entry points

## Acceptance Criteria

- [ ] Positional arguments are parsed as entry files
- [ ] File paths are resolved relative to the current working directory
- [ ] Glob patterns are expanded (e.g., `**/*.ts` matches all TypeScript files recursively)
- [ ] Multiple entry files are all used as starting points
- [ ] Duplicate files (from overlapping globs) are deduplicated
- [ ] Non-existent file paths produce a clear error
- [ ] Globs matching zero files produce a warning

## How to Test

1. Create a project with `src/a.ts → src/b.ts → src/a.ts` (cycle) and `lib/x.ts → lib/y.ts` (no cycle)
2. Run `grove circular ./src/a.ts` → reports the cycle in src
3. Run `grove circular ./lib/x.ts` → no cycles found
4. Run `grove circular "./src/**/*.ts" "./lib/**/*.ts"` → reports only the src cycle

## Edge Cases

- Quoted vs unquoted globs: `"./src/**/*.ts"` (shell doesn't expand) vs `./src/**/*.ts` (shell might expand) → recommend quoting in docs
- Entry file that imports nothing → valid graph with one node, no cycles possible
- All entry files form one big cycle → reported as one cycle
