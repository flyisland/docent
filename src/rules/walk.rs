use std::path::Path;

/// A directory walker applying docent's exclusion policy:
///
/// - respects the project's own `.gitignore` (and only that — never
///   `.git/info/exclude` or the global `core.excludesFile`, which are
///   machine-local and would make results differ between environments);
/// - skips hidden files and directories;
/// - never descends into `excluded_dirs` (the hardcoded fallback, applied even
///   when the tree is not a git repository or has no `.gitignore`).
///
/// Outside a git repository the walker naturally degrades to just the hidden +
/// hardcoded filters. `max_depth` bounds the traversal below the root (use
/// `Some(1)` to enumerate only the root's children without descending).
pub fn walker(
    root: &Path,
    excluded_dirs: &'static [&'static str],
    max_depth: Option<usize>,
) -> ignore::Walk {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.hidden(true);
    builder.git_ignore(true);
    builder.git_global(false);
    builder.git_exclude(false);
    builder.parents(false);
    builder.max_depth(max_depth);
    builder.filter_entry(move |entry| {
        let name = entry.file_name().to_str().unwrap_or_default();
        !excluded_dirs.contains(&name)
    });
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::walker;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn temp(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("docent-walk-test-{}-{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn walked_files(root: &Path, excluded: &'static [&'static str]) -> Vec<String> {
        let mut out: Vec<String> = walker(root, excluded, None)
            .flatten()
            .filter(|e| e.path().is_file())
            .map(|e| {
                e.path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        out.sort();
        out
    }

    #[test]
    fn gitignored_dir_is_skipped_in_git_repo() {
        let root = temp("gitignore");
        fs::create_dir_all(root.join(".git")).unwrap();
        write(&root, ".gitignore", "dist/\n");
        write(&root, "dist/bad.rs", "pub const PURCHASE: u8 = 1;");
        write(&root, "src/main.rs", "fn main() {}");
        assert_eq!(walked_files(&root, &[]), vec!["src/main.rs"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn git_info_exclude_is_not_respected() {
        let root = temp("info-exclude");
        fs::create_dir_all(root.join(".git/info")).unwrap();
        write(&root, ".git/info/exclude", "dist/\n");
        write(&root, "dist/bad.rs", "pub const PURCHASE: u8 = 1;");
        write(&root, "src/main.rs", "fn main() {}");
        assert_eq!(walked_files(&root, &[]), vec!["dist/bad.rs", "src/main.rs"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn hidden_entries_are_skipped() {
        let root = temp("hidden");
        write(&root, ".hidden/bad.rs", "fn hidden() {}");
        write(&root, "src/main.rs", "fn main() {}");
        assert_eq!(walked_files(&root, &[]), vec!["src/main.rs"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn hardcoded_excluded_dirs_apply_without_git() {
        let root = temp("hardcoded");
        write(&root, "dist/bad.rs", "fn dist() {}");
        write(&root, "node_modules/bad.rs", "fn nm() {}");
        write(&root, "src/main.rs", "fn main() {}");
        assert_eq!(
            walked_files(&root, &["node_modules"]),
            vec!["dist/bad.rs", "src/main.rs"]
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn max_depth_bounds_traversal() {
        let root = temp("max-depth");
        write(&root, "top.rs", "fn top() {}");
        write(&root, "a/deep.rs", "fn deep() {}");
        let mut out: Vec<String> = walker(&root, &[], Some(1))
            .flatten()
            .filter(|e| e.path().is_file())
            .map(|e| {
                e.path()
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        out.sort();
        assert_eq!(out, vec!["top.rs"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn gitignore_requires_git_repo() {
        let root = temp("no-git");
        write(&root, ".gitignore", "dist/\n");
        write(&root, "dist/bad.rs", "pub const PURCHASE: u8 = 1;");
        write(&root, "src/main.rs", "fn main() {}");
        assert_eq!(walked_files(&root, &[]), vec!["dist/bad.rs", "src/main.rs"]);
        let _ = fs::remove_dir_all(&root);
    }
}
