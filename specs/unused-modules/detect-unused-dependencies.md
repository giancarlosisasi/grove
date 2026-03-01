# User Story: Detect Unused Dependencies

## Story

**As a** developer,
**I want to** find packages listed in my `package.json` `dependencies` that are never imported in my code,
**So that** I can remove them, reduce my `node_modules` size, and keep my dependency list accurate.

## Command

```
grove unused
```

(Unused dependencies are part of the default output)

Or filtered:

```
grove unused --include dependencies
```

## How It Works

1. **Read `package.json`** — Extract the `dependencies` object (key = package name, value = version)
2. **Collect all imports** — From the module graph, collect every import that references a bare specifier (not a relative path):
   - `import express from 'express'` → uses `express`
   - `import { useState } from 'react'` → uses `react`
   - `const fs = require('fs')` → uses `fs` (built-in, skip)
   - `import config from '@myorg/config'` → uses `@myorg/config`
3. **Map imports to packages** — Extract the package name from the import path:
   - `'lodash/get'` → package is `lodash`
   - `'@scope/package/sub'` → package is `@scope/package`
4. **Compare** — A dependency is unused if:
   - It is listed in `package.json` `dependencies`, AND
   - Its package name is never found in any import across the module graph

## Expected Output

```
Unused Dependencies (2)
  lodash
  moment
```

## Bare Specifier vs Relative Path

- **Bare specifier**: `import x from 'package-name'` → this is a dependency reference
- **Relative path**: `import x from './local-file'` → this is a local file, not a dependency
- **Built-in modules**: `import fs from 'fs'`, `import path from 'path'` → these are Node.js built-ins, never reported as unused dependencies

## Package Name Extraction

The package name is the first segment of the import path, with special handling for scoped packages:

| Import path | Package name |
|---|---|
| `'lodash'` | `lodash` |
| `'lodash/get'` | `lodash` |
| `'@scope/package'` | `@scope/package` |
| `'@scope/package/sub/path'` | `@scope/package` |

## Acceptance Criteria

- [ ] `package.json` `dependencies` are read
- [ ] All bare specifier imports are collected from the module graph
- [ ] Package name is correctly extracted (including scoped packages)
- [ ] Node.js built-in modules are excluded from checking
- [ ] Dependencies not found in any import are reported as unused
- [ ] Results are sorted alphabetically
- [ ] `--ignore-deps` can exclude specific packages from reporting

## How to Test

1. Create `package.json` with dependencies: `express`, `lodash`, `react`
2. Create `src/index.ts` with `import express from 'express'`
3. Run `grove unused`
4. `lodash` and `react` appear as unused
5. `express` does NOT appear

## Edge Cases

- Package imported only via deep path: `import get from 'lodash/get'` → `lodash` is used
- Package only used in a file that is itself unused → the dependency IS still considered used (we track all imports in all parsed files, even unused ones). This is debatable — production mode could handle this differently.
- Scoped packages: `@types/` packages → should be detected separately (they're devDependencies, handled in detect-unused-devdeps)
- Peer dependencies: listed in `peerDependencies` → not checked by default (they're expected to be provided by the consumer)
- Optional dependencies: listed in `optionalDependencies` → not checked by default
- Package referenced only in `package.json` scripts → not detected as used (would require script parsing, future enhancement)
