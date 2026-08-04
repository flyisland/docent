pub mod adr;
pub mod index;
pub mod rfc;

use regex::Regex;
use std::path::{Path, PathBuf};

pub const ADR_DIR: &str = "docs/adrs";
pub const RFC_DIR: &str = "docs/rfcs";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Severity {
    Error,
    Warning,
}

pub struct Violation {
    pub rule: &'static str,
    pub file: String,
    pub line: Option<usize>,
    pub message: String,
    pub severity: Severity,
}

impl Violation {
    pub fn error(
        rule: &'static str,
        file: String,
        line: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Violation {
            rule,
            file,
            line,
            message: message.into(),
            severity: Severity::Error,
        }
    }

    pub fn warning(
        rule: &'static str,
        file: String,
        line: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Violation {
            rule,
            file,
            line,
            message: message.into(),
            severity: Severity::Warning,
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

pub struct Project {
    pub root: PathBuf,
}

impl Project {
    pub fn new(root: PathBuf) -> Self {
        Project { root }
    }

    pub fn path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    pub fn adr_index(&self) -> PathBuf {
        self.path("docs/adrs/README.md")
    }

    pub fn rfc_index(&self) -> PathBuf {
        self.path("docs/rfcs/README.md")
    }

    pub fn architecture(&self) -> PathBuf {
        self.path("docs/architecture.md")
    }

    pub fn agents(&self) -> PathBuf {
        self.path("AGENTS.md")
    }

    pub fn ideas(&self) -> PathBuf {
        self.path("docs/IDEAS.md")
    }
}

pub fn relative_str(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}

pub enum FrontSplit {
    Ok { yaml: String, body: String },
    Missing,
    Unclosed,
}

pub fn split_front_matter(content: &str) -> FrontSplit {
    let lines: Vec<&str> = content.lines().collect();
    if lines.first().map(|l| l.trim()) != Some("---") {
        return FrontSplit::Missing;
    }
    let mut idx = 1;
    let mut yaml_lines = Vec::new();
    while idx < lines.len() {
        if lines[idx].trim() == "---" {
            break;
        }
        yaml_lines.push(lines[idx]);
        idx += 1;
    }
    if idx >= lines.len() {
        return FrontSplit::Unclosed;
    }
    let body = lines[(idx + 1)..].join("\n");
    FrontSplit::Ok {
        yaml: yaml_lines.join("\n"),
        body,
    }
}

pub struct Loaded<T> {
    pub rel: String,
    pub front: Option<T>,
    pub parse_error: Option<String>,
    pub body: String,
}

pub fn load_docs<T>(root: &Path, dir_rel: &str, exclude: &[&str]) -> Vec<Loaded<T>>
where
    T: serde::de::DeserializeOwned,
{
    let dir = root.join(dir_rel);
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if exclude.contains(&name) {
            continue;
        }
        if !name.ends_with(".md") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let rel = relative_str(&path, root);
        let (front, parse_error, body) = match split_front_matter(&content) {
            FrontSplit::Ok { yaml, body } => match serde_yaml::from_str::<T>(&yaml) {
                Ok(f) => (Some(f), None, body),
                Err(e) => (None, Some(e.to_string()), body),
            },
            FrontSplit::Missing => (
                None,
                Some("missing front matter (file must start with ---)".to_string()),
                content.clone(),
            ),
            FrontSplit::Unclosed => (
                None,
                Some("front matter has no closing --- delimiter".to_string()),
                content.clone(),
            ),
        };
        out.push(Loaded {
            rel,
            front,
            parse_error,
            body,
        });
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    out
}

pub fn id_from_filename(name: &str) -> Option<String> {
    let re = Regex::new(r"^(adr|rfc)-(\d{3,4})(?:-[^.]+)?\.md$").unwrap();
    re.captures(name).map(|c| format!("{}-{}", &c[1], &c[2]))
}

pub fn id_from_rel(rel: &str) -> Option<String> {
    let name = Path::new(rel).file_name().and_then(|n| n.to_str())?;
    id_from_filename(name)
}

pub fn valid_date(s: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !re.is_match(s) {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    let y: i64 = parts[0].parse().unwrap_or(0);
    let m: i64 = parts[1].parse().unwrap_or(0);
    let d: i64 = parts[2].parse().unwrap_or(0);
    if !(1970..=9999).contains(&y) {
        return false;
    }
    if !(1..=12).contains(&m) {
        return false;
    }
    if !(1..=31).contains(&d) {
        return false;
    }
    true
}

pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

pub fn date_to_days(s: &str) -> Option<i64> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let y: i64 = parts[0].parse().ok()?;
    let m: i64 = parts[1].parse().ok()?;
    let d: i64 = parts[2].parse().ok()?;
    Some(days_from_civil(y, m, d))
}

pub fn today_days() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    (secs / 86_400) as i64
}
