# User Story: Detect Unused Files (via Circular Command)

## Story

**As a** developer,
**I want to** find files that are not reachable from any entry point while running circular dependency analysis,
**So that** I can get both circular dependencies and unused file information in a single scan.

## Command

```
grove circular ./src/index.ts --detect-unused-files-from "src/**/*.*"
```

## Why This Exists in the Circular Command

The circular dependency command already builds the full dependency graph by traversing all imports from entry points. It already knows which files are reachable. Adding unused file detection on top is essentially free — it just compares the reachable set against a glob pattern.

This is a convenience feature that avoids running two separate commands when you want both circular deps and unused files.

## Expected Behavior

1. The `--detect-unused-files-from` flag accepts a glob pattern
2. After building the dependency graph, Grove collects all files matching the glob
3. Any file matching the glob that is NOT in the dependency graph → reported as unused
4. Unused files are printed in a separate section after circular dependencies

## Expected Output

```
Circular Dependencies (1)
  1) src/a.ts → src/b.ts → src/a.ts

Unused Files (2)
  src/orphan.ts
  src/old-utils.ts

✓ Scanned 47 files in 0.15s
```

## Acceptance Criteria

- [ ] `--detect-unused-files-from` accepts a glob pattern
- [ ] Files matching the glob but NOT in the dependency graph are reported
- [ ] Results appear in a separate "Unused Files" section
- [ ] Works alongside circular detection and tree display
- [ ] JSON output includes an `unusedFiles` array when this flag is used
- [ ] When no unused files are found, the section is omitted

## How to Test

1. Create:
   - `src/index.ts` imports `src/used.ts`
   - `src/used.ts` (imported)
   - `src/orphan.ts` (not imported)
2. Run `grove circular ./src/index.ts --detect-unused-files-from "src/**/*.ts"`
3. `src/orphan.ts` appears in unused files
4. `src/index.ts` and `src/used.ts` do NOT appear

## Edge Cases

- Glob matches zero files → no "Unused Files" section shown
- All files are reachable → "No unused files found"
- Combined with `--no-circular` → only unused files shown (effectively replicating `grove unused` file detection)
- File matched by glob but excluded by `--exclude` → not analyzed, not reported as unused
