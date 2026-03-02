# Epic: Circular Dependency Detection

## Vision

As a developer working on a JavaScript/TypeScript project, I want to find circular dependencies — situations where file A imports file B, which imports file C, which imports file A again — so that I can break these cycles and avoid hard-to-debug runtime issues like undefined values, initialization order bugs, and module loading failures.

## What Are Circular Dependencies?

A circular dependency occurs when two or more modules form a loop in the import graph:

**Simple cycle (2 files):**
```
a.ts → b.ts → a.ts
```

**Longer cycle (3+ files):**
```
a.ts → b.ts → c.ts → a.ts
```

**Multiple cycles:**
A project can have many independent cycles. Some files may participate in more than one cycle.

## Why They're Problematic

1. **Undefined values at runtime**: When module A imports B, and B imports A, one of them will get an incomplete (partially initialized) version of the other during the first load
2. **Initialization order bugs**: Code that runs at module load time may execute before its dependencies are ready
3. **Bundler issues**: Some bundlers handle circular dependencies poorly, producing broken output
4. **Maintainability**: Circular dependencies make it hard to understand the architecture and refactor safely

## Not All Circular Dependencies Are Equal

- **Static imports** (`import x from './y'`) are the most dangerous — they execute at load time and can cause undefined values
- **Dynamic imports** (`import('./y')`) are usually safe — they execute lazily at runtime when called, so initialization order is not a problem
- **Type-only imports** (`import type { X } from './y'`) are completely safe — they are erased at compile time and don't exist at runtime

This distinction is important: Grove should let users control which import types are included in circular detection.

## How Detection Works (High Level)

1. **Specify entry points** — One or more files or globs as the starting point
2. **Parse and follow imports** — For each file, parse it to find all imports, resolve them to file paths, and recurse
3. **Build the dependency graph** — A directed graph where nodes are files and edges are imports
4. **Find cycles** — Use graph traversal (depth-first search) to detect all cycles in the graph
5. **Report** — Display each cycle as a chain: `A → B → C → A`

## Success Criteria for the Epic

- Can detect all circular dependencies in a JS/TS project
- Reports clear, actionable cycle chains showing the exact file paths
- Supports filtering out dynamic imports and type-only imports
- Fast enough for large projects (thousands of files in seconds)
- Can be integrated into CI with exit codes
- Supports both dependency tree display and circular-only reporting

## User Stories in This Epic

1. [Scan Entry Point](./scan-entry-point.md) — Basic circular detection from entry files
2. [Entry Files Argument](./arg-entry-files.md) — Specify what to scan
3. [Detect Circular Dependencies](./detect-circular.md) — Core cycle detection
4. [Dependency Tree Display](./dependency-tree.md) — Show full dependency tree
5. [Output Format Argument](./arg-output-format.md) — Choose output format
6. [Exclude Argument](./arg-exclude.md) — Exclude files from analysis
7. [Skip Dynamic Imports](./arg-skip-dynamic-imports.md) — Exclude dynamic imports from analysis
8. [Transform TypeScript](./arg-transform-ts.md) — Strip type-only imports
9. [TSConfig Support](./arg-tsconfig.md) — Respect TypeScript path aliases
10. [Detect Unused Files](./detect-unused-files.md) — Bonus: find unreachable files
11. [Exit Codes](./exit-codes.md) — CI integration
