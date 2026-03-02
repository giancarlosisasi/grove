# User Story: Skip Dynamic Imports

## Story

**As a** developer,
**I want to** exclude dynamic imports (`import()`) from circular dependency detection,
**So that** I only see cycles caused by static imports, which are the ones that actually cause runtime initialization issues.

## Command

```
grove circular ./src/index.ts --skip-dynamic-imports
```

With more control:

```
grove circular ./src/index.ts --skip-dynamic-imports circular   # skip only in cycle detection
grove circular ./src/index.ts --skip-dynamic-imports tree        # skip only in tree display
grove circular ./src/index.ts --skip-dynamic-imports all         # skip everywhere
```

## Why This Matters

Dynamic imports (`const mod = import('./module')`) are fundamentally different from static imports:

- **Static imports** execute at module load time. If A statically imports B and B statically imports A, one of them will get an incomplete module during initialization → runtime bug.
- **Dynamic imports** execute lazily when the `import()` call runs. By the time it executes, all modules are already initialized → no initialization-order problem.

Many projects intentionally use dynamic imports for code splitting, lazy loading, or breaking potential circular dependencies. These should not be flagged.

## Modes

| Value | Effect |
|---|---|
| `circular` | Dynamic imports are excluded from cycle detection only. They still appear in the dependency tree. |
| `tree` | Dynamic imports are excluded from the tree display only. They still participate in cycle detection. |
| `all` | Dynamic imports are excluded from both tree and cycle detection. |

Default (no flag): dynamic imports are included everywhere.

## Expected Behavior

### Without `--skip-dynamic-imports`
```
Circular Dependencies (2)
  1) src/a.ts → src/b.ts → src/a.ts          (static cycle)
  2) src/x.ts → src/lazy.ts → src/x.ts       (cycle via dynamic import)
```

### With `--skip-dynamic-imports circular`
```
Circular Dependencies (1)
  1) src/a.ts → src/b.ts → src/a.ts          (static cycle)
```

The dynamic import cycle is not reported because `import('./lazy')` edges are excluded from cycle detection.

## Acceptance Criteria

- [ ] `--skip-dynamic-imports` flag is supported
- [ ] Value `circular` excludes dynamic imports from cycle detection only
- [ ] Value `tree` excludes dynamic imports from tree display only
- [ ] Value `all` excludes dynamic imports from everything
- [ ] No value (just `--skip-dynamic-imports`) defaults to `all`
- [ ] Static imports, CommonJS requires, and re-exports are never affected
- [ ] The output indicates when dynamic imports are being skipped

## How to Test

1. Create:
   - `src/a.ts` with `import('./b')` (dynamic)
   - `src/b.ts` with `import './a'` (static)
2. This is a cycle: `a → b → a`
3. Run `grove circular ./src/a.ts` → cycle detected
4. Run `grove circular ./src/a.ts --skip-dynamic-imports circular` → no cycle (the `a→b` edge is dynamic, so it's excluded, breaking the cycle)

## Edge Cases

- File with ONLY dynamic imports → no edges in cycle detection when skipping, never part of a cycle
- Mix of static and dynamic imports to the same file → the static import still creates an edge
- Dynamic import with non-literal path: `import(variable)` → already unresolvable, never in the graph
