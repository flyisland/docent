use crate::model::{Severity, Violation};
use serde::Serialize;

#[derive(Serialize)]
struct JsonViolation {
    rule: &'static str,
    file: String,
    line: Option<usize>,
    message: String,
}

#[derive(Serialize)]
struct JsonSummary {
    errors: usize,
    warnings: usize,
}

#[derive(Serialize)]
struct JsonOut {
    errors: Vec<JsonViolation>,
    warnings: Vec<JsonViolation>,
    summary: JsonSummary,
}

pub fn render(violations: &[Violation]) -> String {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for v in violations {
        let jv = JsonViolation {
            rule: v.rule,
            file: v.file.clone(),
            line: v.line,
            message: v.message.clone(),
        };
        match v.severity {
            Severity::Error => errors.push(jv),
            Severity::Warning => warnings.push(jv),
        }
    }
    let out = JsonOut {
        errors,
        warnings,
        summary: JsonSummary {
            errors: violations.iter().filter(|v| v.is_error()).count(),
            warnings: violations.iter().filter(|v| !v.is_error()).count(),
        },
    };
    serde_json::to_string_pretty(&out).unwrap_or_else(|_| "{}".to_string())
}
