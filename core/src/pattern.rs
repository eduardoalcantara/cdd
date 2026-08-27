use regex::{Regex, RegexBuilder};

#[derive(Debug)]
pub enum QueryPattern {
    Literal(String),
    Wildcard(Regex),
}

impl QueryPattern {
    pub fn compile(raw: &str) -> Result<Self, String> {
        if raw.contains('*') || raw.contains('?') {
            let mut expression = String::from("^");

            for character in raw.chars() {
                match character {
                    '*' => expression.push_str(".*"),
                    '?' => expression.push('.'),
                    _ => expression.push_str(&regex::escape(&character.to_string())),
                }
            }

            expression.push('$');
            let regex = RegexBuilder::new(&expression)
                .case_insensitive(true)
                .build()
                .map_err(|error| format!("curinga inválido {raw:?}: {error}"))?;
            Ok(Self::Wildcard(regex))
        } else {
            Ok(Self::Literal(raw.to_lowercase()))
        }
    }

    pub fn matches_component(&self, component: &str) -> bool {
        match self {
            Self::Literal(literal) => component.to_lowercase().contains(literal),
            Self::Wildcard(regex) => regex.is_match(component),
        }
    }
}

pub fn compile_all(queries: &[String]) -> Result<Vec<QueryPattern>, String> {
    queries
        .iter()
        .map(|query| QueryPattern::compile(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_matching_is_partial_and_case_insensitive() {
        let pattern = QueryPattern::compile("PROJ").unwrap();
        assert!(pattern.matches_component("my-project"));
        assert!(!pattern.matches_component("workspace"));
    }

    #[test]
    fn star_matches_zero_or_more_characters() {
        let pattern = QueryPattern::compile("app*").unwrap();
        assert!(pattern.matches_component("app"));
        assert!(pattern.matches_component("application"));
        assert!(!pattern.matches_component("my-app"));
    }

    #[test]
    fn question_mark_matches_exactly_one_character() {
        let pattern = QueryPattern::compile("app?").unwrap();
        assert!(pattern.matches_component("app1"));
        assert!(!pattern.matches_component("app"));
        assert!(!pattern.matches_component("app10"));
    }

    #[test]
    fn regex_metacharacters_are_literal_without_glob_semantics() {
        let pattern = QueryPattern::compile("release.*").unwrap();
        assert!(pattern.matches_component("release.2026"));
        assert!(!pattern.matches_component("release-candidate"));
    }
}
