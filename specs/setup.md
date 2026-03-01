# Grove - Project Setup & Key Decisions

This document covers the key technical decisions that shape Grove's architecture and infrastructure.

---

## 1. What is Grove?

Grove is a Rust CLI tool distributed via npm that analyzes JavaScript/TypeScript projects to find:
- Unused files, exports, and dependencies (inspired by knip)
- Circular dependencies (inspired by dpdm)

It must handle projects with thousands of files and millions of lines of code with excellent performance.

---

## 2. Language: Rust

**Why Rust over Go:**
- The JS/TS parser ecosystem in Go is limited — no production-ready, pure-Go TypeScript parser exists.
- Rust has **oxc**, a mature, high-performance JS/TS toolchain (parser, resolver, linter) already used by production bundlers like Rolldown.
- Rust's zero-cost abstractions and lack of GC make it ideal for a tool where speed is the primary differentiator.
- Rust compiles to a single static binary per platform — same distribution story as Go.

**Rust version:** 1.93.1 (Feb. 12, 2026)

---

## 3. JS/TS Parser & Resolver: oxc

**Critical decision.** We need to parse JS/TS files to extract imports and exports, and resolve import paths to actual files. Performance is the top priority.

### Parser: `oxc_parser`

- **Extremely fast** — consistently the fastest JS/TS parser in benchmarks, faster than SWC and tree-sitter.
- **Battle-tested** — used by Rolldown (Vite's Rust bundler), oxlint, and other production tools.
- **Feature-complete** — handles ESM imports/exports, CJS require, dynamic imports, TypeScript (.ts, .tsx), JSX, re-exports, barrel files.
- **No type-checking overhead** — parses TypeScript syntax without checking types (exactly what we need).
- **Active development** — backed by the oxc project with strong community and frequent releases.

### Resolver: `oxc_resolver`

- **Drop-in Node.js module resolution** — handles all the resolution rules (extensions, index files, package.json exports, tsconfig paths).
- **Same project as the parser** — guaranteed compatibility, consistent API style.
- **Handles tsconfig paths, baseUrl, package.json exports** — all the resolution edge cases covered.
- **Used by Rolldown** — proven in a production bundler.

Using both from the same project avoids integration mismatches and reduces maintenance burden.

Repository: https://github.com/oxc-project/oxc

### Alternatives NOT chosen

| Parser/Resolver | Why Not |
|-----------------|---------|
| SWC (swc_ecma_parser) | Viable but oxc is faster in benchmarks and has a dedicated resolver |
| tree-sitter | Designed for editors (incremental parsing), not batch analysis; heavier API |
| Biome (rome_js_parser) | Less focused on being a reusable library; fewer users outside Biome itself |
| Regex-only | Too many edge cases, false positives/negatives |

---

## 4. CLI Framework: clap (native)

We use [clap](https://github.com/clap-rs/clap) with derive macros for CLI argument parsing.

**Why clap:**
- De facto standard for Rust CLIs (used by ripgrep, fd, bat, cargo subcommands).
- Derive macros generate argument parsing from struct definitions — minimal boilerplate.
- Built-in help generation, subcommand support, flag parsing, shell completions.
- Zero runtime dependencies beyond what we already pull in.

**No separate config crate needed initially** — we can parse `grove.config.json` / `package.json` with `serde_json` (already a transitive dependency of oxc). Add a config crate later only if complexity warrants it.

---

## 5. Concurrency: rayon

We use [rayon](https://github.com/rayon-rs/rayon) for parallel file parsing.

**Why rayon:**
- Work-stealing thread pool — automatically balances load across CPU cores.
- Simple API: `.par_iter()` on any iterator to parallelize.
- No manual thread/channel management for the common case.
- Industry standard for CPU-bound parallelism in Rust.

The bottleneck is parsing. With rayon, we parse files in parallel at near-maximum CPU utilization with minimal code.

---

## 6. File Walking: ignore / walkdir

We use the [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) crate (from ripgrep) for file discovery.

**Why ignore:**
- Respects `.gitignore` rules out of the box — no need to re-implement ignore logic.
- Parallel directory walking built-in.
- Handles symlinks, hidden files, and all the edge cases.
- Battle-tested — it's what powers `rg` and `fd`.

---

## 7. npm Distribution Strategy

Same approach as other Rust CLI tools (oxlint, biome, turbo): **platform-specific optional npm packages**.

```json
{
  "optionalDependencies": {
    "@giancarlosio/grove-darwin-arm64": "0.1.0",
    "@giancarlosio/grove-darwin-x64": "0.1.0",
    "@giancarlosio/grove-linux-x64-gnu": "0.1.0",
    "@giancarlosio/grove-linux-arm64-gnu": "0.1.0",
    "@giancarlosio/grove-win32-x64": "0.1.0"
  }
}
```

npm automatically installs only the matching platform package. No postinstall scripts needed. This is how oxlint, biome, turbo, and swc distribute their binaries.

Build targets:
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-apple-darwin` (macOS Intel)
- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-pc-windows-msvc` (Windows x64)

---

## 8. Project Folder Structure

```
grove/
├── src/
│   ├── main.rs                # Entry point, CLI setup (clap)
│   ├── lib.rs                 # Library root, re-exports
│   │
│   ├── config/                # Configuration loading
│   │   └── mod.rs
│   │
│   ├── parser/                # JS/TS file parsing (oxc_parser wrapper)
│   │   ├── mod.rs
│   │   └── tests.rs
│   │
│   ├── resolver/              # Module resolution (oxc_resolver wrapper)
│   │   ├── mod.rs
│   │   └── tests.rs
│   │
│   ├── graph/                 # Dependency graph building
│   │   ├── mod.rs
│   │   ├── circular.rs        # Circular dependency detection (DFS)
│   │   ├── unused.rs          # Unused file/export detection
│   │   └── tests.rs
│   │
│   ├── scanner/               # File discovery (ignore crate)
│   │   ├── mod.rs
│   │   └── tests.rs
│   │
│   └── reporter/              # Output formatting
│       ├── mod.rs
│       ├── text.rs            # Human-readable text output
│       ├── json.rs            # JSON output
│       └── github_actions.rs  # GitHub Actions annotations
│
├── tests/                     # Integration tests
│   └── fixtures/              # Test project fixtures
│
├── Cargo.toml
├── Cargo.lock
│
├── npm/                       # npm distribution packages
│   ├── grove/                 # Main package
│   │   └── package.json
│   └── grove-darwin-arm64/    # Platform-specific packages
│       └── package.json
│
├── specs/                     # Spec documentation
├── CLAUDE.md
└── README.md
```

---

## 9. Key Architectural Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Language | Rust | Best parser ecosystem (oxc), zero-cost abstractions, single binary |
| JS/TS parser | oxc_parser | Fastest, battle-tested, used by Rolldown |
| Module resolver | oxc_resolver | Same project as parser, handles all Node.js resolution rules |
| CLI framework | clap (derive) | Industry standard, zero boilerplate |
| Concurrency | rayon | Work-stealing parallelism, simple API |
| File walking | ignore crate | Respects .gitignore, parallel, from ripgrep |
| Config format | JSON (serde_json) | Simple, no extra deps |
| Output formats | Text, JSON, GitHub Actions | Cover human + CI + machine needs |
| Distribution | npm + platform-specific packages | Target audience uses npm, same as oxlint/biome |
| Error handling | thiserror (lib) + anyhow (bin) | Idiomatic Rust error handling split |
