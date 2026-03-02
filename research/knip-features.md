# Knip - Feature Analysis

**Knip** (Dutch for "cut") is a project linter for JavaScript and TypeScript projects that finds unused files, dependencies, and exports. It analyzes repositories holistically — rather than file-by-file like ESLint — connecting all the dots in terms of files, imports, exports, and dependencies, then reporting what is unused.

- **Repository**: [webpro-nl/knip](https://github.com/webpro-nl/knip)
- **Website**: [knip.dev](https://knip.dev)

---

## 1. Unused Files Detection

**What it does:** Identifies files that exist in the project but are not reachable from any entry point through the import/require graph.

**How it works:**
1. Resolves all **entry files** (defaults + plugin-detected entry points + `package.json` fields like `main`, `bin`, `exports`).
2. From entry files, follows all `import`/`require` statements to build a complete module graph.
3. Matches resolved files against the **project** file patterns.
4. Any file matching `project` patterns but NOT reached from any entry point is reported as **unused**.

**Inputs:**
- `entry` patterns define starting points
- `project` patterns define the universe of files to check
- Plugins automatically contribute additional entry patterns

**Output:** List of file paths that are unreachable from any entry point.

**Edge cases:**
- Files referenced only in configuration files (e.g., webpack aliases) need plugins to detect them
- Dynamic imports (`import()`) are followed
- Files referenced via `package.json` fields (`main`, `bin`, `exports`) are automatically treated as entry files
- The `!` suffix on glob patterns respects `.gitignore`

---

## 2. Unused Exports Detection

**What it does:** Finds exported symbols (functions, classes, variables, types, interfaces, enums) that are never imported anywhere in the project.

**Behavior:**
- By default, exports from **entry files** are NOT reported as unused (since they are the public API)
- Set `includeEntryExports: true` to also check entry file exports
- Supports both named exports and default exports
- Detects unused re-exports

**Sub-categories:**
- **Unused exports** (`exports`): Regular exported values
- **Unused exported types** (`types`): Exported TypeScript types/interfaces
- **Unused exported enum members** (`enumMembers`): Individual enum member values
- **Unused class members** (`classMembers`): Public methods/properties of exported classes
- **Duplicate exports** (`duplicates`): The same value exported under multiple names

**JSDoc/TSDoc Tag System:**
- `@public` — Tag an export so Knip will NOT report it as unused
- `@internal` — Identical behavior to `@public`
- `@alias` — Prevents duplicate export warnings
- Custom tags can be defined and used with `--tags` filter

**Auto-fix:** Running `knip --fix` can automatically remove the `export` keyword from unused exports.

---

## 3. Unused Dependencies Detection

**What it does:** Reports packages listed in `dependencies` or `devDependencies` in `package.json` that are never referenced in the code, configuration files, or scripts.

**How it works:**
- Connects packages to their binaries
- Plugins parse tool configuration files to detect dependency usage (e.g., ESLint plugins referenced in `.eslintrc`)
- Script analysis in `package.json` detects binary usage

**Sub-categories:**
- **Unused dependencies** (`dependencies`): Production deps not used in production code
- **Unused devDependencies** (`devDependencies`): Dev deps not used anywhere
- **Unlisted dependencies** (`unlisted`): Imported packages not listed in `package.json`
- **Unlisted binaries** (`binaries`): Executables referenced but not traceable to a package
- **Unresolved imports** (`unresolved`): Import specifiers that cannot be resolved
- **Referenced optional peer dependencies** (`optionalPeerDependencies`): Optional peers that are actually used

**Edge cases:**
- `ignoreDependencies` is available for packages that Knip cannot trace (last resort)
- `ignoreBinaries` for executables that are system-installed or otherwise unrecognizable
- In monorepos, each workspace's `package.json` should list its own dependencies for accurate reporting

---

## 4. Entry File Resolution

Knip resolves entry files from multiple sources:

1. **Default patterns:** `{index,cli,main}.{js,cjs,mjs,jsx,ts,cts,mts,tsx}` and `src/{index,cli,main}.{js,cjs,mjs,jsx,ts,cts,mts,tsx}`
2. **package.json fields:** `main`, `bin`, `exports`
3. **package.json scripts:** Analyzed for referenced files and binaries
4. **Plugin-detected entries:** Each enabled plugin contributes its own entry patterns (e.g., Next.js adds `pages/**/*`)
5. **Custom configuration:** User-specified `entry` patterns

In monorepos, entry file resolution happens independently for each workspace.

---

## 5. Configuration File

### Supported File Names (searched in order):
- `knip.json`
- `knip.jsonc` (JSON with comments)
- `.knip.json`
- `.knip.jsonc`
- `knip.ts`
- `knip.js`
- `knip.config.ts`
- `knip.config.js`
- `package.json` (under the `"knip"` property)

A custom path can be specified with `--config <path>`.

### Default Configuration

When no configuration file is provided, Knip uses the following defaults:

```json
{
  "entry": [
    "{index,cli,main}.{js,cjs,mjs,jsx,ts,cts,mts,tsx}",
    "src/{index,cli,main}.{js,cjs,mjs,jsx,ts,cts,mts,tsx}"
  ],
  "project": ["**/*.{js,cjs,mjs,jsx,ts,cts,mts,tsx}!"]
}
```

### Complete Configuration Options

| Option | Type | Description |
|--------|------|-------------|
| `entry` | `string[]` | Entry file glob patterns — starting points for module resolution |
| `project` | `string[]` | All project file patterns — matched against resolved files to find unused files |
| `ignore` | `string[]` | Glob patterns for files/directories to ignore entirely |
| `ignoreBinaries` | `string[]` | Binary/executable names to ignore when reported as unlisted |
| `ignoreDependencies` | `string[]` | Package names to ignore when reported as unused |
| `ignoreMembers` | `string[]` | Class/enum member patterns to ignore |
| `ignoreUnresolved` | `string[]` | Import specifiers to ignore when reported as unresolved |
| `ignoreExportsUsedInFile` | `boolean \| object` | Do not report exports used internally within the same file |
| `ignoreWorkspaces` | `string[]` | Workspace patterns to exclude from analysis |
| `includeEntryExports` | `boolean` | Report unused exports even in entry files (default: `false`) |
| `rules` | `object` | Per-issue-type severity configuration (`"error"`, `"warn"`, or `"off"`) |
| `paths` | `object` | TypeScript-style path aliases |
| `compilers` | `object` | Custom compiler functions for non-standard file extensions |
| `workspaces` | `object` | Per-workspace configuration overrides |

---

## 6. Plugin/Framework Support (137+ Plugins)

**What it does:** Plugins automatically detect tools and frameworks in your project and configure entry files, configuration files, and dependency patterns accordingly.

**How plugins are activated:** A plugin is enabled when its related package is found in `dependencies` or `devDependencies` in `package.json`.

**What plugins do:**
1. Define additional entry file patterns (e.g., Next.js plugin adds `pages/**/*.{js,jsx,ts,tsx}`)
2. Parse configuration files to find referenced dependencies (e.g., ESLint plugin reads `.eslintrc`)
3. Parse CLI arguments in package.json scripts
4. Recursively find additional config files

**Notable plugins:** Angular, Astro, Babel, Cypress, ESLint, Gatsby, GitHub Actions, GraphQL, Jest, Mocha, Next.js, Nx, Playwright, PostCSS, Prettier, Remix, Rollup, Storybook, Stylelint, Svelte, Tailwind, TypeScript, Vite, Vitest, Webpack, and many more.

---

## 7. Monorepo and Workspace Support

**What it does:** First-class support for monorepos. Automatically discovers workspaces from `package.json#workspaces` or `pnpm-workspace.yaml`.

**Features:**
- Every workspace is part of the analysis automatically
- Cross-workspace import/export tracking
- Workspace-level plugin configuration
- `--workspace <name>` CLI flag to analyze specific workspaces
- `ignoreWorkspaces` to exclude specific workspaces
- Automatic inclusion of ancestor and dependent workspaces when linting a specific workspace

---

## 8. Production Mode

**CLI flag:** `--production`

**Behavior:**
- Uses only production entry files (excludes test files, config files)
- Considers only `dependencies` (excludes `devDependencies`)
- Reports dead code and dependencies that would only be referenced by tests/tooling
- More strict than default mode

---

## 9. Strict Mode

**CLI flag:** `--strict`

**Behavior:**
- Implies `--production`
- Isolates workspaces — considers only direct dependencies per workspace
- Most aggressive analysis mode

---

## 10. Compilers (Non-standard file extensions)

**What it does:** Enables Knip to analyze non-standard file extensions (`.svelte`, `.vue`, `.astro`, `.mdx`, etc.) by extracting import/export statements.

**Built-in compilers:** Basic regex-based extractors for Astro, MDX, Svelte, Vue. These are NOT real compilers — they use regexes to extract import statements. This approach is fast and sufficient for building the module graph.

**Custom compilers:** Signature: `(content: string) => string` — receives file contents, returns JavaScript/TypeScript.

---

## 11. CLI Arguments and Flags

### Core Flags
| Flag | Description |
|------|-------------|
| `--config <path>` | Custom configuration file path |
| `--tsConfig <file>` | Custom TypeScript configuration file |
| `--production` | Lint only production code |
| `--strict` | Isolate workspaces, direct deps only (implies `--production`) |
| `--fix` | Auto-fix: remove unused exports, optionally remove files |
| `--watch` | Watch mode |
| `--cache` | Cache results for faster consecutive runs |

### Output Control
| Flag | Description |
|------|-------------|
| `--reporter <name>` | `symbols` (default), `compact`, `codeowners`, `json`, `markdown`, `codeclimate`, `disclosure`, `github-actions` |
| `--reporter-options <json>` | Extra options to reporter as JSON |
| `--no-progress` | Disable progress updates |
| `--no-exit-code` | Always exit with code 0 |

### Filtering
| Flag | Description |
|------|-------------|
| `--include <type>` | Report only listed issue types |
| `--exclude <type>` | Exclude issue types from report |
| `--dependencies` | Short-hand: only dependency-related issues |
| `--exports` | Short-hand: only export-related issues |
| `--tags <tags>` | Filter by JSDoc/TSDoc tags |

### Debugging
| Flag | Description |
|------|-------------|
| `--debug` | Verbose output |
| `--performance` | Show execution time of expensive functions |
| `--trace` | Trace exports to see where they are imported |
| `--include-libs` | Include type definitions of external libraries |

### Workspace Selection
| Flag | Description |
|------|-------------|
| `--workspace <name>` | Select specific workspace(s) |
| `--directory <path>` | Set working directory |

---

## 12. Issue Types (Complete List)

| Issue Type | Category | Description |
|-----------|----------|-------------|
| `files` | Files | Unused files not reachable from entry points |
| `dependencies` | Dependencies | Unused production dependencies |
| `devDependencies` | Dependencies | Unused development dependencies |
| `optionalPeerDependencies` | Dependencies | Referenced optional peer dependencies |
| `unlisted` | Dependencies | Used but not listed in package.json |
| `binaries` | Dependencies | Referenced binaries not traceable to a package |
| `unresolved` | Dependencies | Import specifiers that cannot be resolved |
| `exports` | Exports | Unused exported values |
| `types` | Exports | Unused exported types/interfaces |
| `enumMembers` | Exports | Unused individual enum member values |
| `classMembers` | Exports | Unused class members |
| `duplicates` | Exports | Same value exported under multiple names |

---

## 13. Rules and Severity

Rules allow per-issue-type severity:

```json
{
  "rules": {
    "files": "error",
    "dependencies": "error",
    "devDependencies": "warn",
    "exports": "warn",
    "types": "warn"
  }
}
```

Severity levels: `"error"` (default, affects exit code), `"warn"` (printed faded, no exit code impact), `"off"` (hidden).

---

## 14. Output Formats / Reporters

| Reporter | Description |
|----------|-------------|
| `symbols` | Default. Unused items grouped by file with symbol detail |
| `compact` | Condensed text output |
| `json` | Machine-readable JSON |
| `codeowners` | Sorted by code owners |
| `markdown` | Markdown-formatted |
| `codeclimate` | CodeClimate-compatible JSON |
| `disclosure` | Collapsible disclosure format |
| `github-actions` | GitHub Actions annotations |

Multiple reporters can be used simultaneously.

---

## 15. Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Successful run, no issues found |
| `1` | Successful run, lint issues found |
| `2` | Configuration or runtime error |

Use `--no-exit-code` to always exit with `0`.

---

## 16. Auto-Fix

**CLI flag:** `--fix`

**What it fixes:**
- Removes the `export` keyword from unused exports
- Removes unused re-exports
- Removes unused exported items from export assignments
- Can remove entire unused files (opt-in)
- Can format modified files after fixing with `--format`

**Safety:** Does NOT remove entire declarations (RHS may have side effects). Always verify dry-run before applying.

---

## 17. Module System Support

**Supported file extensions:** `.js`, `.mjs`, `.cjs`, `.jsx`, `.ts`, `.mts`, `.cts`, `.tsx`

Supports:
- **ESM** — `import`/`export` syntax, `.mjs`/`.mts` files
- **CommonJS** — `require()`/`module.exports`, `.cjs`/`.cts` files
- **TypeScript** — Full support including `.ts`, `.tsx`, `.mts`, `.cts`
- **JSX/TSX** — React and JSX files
- **TypeScript path aliases** — Automatically reads `compilerOptions.paths` from `tsconfig.json`

---

## 18. Ignore/Filtering Mechanisms

| Mechanism | Scope | Description |
|-----------|-------|-------------|
| `ignore` | Files | Glob patterns for files to skip entirely |
| `ignoreDependencies` | Deps | Package names to never report as unused |
| `ignoreBinaries` | Bins | Binary names to never report as unlisted |
| `ignoreMembers` | Exports | Class/enum member patterns to skip |
| `ignoreUnresolved` | Imports | Import specifiers to not report as unresolved |
| `ignoreExportsUsedInFile` | Exports | Do not report exports used internally |
| `ignoreWorkspaces` | Workspaces | Workspace patterns to exclude |
| `@public` / `@internal` JSDoc | Exports | Tag individual exports to exclude |
| `--exclude` CLI | Any type | Exclude specific issue types |
| `rules` with `"off"` | Any type | Permanently disable issue types |

---

## 19. Watch Mode

**CLI flag:** `--watch`

Watches the directory and incrementally updates reported issues when files are modified, added, or deleted.

---

## 20. CI/CD Integration

- Exit code 1 when issues found (fails CI step)
- `--no-exit-code` for advisory mode
- `--reporter json` for machine-readable output
- `--reporter github-actions` for inline annotations in GitHub PRs
- Deterministic output suitable for diff-based PR comments

---

## Sources

- [knip.dev](https://knip.dev/)
- [Getting Started](https://knip.dev/overview/getting-started)
- [Configuration](https://knip.dev/overview/configuration)
- [Configuration Reference](https://knip.dev/reference/configuration)
- [CLI Arguments](https://knip.dev/reference/cli)
- [Issue Types](https://knip.dev/reference/issue-types)
- [Handling Issues](https://knip.dev/guides/handling-issues)
- [Monorepos & Workspaces](https://knip.dev/features/monorepos-and-workspaces)
- [Compilers](https://knip.dev/features/compilers)
- [Auto-fix](https://knip.dev/features/auto-fix)
- [Production Mode](https://knip.dev/features/production-mode)
- [Plugins](https://knip.dev/explanations/plugins)
- [GitHub - webpro-nl/knip](https://github.com/webpro-nl/knip)
