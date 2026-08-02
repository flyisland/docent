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
    pub related_rfc: Option<String>,
}

static ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^adr-\d{3,4}$").unwrap());

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
        errs
    }

    pub fn index_date(&self) -> String {
        self.updated.clone().unwrap_or_else(|| self.created.clone())
    }
}
