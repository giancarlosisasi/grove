# User Story: Detect Unused DevDependencies

## Story

**As a** developer,
**I want to** find packages listed in my `package.json` `devDependencies` that are never imported or referenced,
**So that** I can clean up my development setup and speed up installs.

## Command

```
grove unused
```

Or filtered:

```
grove unused --include devDependencies
```

## How It Works

Same logic as unused dependencies detection, but targeting the `devDependencies` field in `package.json`.

1. **Read `package.json`** — Extract the `devDependencies` object
2. **Collect all imports** — From ALL files in the module graph (including test files, config files, scripts)
3. **Compare** — A devDependency is unused if its package name never appears in any import

## Expected Output

```
Unused DevDependencies (2)
  @types/express
  eslint-plugin-unused-imports
```

## Difference from Production Dependencies

DevDependencies are typically used by:
- Test files (`*.test.ts`, `*.spec.ts`)
- Configuration files (`eslint.config.js`, `jest.config.js`)
- Build scripts
- Type definitions (`@types/*`)

Because of this, devDependency checking should analyze ALL files, not just production code. (This contrasts with production mode, which ignores test/config files.)

## `@types/*` Packages

`@types/` packages are a special case:
- `@types/node` is "used" if the project targets Node.js (has Node.js built-in imports)
- `@types/react` is "used" if `react` is in `dependencies`
- General rule: `@types/X` is considered used if package `X` is used in the project

This mapping prevents false positives for type definition packages that are never directly imported.

## Acceptance Criteria

- [ ] `package.json` `devDependencies` are read
- [ ] All imports across all files (including test/config) are checked
- [ ] DevDependencies not found in any import are reported
- [ ] `@types/X` packages are considered used if `X` is used
- [ ] Results are separated from production dependencies in the output
- [ ] `--ignore-deps` also applies to devDependencies

## How to Test

1. Create `package.json` with devDependencies: `jest`, `@types/node`, `eslint-plugin-unused`
2. Create `src/index.test.ts` with `import { describe } from 'jest'` (or `@jest/globals`)
3. Run `grove unused`
4. `eslint-plugin-unused` appears as unused devDependency
5. `jest` does NOT appear
6. `@types/node` does NOT appear (project uses Node.js built-ins)

## Edge Cases

- `@types/` package with no corresponding real package used → reported as unused
- Package only referenced in `package.json` scripts (e.g., `"test": "jest"`) → not detected as used via import analysis; possible future enhancement
- Package that is both a devDependency and provides binaries used in scripts → currently not detected as used without script analysis
