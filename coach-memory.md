# Grove Coach Memory

## Who Am I
I am a Rust learning coach for Giancarlos. My purpose is to guide him step-by-step through building **Grove**, a Rust CLI tool that analyzes JavaScript/TypeScript projects. I teach through incremental tasks, providing insights about Rust concepts along the way. I request human contributions for meaningful design decisions to maximize learning.

## Coaching Rules
- **I DO NOT write code or modify files.** The human writes ALL code.
- I guide, explain commands, provide insights, and point to the right direction
- I explain concepts while we build — not in abstract, but connected to the task
- **Assume ZERO Rust knowledge.** Every new term/syntax must be explained with what it is, why it exists, and a small example
- I use "Learn by Doing" requests for key design decisions
- I tell the human what commands to run and explain what they do
- Each task should be small enough to complete in one session
- I ask the human to show me their code so I can review and give feedback

## Project: Grove
- **What:** Rust CLI for JS/TS static analysis (unused code + circular deps detection)
- **Inspired by:** knip (unused detection) + dpdm (circular deps)
- **Stack:** Rust 1.93.1, oxc (parser + resolver), clap (CLI), rayon (concurrency), ignore (file walking)
- **Distribution:** npm with platform-specific packages
- **Architecture:** lib.rs + main.rs (library + binary, for testability)
- **Specs:** `specs/circular-deps/`, `specs/unused-modules/`, `specs/cli/`
- **Research:** `research/knip-features.md`, `research/dpdm-features.md`

## Build Strategy (Incremental Phases)

### Phase 1: Foundation
- [x] Task 1.1: CLI skeleton with clap (subcommands: circular, unused)
- [ ] Task 1.2: JS/TS parser wrapper — extract imports from a single file (oxc_parser)
- [ ] Task 1.3: Module resolver wrapper — resolve import specifiers to file paths (oxc_resolver)
- [ ] Task 1.4: File scanner — walk directories, respect .gitignore (ignore crate)

### Phase 2: Circular Dependency Detection (first complete feature)
- [ ] Task 2.1: Dependency graph data structure
- [ ] Task 2.2: Graph builder — parse entry point, resolve imports, recurse
- [ ] Task 2.3: Circular dependency detection (DFS cycle finding)
- [ ] Task 2.4: Text reporter for circular deps
- [ ] Task 2.5: Exit codes + integration test with fixture project

### Phase 3: Unused Module Detection
- [ ] Task 3.1: Unused files detection (compare graph vs project files)
- [ ] Task 3.2: Unused exports detection (track which exports are imported)
- [ ] Task 3.3: Unused dependencies detection (parse package.json, compare)
- [ ] Task 3.4: Text reporter for unused command
- [ ] Task 3.5: Exit codes + integration tests

### Phase 4: Polish & Distribution
- [ ] Task 4.1: JSON output format
- [ ] Task 4.2: Config file support (grove.config.json)
- [ ] Task 4.3: --verbose, --no-progress, --directory flags
- [ ] Task 4.4: Error handling improvements (per spec)
- [ ] Task 4.5: npm distribution setup

## Current Status
- **Session:** 1
- **Current Task:** Task 1.2 — JS/TS parser wrapper (oxc_parser)
- **Branch:** mvp
- **What exists:** main.rs with match on commands, lib.rs with Cli struct + Commands enum, Cargo.toml with clap

## Rust Concepts Tracker
Every concept introduced gets explained and checked off.

### Fundamentals
- [x] Cargo (package manager + build tool) — like npm for Rust
- [x] Cargo.toml — like package.json, declares deps and metadata
- [x] `cargo add` — like `npm install`, adds a dependency
- [x] Crate — a Rust package (library or binary), like an npm package
- [x] `main.rs` vs `lib.rs` — binary entry point vs library root
- [x] Features (in Cargo) — optional functionality in a crate, like feature flags
- [x] Structs — custom data types that group fields together (like a TS interface/class)
- [x] Enums — types that can be one of several variants (like TS union types, but more powerful)
- [x] `fn` — how to define functions
- [x] `let` / `let mut` — variable bindings (immutable by default!)
- [x] `pub` — visibility modifier (everything is private by default)
- [ ] `mod` — module system (how files/folders become modules)
- [x] `use` — importing items from other modules/crates
- [x] Type annotations — `x: String`, `y: bool`, etc.

### Ownership & Borrowing (Rust's unique feature)
- [ ] Ownership — every value has exactly one owner
- [ ] Borrowing — `&x` (shared ref) vs `&mut x` (exclusive ref)
- [ ] Lifetimes — how Rust tracks how long references are valid
- [ ] Clone vs Copy — duplicating values
- [ ] `String` vs `&str` — owned string vs borrowed string slice

### Error Handling
- [ ] `Result<T, E>` — function that can succeed (Ok) or fail (Err)
- [ ] `Option<T>` — value that might exist (Some) or not (None)
- [ ] `?` operator — early return on error (like try/catch shorthand)
- [ ] `unwrap()` — get the value or panic (avoid in production!)
- [ ] `thiserror` — derive macro for custom error types
- [ ] `anyhow` — flexible error handling for applications

### Traits & Generics
- [ ] Traits — like TS interfaces, define shared behavior
- [ ] `impl` blocks — how you add methods to structs/enums
- [x] `#[derive(...)]` — auto-implement common traits
- [ ] Generics — `<T>` type parameters
- [ ] `impl Into<String>` — accepting multiple types that convert to String

### Collections & Iterators
- [ ] `Vec<T>` — growable array (like JS Array)
- [ ] `HashMap<K, V>` — key-value map (like JS Map/Object)
- [ ] `HashSet<T>` — unique values (like JS Set)
- [ ] Iterators — `.iter()`, `.map()`, `.filter()`, `.collect()`
- [ ] `for` loops — iterating over collections

### Pattern Matching
- [x] `match` — exhaustive pattern matching (like switch but way more powerful)
- [ ] `if let` — match a single pattern
- [ ] Destructuring — pulling values out of structs/enums/tuples

### Concurrency
- [ ] `rayon` — parallel iterators (`.par_iter()`)
- [ ] Threads — `std::thread::spawn`
- [ ] `Send` / `Sync` — traits that mark types safe for threading

### CLI & I/O
- [x] `clap` derive macros — `#[derive(Parser)]`, `#[derive(Subcommand)]`
- [ ] `std::fs` — file system operations
- [ ] `std::path::PathBuf` — cross-platform file paths
- [ ] `println!` / `eprintln!` — macros for stdout/stderr
- [ ] Process exit codes — `std::process::exit()`

### Testing
- [ ] `#[test]` — marking test functions
- [ ] `#[cfg(test)]` — conditional compilation for test modules
- [ ] `assert!`, `assert_eq!` — test assertions
- [ ] Integration tests in `tests/` directory
- [ ] Test fixtures

### Advanced (later phases)
- [ ] Closures — anonymous functions `|x| x + 1`
- [ ] `serde` — serialization/deserialization (for JSON)
- [ ] Trait objects — `Box<dyn Trait>` for dynamic dispatch
- [ ] `impl Trait` in return position
