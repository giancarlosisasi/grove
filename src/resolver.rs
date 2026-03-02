use std::path::{Path, PathBuf};

use oxc_resolver::{ResolveOptions, Resolver};

pub fn resolve(resolver: &Resolver, directory: &Path, specifier: &str) -> Option<PathBuf> {
    let result = resolver.resolve(directory, specifier).ok()?;

    Some(result.into_path_buf())
}

pub fn create_resolver() -> Resolver {
    let options = ResolveOptions {
        extensions: vec![
            ".ts".into(),
            ".tsx".into(),
            ".js".into(),
            ".jsx".into(),
            ".mjs".into(),
            ".json".into(),
            ".mts".into(),
            ".cts".into(),
        ],
        ..ResolveOptions::default()
    };

    Resolver::new(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_relative_ts() {
        let resolver = create_resolver();
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/resolver");

        let result = resolve(&resolver, &fixtures, "./a");

        assert!(result.is_some());
        assert!(result.unwrap().ends_with("a.ts"));
    }
}
