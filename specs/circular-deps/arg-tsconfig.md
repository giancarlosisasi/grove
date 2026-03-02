# User Story: TSConfig Support

## Story

**As a** developer using TypeScript path aliases,
**I want** Grove to read my `tsconfig.json` and resolve path aliases correctly,
**So that** imports like `@/components/Button` are resolved to the actual file path instead of being treated as unresolvable.

## Command

```
grove circular ./src/index.ts --tsconfig tsconfig.json
```

Or relying on auto-detection:

```
grove circular ./src/index.ts
```

(Automatically uses `tsconfig.json` if it exists in the project root)

## Why This Is Needed

TypeScript projects commonly use path aliases configured in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["src/*"],
      "@components/*": ["src/components/*"],
      "@utils/*": ["src/utils/*"]
    }
  }
}
```

Without tsconfig support, an import like `import { Button } from '@/components/Button'` would be unresolvable — Grove wouldn't know that `@/components/Button` maps to `src/components/Button.tsx`.

## What Gets Read from tsconfig.json

| Field | Purpose |
|---|---|
| `compilerOptions.baseUrl` | Base directory for non-relative module resolution |
| `compilerOptions.paths` | Path alias mappings |
| `compilerOptions.rootDir` | Root directory of source files |
| `extends` | Inherit from another tsconfig file |

## Expected Behavior

1. If `--tsconfig` is provided, use that file
2. If not provided, look for `tsconfig.json` in the project root (current working directory)
3. Parse the tsconfig file, including resolving `extends` chains
4. Use `baseUrl` and `paths` to resolve import specifiers during module resolution
5. If tsconfig doesn't exist and `--tsconfig` wasn't specified, continue without alias resolution (unresolvable aliases will be warnings)

## Resolution Example

Given tsconfig:
```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["src/*"]
    }
  }
}
```

And code:
```typescript
import { helper } from '@/utils/helper';
```

Grove resolves: `@/utils/helper` → `src/utils/helper.ts`

## Acceptance Criteria

- [ ] `--tsconfig` flag accepts a path to a tsconfig file
- [ ] Auto-detects `tsconfig.json` in project root when flag not provided
- [ ] `baseUrl` is used for non-relative module resolution
- [ ] `paths` aliases are resolved correctly (including wildcards)
- [ ] `extends` chains are followed and merged
- [ ] Invalid tsconfig produces a clear error message
- [ ] Missing tsconfig (without `--tsconfig` flag) is not an error — just no alias resolution

## How to Test

1. Create `tsconfig.json` with `paths: {"@/*": ["src/*"]}`
2. Create `src/index.ts` with `import { foo } from '@/utils/foo'`
3. Create `src/utils/foo.ts` with `import { bar } from '@/utils/bar'`
4. Create `src/utils/bar.ts` with `import { index } from '@/index'`
5. Run `grove circular ./src/index.ts`
6. Verify the cycle is detected: `index.ts → foo.ts → bar.ts → index.ts`
7. All imports resolved correctly via path aliases

## Edge Cases

- Multiple path patterns matching the same import → try in order, use first match
- `paths` with multiple mapping targets: `"@/*": ["src/*", "lib/*"]` → try each in order
- `extends` pointing to a node_modules tsconfig: `"extends": "@tsconfig/node18/tsconfig.json"` → resolve from node_modules
- Circular `extends` → error, stop processing
- tsconfig with comments (JSONC) → parse correctly (tsconfig.json supports comments)
