use crate::model::valid_date;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AdrStatus {
    Accepted,
    Superseded,
    Deprecated,
}

impl AdrStatus {
    pub fn display(&self) -> &'static str {
        match self {
            AdrStatus::Accepted => "Accepted",
            AdrStatus::Superseded => "Superseded",
            AdrStatus::Deprecated => "Deprecated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Implementation {
    Implemented,
    Pending,
}

impl Implementation {
    pub fn display(&self) -> &'static str {
        match self {
            Implementation::Implemented => "Implemented",
            Implementation::Pending => "Pending",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdrAmendment {
    pub adr: String,
    pub decision: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdrFront {
    pub id: String,
    pub title: String,
    pub status: AdrStatus,
    pub implementation: Implementation,
    pub created: String,
    pub updated: Option<String>,
    #[serde(default)]
    pub supersedes: Vec<String>,
    #[serde(default)]
    pub superseded_by: Option<String>,
    #[serde(default)]
    pub amends: Vec<AdrAmendment>,
    #[serde(default)]
    pub amended_by: Vec<String>,
    #[serde(default)]
    pub related_rfc: Option<String>,
}

static ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^adr-\d{3,4}$").unwrap());
static DECISION_SCOPE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$").unwrap());

impl AdrFront {
    pub fn validation_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if !ID_RE.is_match(&self.id) {
            errs.push(format!(
                "id '{}' does not match the adr-\\d{{3}} or adr-\\d{{4}} form",
                self.id
            ));
        }
        if self.title.trim().is_empty() {
            errs.push("title must not be empty".to_string());
        }
        if !valid_date(&self.created) {
            errs.push(format!(
                "created '{}' is not a valid YYYY-MM-DD date",
                self.created
            ));
        }
        if let Some(u) = &self.updated
            && !valid_date(u)
        {
            errs.push(format!("updated '{}' is not a valid YYYY-MM-DD date", u));
        }
        let mut amendment_scopes = std::collections::HashSet::new();
        for amendment in &self.amends {
            if !ID_RE.is_match(&amendment.adr) {
                errs.push(format!(
                    "amends ADR '{}' does not match the adr-\\d{{3}} or adr-\\d{{4}} form",
                    amendment.adr
                ));
            }
            if amendment.decision.trim().is_empty() {
                errs.push(format!(
                    "amends entry for {} must name a non-empty decision scope",
                    amendment.adr
                ));
            } else if !DECISION_SCOPE_RE.is_match(&amendment.decision) {
                errs.push(format!(
                    "amends decision '{}' must be a lowercase kebab-case scope name",
                    amendment.decision
                ));
            }
            if amendment.adr == self.id {
                errs.push("an ADR cannot amend itself".to_string());
            }
            if self.supersedes.contains(&amendment.adr) {
                errs.push(format!(
                    "{} cannot be listed in both supersedes and amends",
                    amendment.adr
                ));
            }
            if !amendment_scopes.insert((&amendment.adr, &amendment.decision)) {
                errs.push(format!(
                    "duplicate amends entry for {} decision '{}'",
                    amendment.adr, amendment.decision
                ));
            }
        }
        let mut amended_by = std::collections::HashSet::new();
        for adr in &self.amended_by {
            if !ID_RE.is_match(adr) {
                errs.push(format!(
                    "amended_by ADR '{}' does not match the adr-\\d{{3}} or adr-\\d{{4}} form",
                    adr
                ));
            }
            if adr == &self.id {
                errs.push("an ADR cannot be amended by itself".to_string());
            }
            if !amended_by.insert(adr) {
                errs.push(format!("duplicate amended_by entry for {}", adr));
            }
        }
        errs
    }

    pub fn index_date(&self) -> String {
        self.updated.clone().unwrap_or_else(|| self.created.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::AdrFront;

    #[test]
    fn partial_amendments_require_distinct_valid_scopes() {
        let front: AdrFront = serde_yaml::from_str(
            r#"
id: adr-003
title: Invalid partial amendment
status: Accepted
implementation: implemented
created: 2026-08-03
updated: null
supersedes: [adr-001]
superseded_by: null
amends:
- adr: adr-001
  decision: Documentation Layout
- adr: adr-003
  decision: self
amended_by: [adr-004, adr-004]
related_rfc: null
"#,
        )
        .unwrap();

        let errors = front.validation_errors();
        assert!(
            errors
                .iter()
                .any(|error| error.contains("lowercase kebab-case"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("both supersedes and amends"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("cannot amend itself"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("duplicate amended_by"))
        );
    }
}
