# Epic: Unused Module Detection

## Vision

As a developer working on a large JavaScript/TypeScript project, I want to identify and remove dead code — files nobody imports, exports nobody uses, and dependencies nobody references — so that my codebase stays clean, my bundle stays small, and my team doesn't waste time maintaining code that has no effect.

## What "Unused" Means

There are three distinct categories of unused code:

### 1. Unused Files
A file exists in the project but no other file imports it. It is completely disconnected from the application's module graph. The file could be deleted and nothing would break.

**Example:** `src/utils/old-helper.ts` exists but no file in the project has `import ... from './utils/old-helper'`.

### 2. Unused Exports
A file exports a function, class, variable, type, or interface, but nothing in the project imports that specific export. The export keyword could be removed (the code might still be used internally within the file, but the `export` is unnecessary).

**Example:** `src/utils/math.ts` exports `add`, `subtract`, and `multiply`. Only `add` and `subtract` are imported anywhere. `multiply` is an unused export.

### 3. Unused Dependencies
A package is listed in `package.json` (`dependencies` or `devDependencies`) but never imported or referenced anywhere in the code or configuration.

**Example:** `package.json` lists `"lodash": "^4.17.21"` but no file in the project has `import ... from 'lodash'` or `require('lodash')`.

## How Detection Works (High Level)

1. **Discover files** — Walk the project directory, collect all JS/TS files matching project patterns
2. **Identify entry points** — Determine which files are the "roots" of the module graph (main files, bin files, etc.)
3. **Build the module graph** — Starting from entry points, parse each file to find all imports, follow them recursively
4. **Compare** — Cross-reference what was discovered:
   - Files in the project NOT in the graph → unused files
   - Exports in parsed files NOT imported anywhere → unused exports
   - Packages in package.json NOT found in any import → unused dependencies

## Success Criteria for the Epic

- Can scan a real-world JS/TS project and correctly identify unused files
- Can identify unused exports with zero false positives on common patterns
- Can identify unused package.json dependencies
- Runs fast enough to be practical on large projects (thousands of files in seconds)
- Provides clear, actionable output that tells the developer exactly what is unused and where

## User Stories in This Epic

1. [Scan Project](./scan-project.md) — Basic project scanning
2. [Entry Files Argument](./arg-entry-files.md) — Specify entry points
3. [Project Files Argument](./arg-project-files.md) — Define project file scope
4. [Ignore Patterns](./arg-ignore-patterns.md) — Exclude files from analysis
5. [Configuration File](./config-file.md) — Support a config file
6. [Detect Unused Files](./detect-unused-files.md) — Find unreachable files
7. [Detect Unused Exports](./detect-unused-exports.md) — Find unused exported symbols
8. [Detect Unused Dependencies](./detect-unused-dependencies.md) — Find unused package.json entries
9. [Detect Unused DevDependencies](./detect-unused-devdeps.md) — Find unused devDependencies
10. [Output Formats](./output-formats.md) — Support different output formats
11. [Exit Codes](./exit-codes.md) — CI-friendly exit codes
12. [Production Mode](./production-mode.md) — Analyze only production code
13. [Auto-Fix](./auto-fix.md) — Automatically remove unused exports
