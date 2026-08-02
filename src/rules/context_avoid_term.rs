use crate::model::context::parse_context;
use crate::model::{Project, Violation, relative_str};
use crate::rules::Rule;
use crate::rules::walk::walker;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

const EXCLUDED_DIRS: [&str; 5] = [".git", "target", "docs", "node_modules", "fixtures"];

const CODE_EXTS: [&str; 19] = [
    "rs", "py", "js", "ts", "jsx", "tsx", "go", "java", "c", "h", "cpp", "hpp", "cs", "rb", "php",
    "sh", "kt", "swift", "zig",
];

fn is_code_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| CODE_EXTS.contains(&e))
        .unwrap_or(false)
}

fn code_files(root: &Path) -> Vec<String> {
    walker(root, &EXCLUDED_DIRS, None)
        .flatten()
        .filter(|e| e.path().is_file() && is_code_file(e.path()))
        .map(|e| relative_str(e.path(), root))
        .collect()
}

fn word_pattern(word: &str) -> String {
    let words: Vec<&str> = word.split_whitespace().collect();
    if words.len() == 1 {
        format!(r"\b{}\b", regex::escape(words[0]))
    } else {
        format!(
            r"\b{}\b",
            words
                .iter()
                .map(|w| regex::escape(w))
                .collect::<Vec<_>>()
                .join(r"\s+")
        )
    }
}

pub struct ContextAvoidTermRule;

impl Rule for ContextAvoidTermRule {
    fn id(&self) -> &'static str {
        "context-avoid-term-violation"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let context_path = project.context();
        let content = match std::fs::read_to_string(&context_path) {
            Ok(c) => c,
            Err(_) => return v,
        };
        let ctx = parse_context(&content);
        let pairs = ctx.avoid_pairs();
        if pairs.is_empty() {
            return v;
        }

        let mut alt = Vec::new();
        for (i, (word, _)) in pairs.iter().enumerate() {
            alt.push(format!("(?P<g{}>{})", i, word_pattern(word)));
        }
        let re = match Regex::new(&format!("(?i)({})", alt.join("|"))) {
            Ok(r) => r,
            Err(_) => return v,
        };

        let mut seen: HashSet<String> = HashSet::new();
        for rel in code_files(&project.root) {
            let path = project.root.join(&rel);
            let file_content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            for (idx, line) in file_content.lines().enumerate() {
                if let Some(caps) = re.captures(line) {
                    let matched =
                        (0..pairs.len()).find(|&i| caps.name(&format!("g{}", i)).is_some());
                    if let Some(i) = matched {
                        let (word, canonical) = &pairs[i];
                        let key = format!("{}:{}:{}", rel, idx + 1, i);
                        if !seen.insert(key) {
                            continue;
                        }
                        v.push(Violation::error(
                            self.id(),
                            rel.clone(),
                            Some(idx + 1),
                            format!(
                                "Uses \"{}\" (CONTEXT.md marks this Avoid — use \"{}\")",
                                word, canonical
                            ),
                        ));
                    }
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::word_pattern;
    use regex::Regex;

    #[test]
    fn whole_word_matching_skips_compound_words() {
        let re = Regex::new(&format!("(?i){}", word_pattern("Alpha"))).unwrap();
        assert!(re.is_match("uses Alpha"));
        assert!(re.is_match("uses alpha"));
        assert!(!re.is_match("AlphaOrder"));
        assert!(!re.is_match("alpha_order"));
    }

    #[test]
    fn multi_word_pattern() {
        let re = Regex::new(&format!("(?i){}", word_pattern("Alpha Bravo"))).unwrap();
        assert!(re.is_match("a Alpha Bravo"));
        assert!(!re.is_match("a Alpha BravoX"));
    }
}
