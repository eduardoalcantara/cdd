use crate::config::CaseSensitivity;
use regex::{Regex, RegexBuilder};

#[derive(Debug)]
pub enum QueryPattern {
    Literal { text: String, case_sensitivity: CaseSensitivity },
    Wildcard(Regex),
}

impl QueryPattern {
    pub fn compile(raw: &str, case_sensitivity: CaseSensitivity) -> Result<Self, String> {
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
            let is_insensitive = case_sensitivity == CaseSensitivity::Insensitive;
            let regex = RegexBuilder::new(&expression)
                .case_insensitive(is_insensitive)
                .build()
                .map_err(|error| format!("invalid wildcard {raw:?}: {error}"))?;
            Ok(Self::Wildcard(regex))
        } else {
            Ok(Self::Literal { 
                text: match case_sensitivity {
                    CaseSensitivity::Insensitive => raw.to_lowercase(),
                    CaseSensitivity::Sensitive => raw.to_string(),
                },
                case_sensitivity
            })
        }
    }

    pub fn matches_component(&self, component: &str) -> bool {
        match self {
            Self::Literal { text, case_sensitivity } => match case_sensitivity {
                CaseSensitivity::Insensitive => component.to_lowercase().contains(text),
                CaseSensitivity::Sensitive => component.contains(text),
            },
            Self::Wildcard(regex) => regex.is_match(component),
        }
    }
}

pub fn compile_all(queries: &[String], case_sensitivity: CaseSensitivity) -> Result<Vec<QueryPattern>, String> {
    queries
        .iter()
        .map(|query| QueryPattern::compile(query, case_sensitivity))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_matching_is_partial_and_case_insensitive() {
        let pattern = QueryPattern::compile("PROJ", CaseSensitivity::Insensitive).unwrap();
        assert!(pattern.matches_component("my-project"));
        assert!(!pattern.matches_component("workspace"));
    }

    #[test]
    fn literal_matching_can_be_case_sensitive() {
        let pattern = QueryPattern::compile("PROJ", CaseSensitivity::Sensitive).unwrap();
        assert!(!pattern.matches_component("my-project"));
        assert!(pattern.matches_component("MY-PROJECT"));
    }

    #[test]
    fn star_matches_zero_or_more_characters() {
        let pattern = QueryPattern::compile("app*", CaseSensitivity::Insensitive).unwrap();
        assert!(pattern.matches_component("app"));
        assert!(pattern.matches_component("application"));
        assert!(!pattern.matches_component("my-app"));
    }

    #[test]
    fn question_mark_matches_exactly_one_character() {
        let pattern = QueryPattern::compile("app?", CaseSensitivity::Insensitive).unwrap();
        assert!(pattern.matches_component("app1"));
        assert!(!pattern.matches_component("app"));
        assert!(!pattern.matches_component("app10"));
    }

    #[test]
    fn regex_metacharacters_are_literal_without_glob_semantics() {
        let pattern = QueryPattern::compile("release.*", CaseSensitivity::Insensitive).unwrap();
        assert!(pattern.matches_component("release.2026"));
        assert!(!pattern.matches_component("release-candidate"));
    }
}
