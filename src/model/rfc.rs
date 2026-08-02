use crate::model::valid_date;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum RfcStatus {
    Draft,
    Accepted,
    Rejected,
}

impl RfcStatus {
    pub fn display(&self) -> &'static str {
        match self {
            RfcStatus::Draft => "Draft",
            RfcStatus::Accepted => "Accepted",
            RfcStatus::Rejected => "Rejected",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RfcFront {
    pub id: String,
    pub title: String,
    pub status: RfcStatus,
    pub created: String,
    pub updated: Option<String>,
    #[serde(default)]
    pub related_adr: Option<String>,
}

static ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^rfc-\d{3}$").unwrap());

impl RfcFront {
    pub fn validation_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if !ID_RE.is_match(&self.id) {
            errs.push(format!(
                "id '{}' does not match the rfc-\\d{{3}} form",
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
