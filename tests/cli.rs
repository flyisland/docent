use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn docent(dir: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_docent"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run docent");
    let code = out.status.code().unwrap_or(-1);
    (code, String::from_utf8_lossy(&out.stdout).into_owned())
}

fn lint_json(dir: &Path) -> Value {
    let (_code, stdout) = docent(dir, &["lint", "--json"]);
    serde_json::from_str(&stdout).expect("docent --json must emit valid JSON")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn count_by_rule(v: &Value, key: &str, rule: &str) -> usize {
    v[key]
        .as_array()
        .map(|arr| arr.iter().filter(|x| x["rule"] == rule).count())
        .unwrap_or(0)
}

#[test]
fn valid_project_has_zero_violations() {
    let dir = fixture("valid-project");
    let (code, stdout) = docent(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("0 errors, 0 warnings"),
        "unexpected output: {}",
        stdout
    );

    let json = lint_json(&dir);
    assert_eq!(json["summary"]["errors"], 0);
    assert_eq!(json["summary"]["warnings"], 0);
}

#[test]
fn json_output_has_required_structure() {
    let json = lint_json(&fixture("broken-project"));
    assert!(json.get("errors").is_some_and(|v| v.is_array()));
    assert!(json.get("warnings").is_some_and(|v| v.is_array()));
    assert!(json.get("summary").is_some());
    assert_eq!(json["errors"][0]["rule"].as_str().is_some(), true);
    let line = &json["errors"][0]["line"];
    assert!(
        line.is_null() || line.is_number(),
        "line must be null or a number"
    );
}

#[test]
fn broken_project_reports_one_violation_per_rule() {
    let json = lint_json(&fixture("broken-project"));
    let errors = json["summary"]["errors"].as_u64().unwrap();
    let warnings = json["summary"]["warnings"].as_u64().unwrap();
    assert_eq!(errors, 7);
    assert_eq!(warnings, 3);

    let expected_errors = [
        "frontmatter-schema-valid",
        "rfc-index-sync",
        "adr-index-sync",
        "adr-missing-required-sections",
        "superseded-backlink-consistency",
        "agents-adr-reference-valid",
        "context-avoid-term-violation",
    ];
    for rule in expected_errors {
        assert_eq!(
            count_by_rule(&json, "errors", rule),
            1,
            "rule {} should fire exactly once in broken-project",
            rule
        );
    }
    let expected_warnings = [
        "architecture-module-sync",
        "adr-pending-implementation-report",
        "rfc-stale-draft",
    ];
    for rule in expected_warnings {
        assert_eq!(
            count_by_rule(&json, "warnings", rule),
            1,
            "rule {} should fire exactly once in broken-project",
            rule
        );
    }
}

fn doc_group(file: &str) -> u8 {
    if file == "IDEAS.md" {
        0
    } else if file.starts_with("docs/rfcs") {
        1
    } else if file.starts_with("docs/adrs") {
        2
    } else if file == "docs/architecture.md" {
        3
    } else if file == "AGENTS.md" {
        4
    } else {
        5
    }
}

#[test]
fn lint_output_is_grouped_in_status_order() {
    let json = lint_json(&fixture("broken-project"));
    for key in ["errors", "warnings"] {
        let groups: Vec<u8> = json[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| doc_group(v["file"].as_str().unwrap()))
            .collect();
        assert!(
            groups.windows(2).all(|w| w[0] <= w[1]),
            "{} not grouped in status order (IDEAS, RFC, ADR, Architecture, AGENTS, code): {:?}",
            key,
            groups
        );
    }
}

#[test]
fn lint_exit_code_is_one_when_errors_present() {
    let (code, _) = docent(&fixture("broken-project"), &["lint"]);
    assert_eq!(code, 1);
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("docent-test-{}-{}", std::process::id(), tag));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn init_creates_template_files_and_skips_existing() {
    let dir = temp_dir("init");
    fs::create_dir_all(&dir).unwrap();
    let (code, stdout) = docent(&dir, &["init"]);
    assert_eq!(code, 0);
    for rel in [
        "IDEAS.md",
        "docs/rfcs/README.md",
        "docs/adrs/README.md",
        "docs/architecture.md",
        "AGENTS.md",
        "docs/.templates/rfc.md",
        "docs/.templates/adr.md",
        "docs/.templates/context.md",
    ] {
        assert!(dir.join(rel).exists(), "{} was not created by init", rel);
    }

    let adr_template = fs::read_to_string(dir.join("docs/.templates/adr.md")).unwrap();
    assert!(adr_template.contains("## Non-goals"));

    let agents = fs::read_to_string(dir.join("AGENTS.md")).unwrap();
    assert!(agents.contains("docs/.templates/"));

    let (code, stdout) = docent(&dir, &["init"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("skipped"),
        "second init should skip existing files: {}",
        stdout
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn status_lists_violation_counts_by_rule() {
    let (code, stdout) = docent(&fixture("broken-project"), &["status"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("Violations by rule") && stdout.contains("10 total"),
        "expected a per-rule breakdown: {}",
        stdout
    );
    for rule in [
        "frontmatter-schema-valid",
        "rfc-index-sync",
        "adr-index-sync",
        "adr-missing-required-sections",
        "superseded-backlink-consistency",
        "agents-adr-reference-valid",
        "architecture-module-sync",
        "context-avoid-term-violation",
        "adr-pending-implementation-report",
        "rfc-stale-draft",
    ] {
        assert!(
            stdout.contains(rule),
            "rule {} missing from breakdown: {}",
            rule,
            stdout
        );
    }
}

#[test]
fn lint_reports_missing_required_sources() {
    let dir = temp_dir("lint-missing");
    fs::create_dir_all(&dir).unwrap();
    let json = lint_json(&dir);
    let mut missing: Vec<String> = json["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| x["rule"] == "required-source-missing")
        .map(|x| x["file"].as_str().unwrap().to_string())
        .collect();
    missing.sort();
    assert_eq!(
        missing,
        vec![
            "AGENTS.md",
            "docs/adrs",
            "docs/architecture.md",
            "docs/rfcs",
        ]
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn status_reports_missing_sources_explicitly() {
    let dir = temp_dir("status-missing");
    fs::create_dir_all(&dir).unwrap();
    let (code, stdout) = docent(&dir, &["status"]);
    assert_eq!(code, 0);
    for needle in [
        "not found (IDEAS.md)",
        "not found (docs/rfcs)",
        "not found (docs/adrs)",
        "not found (docs/architecture.md)",
        "not found (CONTEXT.md)",
    ] {
        assert!(
            stdout.contains(needle),
            "{} missing from output: {}",
            needle,
            stdout
        );
    }
    assert!(
        !stdout.contains("unclaimed entries"),
        "counts must not be reported when the source is missing: {}",
        stdout
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn fix_regenerates_index_and_restores_zero_errors() {
    let src = fixture("valid-project");
    let dir = temp_dir("fix");
    copy_tree(&src, &dir);

    let index = dir.join("docs/adrs/README.md");
    let broken = fs::read_to_string(&index)
        .unwrap()
        .replace("Cache strategy selection", "Deliberately wrong title");
    fs::write(&index, broken).unwrap();

    let (code, stdout) = docent(&dir, &["lint"]);
    assert_eq!(code, 1);
    assert!(
        stdout.contains("adr-index-sync"),
        "expected an index desync error: {}",
        stdout
    );

    let (code, _) = docent(&dir, &["lint", "--fix"]);
    assert_eq!(code, 0);

    let (code, stdout) = docent(&dir, &["lint"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("0 errors, 0 warnings"),
        "after --fix lint should be clean: {}",
        stdout
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn fix_round_trips_superseded_backlinks() {
    let src = fixture("broken-project");
    let dir = temp_dir("backlink");
    copy_tree(&src, &dir);

    let adr = dir.join("docs/adrs/adr-002-payment-retry-policy.md");
    let content = fs::read_to_string(&adr).unwrap();
    assert!(
        !content.contains("superseded_by: adr-003"),
        "fixture must start with a missing backlink"
    );

    let (_, _) = docent(&dir, &["lint", "--fix"]);

    let fixed = fs::read_to_string(&adr).unwrap();
    assert!(
        fixed.contains("superseded_by: adr-003"),
        "missing backlink should be filled: {}",
        fixed
    );
    let _ = fs::remove_dir_all(&dir);
}
