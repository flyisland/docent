use crate::model::{Severity, Violation};
use owo_colors::OwoColorize;
use owo_colors::Stream;

pub fn render(violations: &[Violation]) -> String {
    let mut out = String::new();
    for v in violations {
        let file = match v.line {
            Some(l) => format!("{}:{}", v.file, l),
            None => v.file.clone(),
        };
        match v.severity {
            Severity::Error => {
                out.push_str(&format!(
                    "{} {:<42} [{}] {}\n",
                    "✗".if_supports_color(Stream::Stdout, |s| s.red()),
                    file,
                    v.rule,
                    v.message
                ));
            }
            Severity::Warning => {
                out.push_str(&format!(
                    "{} {:<42} [{}] {}\n",
                    "⚠".if_supports_color(Stream::Stdout, |s| s.yellow()),
                    file,
                    v.rule,
                    v.message
                ));
            }
        }
    }
    let errors = violations.iter().filter(|v| v.is_error()).count();
    let warnings = violations.iter().filter(|v| !v.is_error()).count();
    out.push_str(&format!(
        "{} {}, {} {}",
        errors,
        if errors == 1 { "error" } else { "errors" },
        warnings,
        if warnings == 1 { "warning" } else { "warnings" },
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::model::Violation;

    #[test]
    fn renders_summary() {
        let v = vec![
            Violation::error(
                "adr-missing-required-sections",
                "docs/adrs/adr-001.md".into(),
                None,
                "Missing Non-goals section",
            ),
            Violation::warning(
                "rfc-stale-draft",
                "docs/rfcs/rfc-001.md".into(),
                None,
                "stale",
            ),
        ];
        let out = render(&v);
        assert!(out.contains("1 error"));
        assert!(out.contains("1 warning"));
    }
}
