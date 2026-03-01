# User Story: Exclude Patterns for Circular Detection

## Story

**As a** developer,
**I want to** exclude certain files or directories from circular dependency analysis,
**So that** I can focus on the parts of my codebase that matter and ignore test files, generated files, or vendor code.

## Command

```
grove circular ./src/index.ts --exclude "node_modules"
grove circular ./src/index.ts --exclude "**/*.test.ts" --exclude "**/__mocks__/**"
```

Also, an include filter:

```
grove circular ./src/index.ts --include-pattern "src/**"
```

## Expected Behavior

### Exclude (`--exclude`)
1. Accepts a regular expression or glob pattern
2. Files matching the pattern are skipped during traversal
3. If file A imports file B, and B matches `--exclude`, the edge A→B is NOT added to the graph
4. Multiple `--exclude` flags can be used
5. Default: `node_modules` is always excluded

### Include (`--include-pattern`)
1. Accepts a regular expression or glob pattern
2. Only files matching the pattern are analyzed
3. Default: `.*` (all files)

## How Exclusion Affects the Graph

When a file is excluded:
- It is NOT parsed (saving time)
- It does NOT appear in the dependency tree
- Imports TO it are ignored (the edge is dropped)
- This can "break" cycles that pass through excluded files

**Example:**
```
a.ts → b.ts → test-helper.ts → a.ts
```

With `--exclude "**/*test*"`:
- `test-helper.ts` is excluded
- The edge `b.ts → test-helper.ts` is dropped
- The cycle is no longer detected
- This is the intended behavior — the user chose to exclude test files

## Acceptance Criteria

- [ ] `--exclude` accepts patterns (regexp or glob)
- [ ] `--exclude` can be specified multiple times
- [ ] Excluded files are not parsed and not included in the graph
- [ ] `node_modules` is excluded by default
- [ ] `--include-pattern` limits analysis to matching files only
- [ ] Exclusion is applied to resolved file paths (absolute or relative to project root)

## How to Test

1. Create a cycle: `a.ts → b.ts → c.ts → a.ts`
2. Run `grove circular ./src/a.ts` → cycle detected
3. Run `grove circular ./src/a.ts --exclude "c.ts"` → no cycle (c.ts excluded, breaking the chain)
4. Verify `c.ts` does NOT appear in the tree (if `--tree` is enabled)

## Edge Cases

- Exclude pattern matches an entry file → the entry file is still the starting point but its imports to excluded files are dropped
- Exclude everything → empty graph, no cycles
- Include pattern that matches no files → warning, empty results
- Pattern with special regex characters → properly escaped or treated as glob
