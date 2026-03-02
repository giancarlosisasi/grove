# User Story: Scan Entry Point for Circular Dependencies

## Story

**As a** developer,
**I want to** run `grove circular` and get a report of all circular dependencies in my project,
**So that** I can identify and fix import cycles before they cause runtime issues.

## Command

```
grove circular ./src/index.ts
```

Or with glob patterns:

```
grove circular "./src/**/*.ts"
```

Or with no arguments (uses defaults):

```
grove circular
```

## Expected Behavior

1. Accept one or more file paths or glob patterns as positional arguments
2. If no arguments provided, use the same default entry detection as `grove unused` (index.ts, main.ts, package.json fields)
3. Starting from the entry files, parse each file and follow all imports recursively
4. Build a directed dependency graph
5. Detect all cycles in the graph
6. Print the results: circular dependencies, dependency tree (optional), and warnings

## Default Output

```
Circular Dependencies (2)
  1) src/a.ts → src/b.ts → src/c.ts → src/a.ts
  2) src/x.ts → src/y.ts → src/x.ts

Warnings (1)
  Cannot resolve: ./missing-module (imported from src/index.ts)

✓ Scanned 47 files in 0.12s
```

When no circular dependencies are found:

```
No circular dependencies found.

✓ Scanned 47 files in 0.12s
```

## What Gets Scanned

Starting from entry files, Grove follows:
- Static imports: `import x from './y'`
- Dynamic imports: `import('./y')` (unless `--skip-dynamic-imports` is set)
- CommonJS requires: `const x = require('./y')`
- Re-exports: `export { x } from './y'`

Each resolved file is then parsed and its imports are followed, continuing recursively until all reachable files have been visited.

## Acceptance Criteria

- [ ] Positional arguments accept file paths and glob patterns
- [ ] Multiple entry points can be specified
- [ ] If no positional arguments, defaults are used (index.ts, main.ts, etc.)
- [ ] All reachable files are parsed and added to the dependency graph
- [ ] All circular dependencies are detected and reported
- [ ] Unresolvable imports are reported as warnings (not errors)
- [ ] A summary line shows file count and elapsed time
- [ ] Progress indicator is shown for large projects

## How to Test

### Basic Cycle
1. Create:
   - `src/a.ts` with `import './b'`
   - `src/b.ts` with `import './a'`
2. Run `grove circular ./src/a.ts`
3. Output shows: `src/a.ts → src/b.ts → src/a.ts`

### No Cycles
1. Create:
   - `src/a.ts` with `import './b'`
   - `src/b.ts` with `import './c'`
   - `src/c.ts` (no imports)
2. Run `grove circular ./src/a.ts`
3. Output: "No circular dependencies found."

### Multiple Entry Points
1. Create two independent module graphs, one with a cycle
2. Run `grove circular ./src/app1.ts ./src/app2.ts`
3. Only the graph with the cycle reports it

## Edge Cases

- Entry file that doesn't exist → error message, exit code 2
- Glob that matches zero files → warning, exit gracefully
- File with syntax errors → warning, skip that file, continue scanning
- Import that resolves to a file outside the project (node_modules) → stop at the boundary, don't traverse into node_modules
- Very deep import chains → no stack overflow (use iterative DFS, not recursive)
