pub struct IndexTable {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl IndexTable {
    pub fn col(&self, name: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|c| c.eq_ignore_ascii_case(name))
    }
}

pub fn parse_index_table(content: &str) -> Option<IndexTable> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with('|') {
            continue;
        }
        let trimmed = t.trim_matches('|');
        let cells: Vec<String> = trimmed.split('|').map(|c| c.trim().to_string()).collect();
        if cells.iter().all(|c| c.is_empty()) {
            continue;
        }
        if cells
            .iter()
            .all(|c| c.chars().all(|ch| ch == '-' || ch == ':' || ch == ' '))
        {
            continue;
        }
        rows.push(cells);
    }
    if rows.is_empty() {
        return None;
    }
    let columns = rows.remove(0);
    Some(IndexTable { columns, rows })
}

pub fn render_adr_index(rows: &[(String, String, String, String, String)]) -> String {
    let mut s = String::from(
        "# Table of ADRs\n\n|ID|Title|Status|Implementation|Last Updated|\n|---|---|---|---|---|\n",
    );
    for (id, title, status, impl_, date) in rows {
        s.push_str(&format!(
            "|{}|{}|{}|{}|{}|\n",
            id, title, status, impl_, date
        ));
    }
    s
}

pub fn render_rfc_index(rows: &[(String, String, String, String, String)]) -> String {
    let mut s = String::from(
        "# Table of RFCs\n\n|ID|Title|Status|Date|Linked ADR|\n|---|---|---|---|---|\n",
    );
    for (id, title, status, date, linked) in rows {
        s.push_str(&format!(
            "|{}|{}|{}|{}|{}|\n",
            id, title, status, date, linked
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::{parse_index_table, render_adr_index};

    #[test]
    fn parses_adr_index_table() {
        let content = "# Table of ADRs\n\n|ID|Title|Status|Implementation|Last Updated|\n|---|---|---|---|---|\n|adr-001|Initial architecture selection|Accepted|Pending|2026-08-01|\n";
        let table = parse_index_table(content).unwrap();
        assert_eq!(
            table.columns,
            vec!["ID", "Title", "Status", "Implementation", "Last Updated"]
        );
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0][0], "adr-001");
        assert_eq!(table.col("Last Updated"), Some(4));
    }

    #[test]
    fn render_round_trip() {
        let rows = vec![(
            "adr-001".to_string(),
            "Initial architecture selection".to_string(),
            "Accepted".to_string(),
            "Implemented".to_string(),
            "2026-08-01".to_string(),
        )];
        let rendered = render_adr_index(&rows);
        let table = parse_index_table(&rendered).unwrap();
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0][0], "adr-001");
        assert_eq!(table.rows[0][1], "Initial architecture selection");
    }
}
