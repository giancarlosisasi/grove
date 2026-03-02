
# CLAUDE.md — Rust Best Practices

## Code Style

- We are building a Rust CLI project that will be distributed on npm.
- We are using Rust version 1.93.1 (Feb. 12, 2026)
- Write idiomatic Rust — prefer clarity over cleverness.
- No global mutable state; pass dependencies explicitly.
- Use `Result<T, E>` for fallible operations — avoid `.unwrap()` in library/production code.
- Keep functions short (~20-40 lines max); extract when complexity grows.
- Prefer traits for abstraction; define traits where they're used.
- Keep traits small and focused (1-3 methods).

## Naming

- Use `snake_case` for functions, methods, variables, and modules.
- Use `CamelCase` for types, traits, and enums.
- Use `SCREAMING_SNAKE_CASE` for constants.
- Short, clear names: `cfg` not `configuration`, `buf` not `buffer`.
- Avoid stuttering: `parser::Parser` → `parser::Instance`.
- Acronyms follow Rust conventions: `Id`, `Url`, `Ast` in CamelCase; `id`, `url`, `ast` in snake_case.

## Error Handling

```rust
// Use thiserror for library errors
#[derive(Debug, thiserror::Error)]
pub enum GroveError {
    #[error("parsing {path}: {source}")]
    Parse { path: PathBuf, source: oxc::diagnostics::Error },

    #[error("reading {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },

    #[error("config not found: {0}")]
    ConfigNotFound(PathBuf),
}

// Use anyhow for application-level (CLI) errors
fn main() -> anyhow::Result<()> {
    // ...
}

// Always add context when propagating
let content = std::fs::read_to_string(&path)
    .with_context(|| format!("reading {}", path.display()))?;
```

## Struct Design

```rust
// Use the builder pattern for flexible configuration
pub struct Config {
    entry: Vec<String>,
    ignore: Vec<String>,
    concurrency: usize,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder { /* ... */ }

impl ConfigBuilder {
    pub fn entry(mut self, e: impl Into<String>) -> Self {
        self.entry.push(e.into());
        self
    }
    pub fn build(self) -> Config { /* ... */ }
}
```

## Concurrency

- Use `rayon` for CPU-bound parallel iteration (file parsing).
- Use scoped threads or `rayon::scope` instead of spawning raw threads.
- Prefer `crossbeam` channels over `std::sync::mpsc` when needed.
- Avoid shared mutable state; prefer message passing or `DashMap` when necessary.

```rust
use rayon::prelude::*;

let results: Vec<ParseResult> = file_paths
    .par_iter()
    .map(|path| parse_file(path))
    .collect::<Result<Vec<_>, _>>()?;
```

## Performance

- Profile before optimizing: `cargo bench`, `cargo flamegraph`.
- Preallocate collections: `Vec::with_capacity(expected_len)`.
- Use `&str` and borrows over cloning where possible.
- Stream large data; avoid loading entirely into memory.
- Reuse buffers; avoid allocations in hot paths.
- Use `String::with_capacity` for string building.

## Testing

- Use `#[cfg(test)]` modules for unit tests.
- Use integration tests in `tests/` directory.
- Test public API primarily.
- Use `testdata/` or fixtures in `tests/fixtures/` for test projects.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_esm_imports() {
        let source = r#"import { foo } from "./bar";"#;
        let result = parse(source, "test.ts").unwrap();
        assert_eq!(result.imports.len(), 1);
        assert_eq!(result.imports[0].specifier, "./bar");
    }

    #[test]
    fn test_parse_error() {
        let source = "import {";
        let result = parse(source, "test.ts");
        assert!(result.is_err());
    }
}
```

## Logging

- Use `tracing` for structured, leveled logging.
- Debug for verbose, Info for user-facing.

```rust
tracing::debug!(key = %value, "processing");
tracing::error!(?err, "failed");
```

## Avoid

- `.unwrap()` / `.expect()` in production code — use `?` operator.
- `unsafe` blocks unless absolutely necessary and well-documented.
- Returning `Box<dyn Any>` — define concrete types.
- Ignoring `Result` values — always handle or explicitly discard with `let _ =`.
- Deep nesting — use early returns and `?` to flatten logic.
- `clone()` when a borrow would suffice.
