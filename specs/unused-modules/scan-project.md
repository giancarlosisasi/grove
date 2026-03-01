# User Story: Scan a Project

## Story

**As a** developer,
**I want to** run `grove unused` in my project directory,
**So that** I get a report of all unused files, exports, and dependencies in my project.

## Command

```
grove unused
```

No arguments required. The tool should work with sensible defaults when run from the root of a JS/TS project.

## Expected Behavior

1. Grove looks for a `package.json` in the current directory to confirm it's a JS/TS project
2. It uses default entry file patterns to find starting points:
   - `index.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`
   - `src/index.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`
   - `main.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`
   - `src/main.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`
   - Files referenced in `package.json` fields: `main`, `bin`, `exports`
3. It uses default project file patterns: `**/*.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`
4. It automatically excludes `node_modules`, `.git`, and files in `.gitignore`
5. It parses all entry files, follows their imports recursively, and builds the module graph
6. It compares the graph against all project files and reports what is unused

## Expected Output (Default Text Format)

```
Unused Files (3)
  src/utils/old-helper.ts
  src/components/DeprecatedButton.tsx
  src/legacy/migration.js

Unused Exports (5)
  src/utils/math.ts: multiply, divide
  src/api/client.ts: debugMode
  src/types/index.ts: LegacyUser, OldConfig

Unused Dependencies (2)
  lodash
  moment

Unused DevDependencies (1)
  @types/express

Found 11 issues.
```

## Acceptance Criteria

- [ ] Running `grove unused` in a project root scans the project and prints results
- [ ] Default entry files are detected automatically (index, main, cli files + package.json fields)
- [ ] Default project patterns cover all standard JS/TS extensions
- [ ] `node_modules` and `.git` are excluded by default
- [ ] `.gitignore` patterns are respected by default
- [ ] If no entry files are found, the tool prints a helpful error message suggesting how to configure entry points
- [ ] If no `package.json` is found, the tool prints a warning but still attempts to scan
- [ ] The scan completes within a reasonable time (< 5 seconds for a project with 1,000 files)
- [ ] A progress indicator is shown during scanning for projects with many files

## How to Test

1. Create a small project with a known structure:
   - `src/index.ts` imports `src/used.ts`
   - `src/used.ts` exists and is imported
   - `src/unused.ts` exists but is NOT imported
   - `package.json` lists `lodash` but no file imports it
2. Run `grove unused`
3. Verify that `src/unused.ts` appears in unused files
4. Verify that `src/used.ts` does NOT appear
5. Verify that `lodash` appears in unused dependencies

## Edge Cases

- Empty project (no files found) → helpful message
- Project with only entry files (nothing unused) → "No issues found" message
- Project with no `package.json` → scan files only, skip dependency check
- Project with TypeScript path aliases → should still resolve correctly if `tsconfig.json` exists
- Binary files, images, etc. → should be ignored by default (only JS/TS extensions scanned)
