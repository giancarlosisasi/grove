# User Story: Detect Unused Files

## Story

**As a** developer,
**I want to** find files in my project that are not imported by any other file,
**So that** I can safely delete them and reduce codebase clutter.

## Command

```
grove unused
```

(Unused file detection is part of the default `grove unused` output)

Or filtered to show only unused files:

```
grove unused --include files
```

## How It Works

1. **Discover all project files** — Walk the project directory and collect all files matching `--project` patterns (default: `**/*.{js,ts,jsx,tsx,mjs,cjs,mts,cts}`)
2. **Build the module graph** — Starting from entry files, parse each file, follow all imports/requires recursively. Collect every file that is reachable.
3. **Compare** — Any file that:
   - Matches the project patterns, AND
   - Is NOT in the set of reachable files from entry points
   → is reported as an **unused file**

## Expected Output

```
Unused Files (3)
  src/utils/old-helper.ts
  src/components/DeprecatedButton.tsx
  src/legacy/migration.js
```

## What Counts as "Reachable"

A file is reachable if ANY of these are true:
- It is an entry file
- It is imported (statically) by a reachable file: `import x from './file'`
- It is required by a reachable file: `const x = require('./file')`
- It is dynamically imported by a reachable file: `import('./file')`
- It is re-exported by a reachable file: `export { x } from './file'`

## What Does NOT Count as "Reachable"

- Files only referenced in comments
- Files only referenced in strings (not import/require calls)
- Files only referenced in non-JS config files (unless a plugin handles them)
- Files whose path appears in a variable (dynamic path, not a literal string)

## Acceptance Criteria

- [ ] All files matching project patterns are collected
- [ ] The module graph is built from entry files by following all import types
- [ ] Files not in the graph are reported as unused
- [ ] Entry files themselves are never reported as unused
- [ ] `--include files` filters output to show only unused files
- [ ] Results are sorted alphabetically by file path
- [ ] The count of unused files is shown in the header

## How to Test

### Basic Test
1. Create:
   - `src/index.ts` with `import { helper } from './helper'`
   - `src/helper.ts` (imported by index)
   - `src/orphan.ts` (not imported by anything)
2. Run `grove unused`
3. `src/orphan.ts` appears in unused files
4. `src/index.ts` and `src/helper.ts` do NOT appear

### Dynamic Import Test
1. Create:
   - `src/index.ts` with `const mod = import('./lazy')`
   - `src/lazy.ts` (dynamically imported)
   - `src/dead.ts` (not imported)
2. Run `grove unused`
3. `src/lazy.ts` should NOT be reported (it's dynamically imported)
4. `src/dead.ts` should be reported

### Re-export Test
1. Create:
   - `src/index.ts` with `export { foo } from './foo'`
   - `src/foo.ts` (re-exported from index)
   - `src/bar.ts` (not imported)
2. Run `grove unused`
3. `src/foo.ts` should NOT be reported
4. `src/bar.ts` should be reported

## Edge Cases

- File imported only by another unused file → both are unused (the entire disconnected subgraph is unused)
- File that imports itself → still considered reachable if it's imported by another reachable file
- Empty file (no imports, no exports) → still tracked, reported as unused if not imported
- File with only side effects (no exports, just code) → still tracked as a node in the graph if imported via `import './file'`
- Circular import chain disconnected from entry points → all files in the chain are unused
