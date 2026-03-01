# User Story: Auto-Fix Unused Exports

## Story

**As a** developer,
**I want** Grove to automatically remove unused `export` keywords from my code,
**So that** I can clean up my codebase without manually editing dozens of files.

## Command

```
grove unused --fix
```

Dry run (show what would change without changing):

```
grove unused --fix --dry-run
```

## What Gets Fixed

### Removes `export` keyword from unused exports

**Before:**
```typescript
export function unusedHelper() { ... }
export const UNUSED_CONST = 42;
```

**After:**
```typescript
function unusedHelper() { ... }
const UNUSED_CONST = 42;
```

### Removes unused items from export statements

**Before:**
```typescript
export { used, unused } from './module';
```

**After:**
```typescript
export { used } from './module';
```

### Removes unused re-exports entirely

**Before:**
```typescript
export { unusedA } from './a';
export { unusedB } from './b';
```

**After:** (lines removed entirely)

## What Does NOT Get Fixed

- **Unused files are NOT deleted** — deleting files is too risky for auto-fix. The user should decide.
- **Unused dependencies are NOT removed from `package.json`** — modifying `package.json` could break install scripts.
- **The declaration itself is NOT removed** — only the `export` keyword is removed. The function/variable might have side effects or be used internally within the file.

## Expected Output

```
Fixed 5 unused exports in 3 files:
  src/utils/math.ts
    - Removed export from: multiply (function)
    - Removed export from: divide (function)
  src/api/client.ts
    - Removed export from: debugMode (variable)
  src/types/barrel.ts
    - Removed re-export: LegacyUser
    - Removed re-export: OldConfig
```

## Dry Run

With `--dry-run`, the same output is shown but no files are modified:

```
Would fix 5 unused exports in 3 files:
  src/utils/math.ts
    - Would remove export from: multiply (function)
    ...
```

## Acceptance Criteria

- [ ] `--fix` removes the `export` keyword from unused named exports
- [ ] `--fix` removes unused items from `export { ... }` statements
- [ ] `--fix` removes entire re-export lines when all items are unused
- [ ] `--fix` does NOT delete files
- [ ] `--fix` does NOT modify `package.json`
- [ ] `--fix` does NOT remove declarations (only the `export` keyword)
- [ ] `--dry-run` shows what would change without modifying files
- [ ] Modified files are listed in the output with details of what changed
- [ ] The tool reads the file, modifies in memory, and writes back atomically
- [ ] Original file formatting is preserved as much as possible

## How to Test

1. Create `src/math.ts` with `export function add() {}` and `export function unused() {}`
2. Create `src/index.ts` with `import { add } from './math'`
3. Run `grove unused --fix --dry-run` → shows "Would remove export from: unused"
4. Verify `src/math.ts` is NOT modified
5. Run `grove unused --fix`
6. Verify `src/math.ts` now has `function unused() {}` (no `export`)
7. Run `grove unused` → no unused exports reported

## Edge Cases

- `export default function() {}` (anonymous default export) → remove entire `export default` prefix? This is tricky. Skip anonymous defaults for now.
- `export` on a class with decorators → only remove the `export` keyword, keep decorators
- File with mixed line endings → preserve original line endings
- File with BOM → preserve BOM
- Read-only file → error message, skip, continue with other files
