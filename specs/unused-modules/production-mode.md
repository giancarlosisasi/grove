# User Story: Production Mode

## Story

**As a** developer,
**I want to** analyze only my production code (excluding tests, dev configs, dev dependencies),
**So that** I can find dead code that affects my production bundle and runtime.

## Command

```
grove unused --production
```

## Why This Is Needed

In default mode, Grove analyzes everything: production code, test files, configuration files, dev scripts. This means a function used only in tests won't be reported as unused.

Production mode is stricter. It only considers production entry files and production dependencies. A function used only in test files WILL be reported as unused in production mode — because from the production bundle's perspective, it is dead code.

## Expected Behavior

When `--production` is enabled:

1. **Entry files**: Only production entries are used (not test configs, not dev scripts)
   - `package.json` `main`, `bin`, `exports` fields
   - Custom `--entry` patterns (assumed to be production)
   - NOT: `*.test.ts`, `*.spec.ts`, `jest.config.*`, `eslint.config.*`, etc.

2. **Dependencies**: Only `dependencies` are checked (not `devDependencies`)
   - `devDependencies` are completely ignored
   - Packages like `jest`, `eslint`, `prettier` won't be reported as unused

3. **Files**: Test files and config files are excluded from the project scope
   - Files matching common test patterns are not analyzed
   - This means functions used ONLY in tests will be reported as unused exports

## Default Test File Patterns (excluded in production mode)

```
**/*.test.{ts,tsx,js,jsx}
**/*.spec.{ts,tsx,js,jsx}
**/__tests__/**
**/__mocks__/**
**/test/**
**/tests/**
*.config.{ts,js,mjs,cjs}
```

## Expected Output Difference

**Default mode:**
```
Unused Exports (1)
  src/utils.ts
    - internalHelper (function)
```

**Production mode:**
```
Unused Exports (3)
  src/utils.ts
    - internalHelper (function)
    - testOnlyHelper (function)     ← used in tests but not production
    - devModeLogger (function)       ← used in dev config but not production
```

## Acceptance Criteria

- [ ] `--production` flag enables production mode
- [ ] Test files are excluded from analysis
- [ ] Dev configuration files are excluded
- [ ] Only `dependencies` are checked (not `devDependencies`)
- [ ] Functions used only in tests are reported as unused
- [ ] The output clearly indicates production mode is active
- [ ] Can be combined with other flags (`--output json`, `--entry`, etc.)

## How to Test

1. Create a project:
   - `src/index.ts` imports `src/utils.ts` `add` function
   - `src/utils.ts` exports `add` and `testHelper`
   - `src/utils.test.ts` imports `testHelper`
   - `package.json` has `jest` in devDependencies
2. Run `grove unused` → `testHelper` is NOT reported (used in test)
3. Run `grove unused --production` → `testHelper` IS reported (test file excluded)
4. `jest` is NOT reported in production mode (devDependencies are ignored)

## Edge Cases

- A function used in both production and test code → NOT reported (it has a production usage)
- `--production` with custom `--entry` → custom entries are treated as production entries
- No production entry files found → error message suggesting to configure entry points
