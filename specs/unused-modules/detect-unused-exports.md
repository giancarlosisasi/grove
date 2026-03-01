# User Story: Detect Unused Exports

## Story

**As a** developer,
**I want to** find exported functions, classes, variables, and types that no other file imports,
**So that** I can remove unnecessary `export` keywords and simplify my module's public API.

## Command

```
grove unused
```

(Unused exports are part of the default output)

Or filtered:

```
grove unused --include exports
```

## How It Works

1. **Parse every reachable file** — For each file in the module graph, extract ALL exports:
   - Named exports: `export function foo()`, `export const bar`, `export { baz }`
   - Default exports: `export default class MyClass`
   - Re-exports: `export { x } from './other'`
   - Type exports: `export type Foo`, `export interface Bar`
2. **Track all imports** — For each file, record exactly which symbols it imports from each module:
   - `import { foo, bar } from './module'` → imports `foo` and `bar`
   - `import baz from './module'` → imports `default`
   - `import * as mod from './module'` → imports all (namespace import — all exports considered used)
   - `import type { Foo } from './module'` → imports type `Foo`
3. **Compare** — An export is unused if:
   - It is exported by a reachable file, AND
   - No other file in the module graph imports that specific symbol

## Expected Output

```
Unused Exports (5)
  src/utils/math.ts
    - multiply (function)
    - divide (function)
  src/api/client.ts
    - debugMode (variable)
  src/types/index.ts
    - LegacyUser (type)
    - OldConfig (interface)
```

## Entry File Exception

By default, exports from **entry files** are NOT reported as unused. This is because entry files define the project's public API — their exports are meant to be consumed by external code (other packages, the runtime, etc.).

This can be changed with `--include-entry-exports` to also check entry file exports.

## Namespace Imports

When a file does `import * as mod from './module'`, ALL exports from `./module` are considered used. We cannot statically determine which specific exports are accessed (e.g., `mod.foo` vs `mod.bar`) without full data-flow analysis, which is out of scope.

## Re-exports

Re-exports (`export { x } from './other'`) are both an import AND an export:
- They count as an import of `x` from `./other`
- They also create an export of `x` from the current file
- If the re-export itself is never imported by anyone, it's reported as unused

## Acceptance Criteria

- [ ] All export types are detected (named, default, re-export, type, interface)
- [ ] All import types are tracked (named, default, namespace, dynamic, type)
- [ ] An export is reported as unused only if no other file imports it
- [ ] Entry file exports are excluded from reporting by default
- [ ] `--include-entry-exports` overrides the entry file exception
- [ ] Namespace imports (`import *`) mark all exports as used
- [ ] Re-exports are tracked as both import and export
- [ ] Output shows the file path, export name, and kind (function, class, variable, type)
- [ ] Results are grouped by file

## How to Test

### Basic Test
1. Create `src/index.ts` with `import { add } from './math'`
2. Create `src/math.ts` with `export function add()`, `export function multiply()`
3. Run `grove unused`
4. `multiply` is reported as unused export
5. `add` is NOT reported

### Type Export Test
1. Create `src/types.ts` with `export type Foo = ...` and `export type Bar = ...`
2. Create `src/index.ts` with `import type { Foo } from './types'`
3. Run `grove unused`
4. `Bar` is reported as unused type export
5. `Foo` is NOT reported

### Namespace Import Test
1. Create `src/utils.ts` with `export function a()`, `export function b()`
2. Create `src/index.ts` with `import * as utils from './utils'`
3. Run `grove unused`
4. Neither `a` nor `b` should be reported (namespace import marks all as used)

### Entry File Test
1. Create `src/index.ts` (entry) with `export function publicApi()` (not imported by anyone)
2. Run `grove unused`
3. `publicApi` is NOT reported (entry file exports are exempt)
4. Run `grove unused --include-entry-exports`
5. `publicApi` IS reported

## Edge Cases

- Export used only within the same file → still reported as unused export (the `export` keyword is unnecessary)
- Default export never imported → reported as unused default export
- Re-export chain: A re-exports from B, B re-exports from C → if nobody imports from A, both the A and B re-exports are unused
- Dynamic import: `import('./module').then(m => m.foo)` → we track that `./module` is imported but cannot determine which specific export is used; treat all exports as used (same as namespace)
- Barrel files (index.ts that re-exports everything) → each re-export is individually tracked
