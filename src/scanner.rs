use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

pub fn scan(root: &Path) -> Vec<PathBuf> {
    let walk = WalkBuilder::new(root).build();

    walk.filter_map(|e| e.ok())
        .filter(|entry| {
            // let mut v = false;
            // let e = entry.file_type();

            // if let Some(ent) = e {
            //     v = ent.is_file()
            // }

            // return v;

            entry.file_type().is_some_and(|ft| ft.is_file())
        })
        .filter(|f| {
            f.path().extension().is_some_and(|ext| {
                ext == "ts"
                    || ext == "tsx"
                    || ext == "js"
                    || ext == "jsx"
                    || ext == "mjs"
                    || ext == "mts"
                    || ext == "cts"
            })
        })
        .map(|f| f.into_path())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_directory() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scanner");
        let result = scan(&fixtures);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.ends_with("app.tsx")));
        assert!(result.iter().any(|p| p.ends_with("index.ts")));
    }
}
