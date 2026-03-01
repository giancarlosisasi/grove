# Grove - Project Setup & Architecture

This document covers the technical foundation for the Grove CLI tool — the only spec file that includes code, commands, and technical decisions.

---

## 1. What is Grove?

Grove is a Go CLI tool distributed via npm that analyzes JavaScript/TypeScript projects to find:
- Unused files, exports, and dependencies (inspired by knip)
- Circular dependencies (inspired by dpdm)

It must handle projects with thousands of files and millions of lines of code with excellent performance.

---

## 2. Go Version

```bash
go version
# Expected: go1.26.0
```

Install via:
```bash
# macOS
brew install go

# Or download from https://go.dev/dl/
```

Verify:
```bash
go version
# go1.26.0 darwin/arm64
```

---

## 3. CLI Framework: Cobra

We use [Cobra](https://github.com/spf13/cobra) as the CLI framework.

**Why Cobra:**
- Industry standard for Go CLIs (used by kubectl, docker, hugo, gh)
- Built-in help generation, subcommand support, flag parsing
- Excellent documentation and community
- Supports persistent flags (global flags across all subcommands)
- Auto-completion generation (bash, zsh, fish, powershell)

```bash
go get -u github.com/spf13/cobra@latest
```

**Companion:** [Viper](https://github.com/spf13/viper) for configuration file handling.

```bash
go get -u github.com/spf13/viper@latest
```

**Why Viper:**
- Reads JSON, JSONC, YAML, TOML config files
- Environment variable binding
- Default values
- Works seamlessly with Cobra flags

---

## 4. JavaScript/TypeScript Parser: esbuild-internal

**Critical decision.** We need to parse JS/TS files to extract imports and exports. Performance is the top priority.

### Recommended: `github.com/nicolo-ribaudo/esbuild-internal` (fork of esbuild internals)

```bash
go get github.com/nicolo-ribaudo/esbuild-internal@latest
```

**Why esbuild's parser:**
- **Written in pure Go** — no CGo, no FFI, no Node.js dependency
- **Extremely fast** — esbuild's parser is the fastest Go-native JS/TS parser, with built-in parallel parsing via goroutines
- **Battle-tested** — used by millions of developers daily
- **Feature-complete** — handles ESM imports/exports, CJS require, dynamic imports, TypeScript (.ts, .tsx), JSX, re-exports, barrel files
- **ImportRecord abstraction** — produces structured import records with path, kind (static/dynamic/require), and source location
- **No type-checking overhead** — strips TypeScript types without checking them (exactly what we need)

**Key packages we'll use:**
- `js_parser` — parse a file into AST + ImportRecords
- `js_ast` — AST types, ImportRecord struct
- `js_lexer` — tokenizer
- `config` — parser options (enable TS, JSX, etc.)
- `logger` — error/warning handling

**Alternative considered: `github.com/nicolo-ribaudo/esbuild-internal`**

Note: There are two known forks exposing esbuild internals:
- `github.com/ije/esbuild-internal` — maintained by the esm.sh author
- `github.com/kyle-retool/esbuild_internal` — alternative fork

Evaluate both for freshness (last sync with upstream esbuild) and pick the one most up-to-date.

**Future option:** Watch [microsoft/typescript-go Discussion #2442](https://github.com/microsoft/typescript-go/discussions/2442) — if Microsoft extracts their TS parser into a public Go module, it would be the most correct and complete option. Currently their parser is in `internal/` packages and cannot be imported.

### Alternatives NOT chosen

| Parser | Why Not |
|--------|---------|
| microsoft/typescript-go | Parser in `internal/` packages, can't import without forking |
| SWC | Rust-based, no Go bindings |
| OXC | Rust-based, no Go bindings |
| tree-sitter | CGo dependency, moderate speed, requires custom extraction code |
| go-fAST | No TypeScript support |
| goja/otto | No TypeScript, incomplete ES6 |
| Regex-only | Too many edge cases, false positives/negatives |

---

## 5. Additional Required Packages

```bash
# Glob pattern matching (for file discovery)
go get github.com/bmatcuk/doublestar/v4@latest

# Fast file walking (faster than filepath.Walk)
go get github.com/charlievieth/fastwalk@latest

# JSON output formatting
# (stdlib encoding/json is sufficient)

# Colored terminal output
go get github.com/fatih/color@latest

# Progress bar (for large projects)
go get github.com/schollz/progressbar/v3@latest

# Testing
# (stdlib testing + testify for assertions)
go get github.com/stretchr/testify@latest
```

---

## 6. Project Folder Structure

```
grove/
├── cmd/                        # CLI command definitions (Cobra)
│   ├── root.go                 # Root command, global flags
│   ├── unused.go               # `grove unused` subcommand
│   ├── circular.go             # `grove circular` subcommand
│   └── version.go              # `grove version` subcommand
│
├── internal/                   # Private application code
│   ├── config/                 # Configuration loading (Viper)
│   │   └── config.go
│   │
│   ├── parser/                 # JS/TS file parsing (esbuild wrapper)
│   │   ├── parser.go           # Parse a file, extract imports/exports
│   │   └── parser_test.go
│   │
│   ├── resolver/               # Module resolution (resolve import paths to files)
│   │   ├── resolver.go
│   │   └── resolver_test.go
│   │
│   ├── graph/                  # Dependency graph building
│   │   ├── graph.go            # Build graph from parsed files
│   │   ├── circular.go         # Circular dependency detection (DFS)
│   │   ├── unused.go           # Unused file/export detection
│   │   └── graph_test.go
│   │
│   ├── scanner/                # File discovery (walk + glob)
│   │   ├── scanner.go
│   │   └── scanner_test.go
│   │
│   ├── reporter/               # Output formatting
│   │   ├── reporter.go         # Reporter interface
│   │   ├── text.go             # Human-readable text output
│   │   ├── json.go             # JSON output
│   │   └── github_actions.go   # GitHub Actions annotations
│   │
│   └── types/                  # Shared types/structs
│       └── types.go
│
├── main.go                     # Entry point
├── go.mod
├── go.sum
│
├── npm/                        # npm distribution package
│   ├── package.json
│   ├── install.js              # Post-install script to download Go binary
│   └── bin/                    # Platform-specific binaries
│
├── research/                   # Research documentation
├── specs/                      # Spec documentation
├── CLAUDE.md
└── README.md
```

---

## 7. Software Design Patterns

### Pattern: Pipeline

The core analysis flow follows a pipeline pattern:

```
File Discovery → Parsing → Graph Building → Analysis → Reporting
```

Each stage is independent and testable. Data flows forward through well-defined interfaces.

### Pattern: Strategy (for Reporters)

Different output formats implement a common `Reporter` interface:

```go
type Reporter interface {
    ReportUnusedFiles(files []string)
    ReportUnusedExports(exports []UnusedExport)
    ReportCircularDeps(chains [][]string)
    Flush() error
}
```

### Pattern: Concurrent Worker Pool (for Parsing)

For parsing thousands of files, we use a worker pool with bounded concurrency:

```
File paths → [Worker Pool (N goroutines)] → Parsed results
```

N = `runtime.NumCPU()` by default, configurable via `--concurrency`.

### Pattern: Visitor (for Graph Traversal)

Circular dependency detection uses DFS with a visitor pattern to walk the dependency graph.

---

## 8. Concurrency Strategy

Performance is critical. The architecture leverages Go's concurrency:

1. **File discovery**: Single goroutine with `fastwalk` (already fast)
2. **File parsing**: Worker pool of N goroutines, each parsing files independently using esbuild's parser
3. **Graph building**: Sequential (needs all parse results), but fast since it's just connecting edges
4. **Circular detection**: DFS — sequential but O(V+E), fast enough
5. **Unused detection**: Set operations — sequential, O(N)

The bottleneck is parsing. With a worker pool, we can parse files in parallel at near-maximum CPU utilization.

---

## 9. Module Resolution Strategy

Resolving `import "./foo"` to an actual file path requires:

1. Check if the path has an extension → use directly
2. Try appending each extension in order: `.ts`, `.tsx`, `.js`, `.jsx`, `.mjs`, `.cjs`, `.json`
3. Check if path is a directory → try `index.{ts,tsx,js,jsx}`
4. Check `tsconfig.json` `paths` and `baseUrl` for aliases
5. For bare specifiers (`lodash`), resolve from `node_modules`

This mirrors how Node.js and TypeScript resolve modules.

---

## 10. npm Distribution Strategy

The Go binary is compiled for multiple platforms and distributed via npm:

```json
{
  "name": "grove",
  "bin": {
    "grove": "./bin/grove"
  },
  "scripts": {
    "postinstall": "node install.js"
  }
}
```

The `install.js` script detects the platform (darwin/linux/windows + amd64/arm64) and downloads the appropriate pre-compiled binary from GitHub releases.

Build targets:
- `darwin/amd64` (macOS Intel)
- `darwin/arm64` (macOS Apple Silicon)
- `linux/amd64`
- `linux/arm64`
- `windows/amd64`

---

## 11. Testing Strategy

- **Unit tests**: Each package has `_test.go` files testing individual functions
- **Integration tests**: Test fixtures with known project structures and expected outputs
- **Snapshot tests**: Compare CLI output against golden files
- **Benchmark tests**: `go test -bench` for parsing and graph operations at scale

Test fixtures should include:
- Simple ESM project
- Simple CJS project
- Mixed ESM/CJS project
- TypeScript project with path aliases
- Project with circular dependencies
- Project with unused files
- Project with unused exports
- Large-scale project (generated, for benchmarks)

---

## 12. Build & Release

```bash
# Development
go run main.go

# Build
go build -o grove main.go

# Cross-compile
GOOS=darwin GOARCH=arm64 go build -o grove-darwin-arm64 main.go
GOOS=linux GOARCH=amd64 go build -o grove-linux-amd64 main.go

# Run tests
go test ./...

# Run benchmarks
go test -bench=. ./internal/parser/
```

---

## 13. Key Architectural Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| CLI framework | Cobra + Viper | Industry standard, subcommands, flags, config |
| JS/TS parser | esbuild-internal | Pure Go, fastest available, battle-tested |
| File walking | fastwalk | Faster than stdlib filepath.Walk |
| Glob matching | doublestar | Full doublestar glob support |
| Concurrency | Worker pool | Maximize CPU for parsing |
| Config format | JSON (primary) | Simple, widely understood |
| Output formats | Text, JSON, GitHub Actions | Cover human + CI + machine needs |
| Distribution | npm + prebuilt binaries | Target audience uses npm |
| Testing | stdlib + testify + fixtures | Standard Go testing practices |
