# User Story: Detect Circular Dependencies

## Story

**As a** developer,
**I want** Grove to find all circular dependency chains in my module graph,
**So that** I can see exactly which files form import loops and decide how to break them.

## Command

```
grove circular ./src/index.ts
```

## How Detection Works

Once the dependency graph is built (files as nodes, imports as directed edges):

1. **Traverse the graph** using depth-first search (DFS)
2. **Track the current path** — the chain of files being visited in the current DFS branch
3. **When an import leads to a file already in the current path** → a cycle is found
4. **Record the cycle** — from the repeated file back to itself through the chain
5. **Continue traversal** — find ALL cycles, not just the first one

## What a Cycle Looks Like

A cycle is a chain of file paths where the last file imports the first:

```
src/a.ts → src/b.ts → src/c.ts → src/a.ts
```

This means:
- `a.ts` imports `b.ts`
- `b.ts` imports `c.ts`
- `c.ts` imports `a.ts` ← this closes the loop

## Output Format

```
Circular Dependencies (3)
  1) src/a.ts → src/b.ts → src/c.ts → src/a.ts
  2) src/x.ts → src/y.ts → src/x.ts
  3) src/models/user.ts → src/services/auth.ts → src/models/user.ts
```

Each cycle is:
- Numbered sequentially (1, 2, 3...)
- Shows the full chain with `→` arrows
- The last file in the chain equals the first (closing the loop)
- File paths are relative to the project root

## Import Types Included

By default, ALL import types are included in cycle detection:

| Import Type | Included | Can be excluded |
|---|---|---|
| Static import (`import x from './y'`) | Yes | No (always included) |
| Dynamic import (`import('./y')`) | Yes | Yes, via `--skip-dynamic-imports` |
| CommonJS require (`require('./y')`) | Yes | No (always included) |
| Re-export (`export { x } from './y'`) | Yes | No (always included) |
| Type-only import (`import type { X }`) | Yes | Yes, via `--transform` |

## Acceptance Criteria

- [ ] All cycles in the graph are detected (not just the first one)
- [ ] Each cycle is reported as a complete chain from a file back to itself
- [ ] Cycles are numbered sequentially
- [ ] File paths are relative to the project root
- [ ] No duplicate cycles in the output (A→B→A and B→A→B are the same cycle)
- [ ] Self-imports (a file importing itself) are detected as a cycle of length 1
- [ ] The count of cycles is shown in the header

## How to Test

### Simple 2-file cycle
1. `a.ts` imports `b.ts`, `b.ts` imports `a.ts`
2. Output: `a.ts → b.ts → a.ts`

### 3-file cycle
1. `a.ts → b.ts → c.ts → a.ts`
2. Output: `a.ts → b.ts → c.ts → a.ts`

### Multiple independent cycles
1. `a.ts ↔ b.ts` and `x.ts ↔ y.ts`
2. Both cycles reported separately

### No cycles
1. Linear chain: `a.ts → b.ts → c.ts`
2. Output: "No circular dependencies found."

### Diamond (no cycle)
1. `a.ts → b.ts`, `a.ts → c.ts`, `b.ts → d.ts`, `c.ts → d.ts`
2. No cycle — `d.ts` is reached by two paths but there's no loop
3. Output: "No circular dependencies found."

### Self-import
1. `a.ts` imports `a.ts`
2. Output: `a.ts → a.ts`

## Edge Cases

- Very large cycles (20+ files) → still displayed fully
- File participating in multiple cycles → appears in each cycle separately
- Cycle detection with thousands of files → must complete in seconds (efficient algorithm)
- The same cycle should not be reported multiple times (deduplicate by normalizing the start node)
