# dpdm - Feature Analysis

**dpdm** (Detect circular dependencies in your TypeScript projects) is a static dependency analyzer for JavaScript and TypeScript projects. It uses the TypeScript compiler's AST parser internally, giving it accurate parsing of both `.ts` and `.js` files.

- **Repository**: [github.com/acrazing/dpdm](https://github.com/acrazing/dpdm)
- **npm**: [npmjs.com/package/dpdm](https://www.npmjs.com/package/dpdm)
- **Current version**: 3.14.0

---

## 1. Circular Dependency Detection

### What it does
Parses all files reachable from one or more entry points, builds a full dependency graph (the `DependencyTree`), then traverses that graph to find all cycles. Each cycle is reported as a chain of file paths forming the loop.

### Input
One or more entry files or glob patterns (e.g., `./src/index.ts`, `./src/**/*.ts`).

### Output format
Circular dependencies are printed to stdout under a "Circular Dependencies" header. Each cycle is numbered:

```
Circular Dependencies
01) src/a.ts -> src/b.ts -> src/c.ts -> src/a.ts
02) src/x.ts -> src/y.ts -> src/x.ts
```

Each chain shows the full path from the first file through every intermediate file and back to the originating file (the last entry equals the first, closing the loop).

### Controlling circular output
- `--circular` (default: `true`) — print circular dependencies to stdout
- `--no-circular` — suppress circular dependency output
- `--skip-dynamic-imports circular` — exclude dynamic `import()` from circular analysis only (still included in tree). Useful because dynamic imports are resolved at runtime and don't cause initialization-order problems.
- `--exit-code circular:N` — exit with code `N` (0-128) if any circular dependencies found. Primary CI integration mechanism.

### Edge cases
- Dynamic imports (`import()`) can optionally be excluded from circular analysis
- Type-only imports in TypeScript can be excluded using `-T` / `--transform` (strips `import type`)
- Unresolvable dependencies (recorded with `id: null`) do NOT participate in circular detection — they appear in warnings instead

---

## 2. Dependency Tree Building and Display

### What it does
Recursively parses all imports/requires/exports starting from entry points and builds a complete dependency tree: a mapping where each key is a resolved file path and the value is an array of that file's dependencies.

### The Dependency structure
Each dependency has:
- **issuer**: the file containing the import
- **request**: the raw import specifier (e.g., `"./foo"`, `"lodash"`)
- **kind**: the type of import (`CommonJS`, `StaticImport`, `DynamicImport`, `StaticExport`)
- **id**: resolved file path, or `null` if unresolvable

### Module types handled
| Module type | Kind | Example |
|---|---|---|
| ESM static imports | `StaticImport` | `import x from "y"` |
| ESM dynamic imports | `DynamicImport` | `import("y")` |
| ESM re-exports | `StaticExport` | `export { x } from "y"` |
| CommonJS | `CommonJS` | `require("y")` |

### Tree output
When `--tree` is enabled (default), prints an indented tree to stdout showing the hierarchy from each entry point. Each line indicates the import kind (`S` for StaticImport, `C` for CommonJS, `D` for DynamicImport, `E` for StaticExport).

### Controlling tree output
- `--tree` (default: `true`) — print tree to stdout
- `--no-tree` — suppress tree output
- `--skip-dynamic-imports tree` — exclude dynamic `import()` entirely from tree parsing

### JSON output
- `-o <file>` / `--output <file>` — writes the full dependency tree as JSON to a file

---

## 3. Warning System

### What it does
During dependency resolution, any import that cannot be resolved (file not found, unsupported extension, etc.) is recorded as a warning rather than causing a hard failure. This allows the tool to continue analyzing the rest of the graph.

### Output
Warnings are printed to stdout. Each warning identifies the unresolvable import and the file containing it.

### Controlling warning output
- `--warning` (default: `true`) — print warnings
- `--no-warning` — suppress warnings

---

## 4. Unused File Detection

### What it does
The `--detect-unused-files-from` flag accepts a glob pattern. dpdm compares all files matching this glob against the set of files reachable from entry points. Any file that exists in the glob but is NOT reachable is reported as "unused."

### Example usage
```bash
dpdm --no-tree --no-warning --no-circular \
     --detect-unused-files-from 'src/**/*.*' \
     ./src/index.ts
```

---

## 5. Complete CLI Reference

```
Usage: dpdm [options] <files...>

Positionals:
  files    The file paths or globs                                    [string]

Options:
  --version                Show version number                       [boolean]
  --context                The context directory to shorten path      [string]
  --extensions, --ext      Comma separated extensions to resolve
                           [default: ".ts,.tsx,.mjs,.js,.jsx,.json"]
  --js                     Comma separated extensions indicating JS-like
                           [default: ".ts,.tsx,.mjs,.js,.jsx"]
  --include                Included filenames regexp [default: ".*"]
  --exclude                Excluded filenames regexp [default: "node_modules"]
  -o, --output             Output json to file                        [string]
  --tree                   Print tree to stdout         [boolean] [default: true]
  --circular               Print circular to stdout     [boolean] [default: true]
  --warning                Print warning to stdout      [boolean] [default: true]
  --tsconfig               tsconfig path for resolving aliases         [string]
  -T, --transform          Transform TS to JS before analysis
                           (strips type-only imports)                [boolean]
  --exit-code              Exit with code. Format: CASE:CODE          [string]
  --skip-dynamic-imports   Skip import(). Choices: "tree" | "circular" [string]
  --detect-unused-files-from  Detect unused files from glob pattern   [string]
  --progress               Show progress bar            [boolean] [default: true]
  -h, --help               Show help                                 [boolean]
```

---

## 6. File Traversal and Module Resolution

### Entry point specification
Files are positional arguments. Globs supported. Multiple entries allowed.

### Resolution behavior
- Uses TypeScript compiler's resolution when `--tsconfig` provided (respects `paths`, `baseUrl`, aliases)
- If `--tsconfig` not set but `tsconfig.json` exists in context directory, it is used automatically
- Extensions resolved in order specified by `--extensions`

### Include/exclude patterns
- `--include` accepts a regexp string (default: `".*"` — all files)
- `--exclude` accepts a regexp string (default: `"node_modules"`)
- Patterns applied to resolved file paths

### File types NOT supported
- CSS/SCSS/Less — generate warnings
- Vue SFCs (`.vue`) — not supported
- JSON — resolved but not parsed for further imports

---

## 7. The Transform Flag (`-T`)

When enabled, TypeScript source is transpiled to JavaScript before dependency extraction. This strips `import type` statements and type-only re-exports, meaning type-only imports will NOT appear in the dependency tree or circular analysis.

This is significant because type-only imports are erased at compile time and cannot cause runtime circular dependency issues.

---

## 8. Exit Codes

- `0` — success (default, even if circulars exist)
- Custom via `--exit-code circular:N` — exit with code `N` when circular dependencies detected

---

## 9. Performance Characteristics

- **Asynchronous parsing**: loads and parses modules asynchronously
- **TypeScript AST**: Uses the TypeScript compiler API (accurate but heavier than regex)
- **Progress bar**: `--progress` (default `true`), disable with `--no-progress`
- **dpdm-fast**: Rust-based reimplementation exists ([GrinZero/dpdm-fast](https://github.com/GrinZero/dpdm-fast)) for even faster performance

---

## 10. Alternatives Comparison

### Madge
- **Extra features**: Visual dependency graph generation (SVG, DOT, JPG via Graphviz), color-coded circular highlighting, CSS preprocessor support (Sass, Stylus, Less), Webpack config integration
- **Where dpdm is better**: More reliable TypeScript parsing, simpler CI integration
- **Stats**: ~1.28M weekly downloads, ~9.7k stars

### dependency-cruiser
- **Extra features**: Custom rule engine (forbidden/allowed rules), architectural constraint validation, visual graph output, configurable severity, orphan detection, package.json dependency validation
- **Where dpdm is better**: Zero-config simplicity, faster for "just find circulars"
- **Stats**: ~373k weekly downloads, ~6.3k stars

### eslint-plugin-import `no-cycle`
- **Extra features**: Integrated into ESLint, `maxDepth` option, per-file inline disable
- **Where dpdm is better**: Standalone, significantly faster, reports all cycles at once

---

## 11. Key Strengths

1. **TypeScript-first**: Uses TS compiler for parsing, accurate on `.ts`/`.tsx`
2. **Simple CI integration**: `--exit-code circular:1` trivially fails builds
3. **Type-import awareness**: `-T` strips type-only imports that can't cause runtime issues
4. **Dynamic import awareness**: `--skip-dynamic-imports circular` ignores lazy imports
5. **Unused file detection**: `--detect-unused-files-from` as bonus feature
6. **Zero config**: Works immediately with sensible defaults

## 12. Key Limitations

1. **No visual graph output** — cannot generate SVG/DOT/image graphs
2. **No custom rules** — cannot define architectural constraints
3. **No CSS preprocessor support** — doesn't parse Sass/Stylus/Less imports
4. **No Vue SFC support** — cannot parse `.vue` files
5. **No per-file severity** — all circulars treated equally
6. **Limited output formats** — only plain text and JSON

---

## Sources

- [GitHub - acrazing/dpdm](https://github.com/acrazing/dpdm)
- [dpdm - npm](https://www.npmjs.com/package/dpdm)
- [dpdm README](https://github.com/acrazing/dpdm/blob/master/README.md)
- [dpdm-fast (Rust reimplementation)](https://github.com/GrinZero/dpdm-fast)
- [GitHub - pahen/madge](https://github.com/pahen/madge)
- [GitHub - sverweij/dependency-cruiser](https://github.com/sverweij/dependency-cruiser)
