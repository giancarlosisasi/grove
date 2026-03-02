# User Story: Transform TypeScript (Strip Type-Only Imports)

## Story

**As a** developer using TypeScript,
**I want to** exclude type-only imports from circular dependency detection,
**So that** I only see cycles that exist at runtime, not cycles caused by type imports that are erased during compilation.

## Command

```
grove circular ./src/index.ts --transform
```

Short form:

```
grove circular ./src/index.ts -T
```

## Why This Matters

TypeScript has type-only imports that are completely erased during compilation:

```typescript
import type { User } from './models/user';     // erased at compile time
import { UserService } from './services/user';   // exists at runtime
```

A cycle through type-only imports is harmless because the import doesn't exist in the compiled JavaScript. It cannot cause initialization-order bugs.

**Example of a harmless cycle:**
```
models/user.ts --[type import]--> services/auth.ts --[static import]--> models/user.ts
```

If `models/user.ts` only uses `import type { AuthService }`, this cycle doesn't exist at runtime.

## What Gets Stripped

When `--transform` is enabled:

| Import Statement | Stripped? |
|---|---|
| `import type { X } from './y'` | Yes |
| `import { type X } from './y'` | Only the `type X` part. If other non-type imports remain, the import stays. |
| `import { X } from './y'` | No |
| `import X from './y'` | No |
| `const X = require('./y')` | No |
| `export type { X } from './y'` | Yes |
| `export { type X } from './y'` | Only the `type X` part |

## Expected Behavior

### Without `--transform`
```
Circular Dependencies (2)
  1) src/models/user.ts → src/services/auth.ts → src/models/user.ts
  2) src/a.ts → src/b.ts → src/a.ts
```

### With `--transform`
```
Circular Dependencies (1)
  1) src/a.ts → src/b.ts → src/a.ts
```

The first cycle disappears because the import from `user.ts` to `auth.ts` was type-only.

## How It Works

1. When parsing each file, identify which imports are type-only
2. Mark those import edges as "type-only" in the dependency graph
3. During cycle detection, exclude edges that are type-only
4. Alternatively: before parsing, strip TypeScript type annotations and re-parse — but edge marking is more efficient

The esbuild parser already distinguishes between regular imports and type imports in its ImportRecord structure, so this information is available without extra work.

## Acceptance Criteria

- [ ] `--transform` / `-T` flag is supported
- [ ] Type-only imports (`import type`) are excluded from cycle detection
- [ ] Inline type imports (`import { type X, Y }`) only exclude the type part
- [ ] Type-only re-exports (`export type`) are excluded
- [ ] Regular imports, requires, and dynamic imports are unaffected
- [ ] The tree display (if enabled) can optionally show or hide type imports

## How to Test

### Type-only cycle
1. Create:
   - `src/a.ts` with `import type { B } from './b'`
   - `src/b.ts` with `import { A } from './a'`
2. Run `grove circular ./src/a.ts` → cycle detected
3. Run `grove circular ./src/a.ts --transform` → no cycle (a→b is type-only)

### Mixed imports
1. Create:
   - `src/a.ts` with `import { type TypeB, funcB } from './b'`
   - `src/b.ts` with `import { funcA } from './a'`
2. Run `grove circular ./src/a.ts --transform` → cycle still detected (a→b has a non-type import `funcB`)

## Edge Cases

- File with ONLY type imports → no edges when transform is on, never part of a cycle
- `import type * as X from './y'` → type-only, stripped
- Re-export with mixed type and value: `export { type X, Y } from './z'` → only the Y part creates an edge
- JavaScript files (.js) → `--transform` has no effect (no type imports in JS)
