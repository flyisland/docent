use regex::Regex;

pub struct Term {
    pub name: String,
    pub avoid: Vec<String>,
}

pub struct Context {
    pub terms: Vec<Term>,
}

impl Context {
    pub fn avoid_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        for t in &self.terms {
            for w in &t.avoid {
                pairs.push((w.clone(), t.name.clone()));
            }
        }
        pairs
    }
}

pub fn parse_context(content: &str) -> Context {
    let mut terms: Vec<Term> = Vec::new();
    let mut current: Option<Term> = None;
    let mut in_language = false;
    let term_re = Regex::new(r"^\*\*(.+?)\*\*\s*[:：]").unwrap();
    let avoid_re = Regex::new(r"^_?Avoid_?\s*[:：]\s*(.*)$").unwrap();

    for line in content.lines() {
        if in_language {
            if let Some(caps) = term_re.captures(line) {
                if let Some(t) = current.take() {
                    terms.push(t);
                }
                current = Some(Term {
                    name: caps[1].trim().to_string(),
                    avoid: Vec::new(),
                });
            } else if let Some(caps) = avoid_re.captures(line)
                && let Some(t) = current.as_mut()
            {
                for w in caps[1]
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                {
                    t.avoid.push(w.to_string());
                }
            }
        } else if line.trim() == "## Language" || line.trim() == "## language" {
            in_language = true;
        }
    }
    if let Some(t) = current.take() {
        terms.push(t);
    }
    Context { terms }
}

#[cfg(test)]
mod tests {
    use super::parse_context;

    #[test]
    fn parses_terms_and_avoid_words() {
        let content = "# docent\n\n## Language\n\n**Alpha**:\nOne piece of lint logic.\n_Avoid_: Bravo, Charlie\n\n**Delta**:\nAn instance of non-compliance.\n_Avoid_: Echo, Foxtrot\n";
        let ctx = parse_context(content);
        assert_eq!(ctx.terms.len(), 2);
        assert_eq!(ctx.terms[0].name, "Alpha");
        assert_eq!(ctx.terms[0].avoid, vec!["Bravo", "Charlie"]);
        assert_eq!(ctx.terms[1].avoid, vec!["Echo", "Foxtrot"]);
    }

    #[test]
    fn parses_fullwidth_colon() {
        let content = "## Language\n\n**Alpha**：\nThe request.\n_Avoid_：Bravo\n";
        let ctx = parse_context(content);
        assert_eq!(ctx.terms.len(), 1);
        assert_eq!(ctx.terms[0].name, "Alpha");
        assert_eq!(ctx.terms[0].avoid, vec!["Bravo"]);
    }
}
