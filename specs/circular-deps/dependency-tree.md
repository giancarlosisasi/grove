# User Story: Dependency Tree Display

## Story

**As a** developer,
**I want to** see the full dependency tree of my project,
**So that** I can understand the import structure and identify where dependencies come from.

## Command

```
grove circular ./src/index.ts --tree
```

To show ONLY the tree (no circular detection output):

```
grove circular ./src/index.ts --tree --no-circular
```

To suppress the tree (only show circulars):

```
grove circular ./src/index.ts --no-tree
```

## Expected Output

An indented tree showing the import hierarchy from entry points:

```
src/index.ts
├── [S] src/app.ts
│   ├── [S] src/components/Header.tsx
│   │   └── [S] src/utils/format.ts
│   ├── [S] src/components/Footer.tsx
│   └── [D] src/pages/About.tsx
├── [S] src/config.ts
│   └── [C] src/env.ts
└── [S] src/types/index.ts
```

### Import Kind Indicators

| Indicator | Meaning |
|---|---|
| `[S]` | Static import (`import x from '...'`) |
| `[D]` | Dynamic import (`import('...')`) |
| `[C]` | CommonJS require (`require('...')`) |
| `[E]` | Static re-export (`export { x } from '...'`) |

### Circular Reference Indicator

When a file in the tree creates a circular reference, it is marked:

```
src/a.ts
├── [S] src/b.ts
│   └── [S] src/a.ts (circular)
└── [S] src/c.ts
```

The `(circular)` marker indicates the tree stops here because following this import would loop back.

## Behavior

1. Tree is built from the dependency graph
2. Each file appears with its children (the files it imports)
3. If a file has already been shown in the current branch → show it with `(circular)` and stop recursion
4. If a file has already been fully expanded in another branch → show it with `(already shown)` and stop recursion to avoid redundant output
5. Tree lines use box-drawing characters (`├──`, `└──`, `│`) for clean visual hierarchy

## Default Behavior

- `--tree` is **disabled** by default (unlike dpdm which shows it by default)
- When enabled, the tree is printed BEFORE circular dependencies
- Both tree and circular output can be shown together

## Acceptance Criteria

- [ ] `--tree` flag enables dependency tree output
- [ ] `--no-tree` explicitly disables it (redundant since off by default, but explicit)
- [ ] Tree shows correct import hierarchy from entry points
- [ ] Each import shows its kind indicator (`[S]`, `[D]`, `[C]`, `[E]`)
- [ ] Circular references are marked with `(circular)`
- [ ] Already-expanded files are marked with `(already shown)` to avoid duplication
- [ ] Box-drawing characters create clean visual hierarchy
- [ ] Multiple entry points each get their own tree root

## How to Test

1. Create:
   - `src/index.ts` imports `src/a.ts` and `src/b.ts`
   - `src/a.ts` imports `src/c.ts`
   - `src/b.ts` imports `src/c.ts`
2. Run `grove circular ./src/index.ts --tree`
3. Verify tree shows `index.ts` with children `a.ts` and `b.ts`
4. `c.ts` appears under `a.ts` fully, and under `b.ts` with `(already shown)`

## Edge Cases

- Very deep trees (50+ levels) → still displayed, but consider a `--depth` flag in the future
- File with many imports → all shown as children
- Entry file with no imports → just the root node, no children
- Tree combined with circular → tree printed first, then circular section
