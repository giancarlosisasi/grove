# User Story: Ignore Patterns

## Story

**As a** developer,
**I want to** exclude specific files, directories, or dependencies from the analysis,
**So that** Grove doesn't report false positives for files I know are used in ways the tool can't detect (e.g., loaded dynamically, referenced in non-JS configs, or intentionally standalone).

## Command

```
grove unused --ignore "**/*.test.ts" --ignore "scripts/**"
```

For dependencies:

```
grove unused --ignore-deps "lodash" --ignore-deps "@internal/config"
```

## Why This Is Needed

Some files are legitimately used but not through standard imports — they might be loaded by a test runner, referenced in a CI config, or used as scripts. The ignore mechanism prevents these from cluttering the results.

## Expected Behavior

### File Ignoring (`--ignore`)
1. Files matching `--ignore` patterns are completely excluded from analysis
2. They are not parsed, not included in the graph, and never reported as unused
3. Multiple `--ignore` flags can be used
4. Patterns use glob syntax
5. This is additive to default ignores (`node_modules`, `.git`)

### Dependency Ignoring (`--ignore-deps`)
1. Dependencies matching `--ignore-deps` are never reported as unused
2. Exact package name matching (not glob)
3. Multiple flags can be used

## Examples

```bash
# Ignore all test files
grove unused --ignore "**/*.test.ts" --ignore "**/*.spec.ts"

# Ignore entire directories
grove unused --ignore "scripts/**" --ignore "fixtures/**"

# Ignore specific dependencies
grove unused --ignore-deps "webpack" --ignore-deps "@types/node"

# Combine file and dependency ignoring
grove unused --ignore "**/__mocks__/**" --ignore-deps "jest"
```

## Acceptance Criteria

- [ ] `--ignore` accepts glob patterns for files/directories to skip
- [ ] `--ignore` can be specified multiple times
- [ ] Ignored files are not parsed and not included in the module graph
- [ ] `--ignore-deps` accepts package names to skip in dependency checking
- [ ] `--ignore-deps` can be specified multiple times
- [ ] Default ignores (`node_modules`, `.git`) always apply regardless of `--ignore`
- [ ] `.gitignore` patterns are respected by default

## How to Test

1. Create a project:
   - `src/index.ts` imports `src/used.ts`
   - `src/unused.ts` exists, not imported
   - `src/test-helper.ts` exists, not imported (used by test runner)
   - `package.json` lists `jest` (used by test runner, not imported in code)
2. Run `grove unused --ignore "src/test-helper.ts" --ignore-deps "jest"`
3. Verify `src/test-helper.ts` is NOT reported
4. Verify `jest` is NOT reported as unused dependency
5. Verify `src/unused.ts` IS still reported

## Edge Cases

- `--ignore` pattern matches an entry file → the entry file is still skipped (user explicitly ignored it)
- `--ignore-deps` with a typo in package name → no effect, no warning (the dep simply won't be found to ignore)
- `--ignore` with a directory that doesn't exist → no effect, no warning
