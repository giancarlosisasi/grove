# User Story: Project Files Argument

## Story

**As a** developer,
**I want to** define which files are considered "part of my project,"
**So that** Grove only reports unused files within the scope I care about and doesn't flag vendor files, generated files, or other non-project code.

## Command

```
grove unused --project "src/**/*.ts" --project "lib/**/*.ts"
```

## Why This Is Needed

The "unused files" detection works by comparing files in the project against files in the module graph. The `--project` flag defines what "files in the project" means. Without it, Grove might flag generated files, build outputs, or scripts that are intentionally standalone.

## Expected Behavior

1. `--project` defines the universe of files that are candidates for "unused file" detection
2. When provided, it **replaces** the default project patterns
3. A file is reported as "unused" only if:
   - It matches a `--project` pattern, AND
   - It is NOT reachable from any entry point in the module graph
4. Files that don't match any `--project` pattern are completely ignored (not scanned, not reported)
5. Default: `**/*.{js,ts,jsx,tsx,mjs,cjs,mts,cts}` (excluding `node_modules`, `.git`, `.gitignore`)

## Examples

```bash
# Only check src directory
grove unused --project "src/**/*.ts"

# Check multiple directories
grove unused --project "src/**/*.{ts,tsx}" --project "lib/**/*.ts"

# Exclude test files from project scope
grove unused --project "src/**/*.ts" --project "!src/**/*.test.ts"
```

## Acceptance Criteria

- [ ] `--project` flag accepts glob patterns
- [ ] `--project` can be specified multiple times
- [ ] When `--project` is provided, default patterns are not used
- [ ] Only files matching `--project` patterns are candidates for "unused" reporting
- [ ] Files outside `--project` scope are completely ignored
- [ ] Negation patterns (with `!` prefix) exclude files from the project scope
- [ ] The module graph is still built by following imports (even into files outside `--project` scope) — only the "unused" comparison is limited to project files

## How to Test

1. Create a project:
   - `src/index.ts` imports `src/used.ts`
   - `src/unused.ts` exists, not imported
   - `scripts/build.ts` exists, not imported (intentionally standalone)
2. Run `grove unused --project "src/**/*.ts"`
3. Verify `src/unused.ts` IS reported as unused
4. Verify `scripts/build.ts` is NOT reported (outside project scope)

## Edge Cases

- `--project` pattern matches zero files → warning, no results
- `--project` overlapping with `--entry` → entry files are never reported as unused regardless
- Import path leads outside project scope → the import is still followed for graph building, but the target file is not flagged as unused
