use inquire::{
    Select,
    ui::{Color, RenderConfig, StyleSheet, Styled},
};
use std::fmt::{self, Display};

pub fn select_directory(matches: Vec<String>, list_size: usize) -> Option<String> {
    let searchable_paths: Vec<String> = matches.iter().map(|path| path.to_lowercase()).collect();
    let mut options: Vec<SelectEntry> = matches
        .into_iter()
        .map(|path| SelectEntry::Directory {
            searchable: path.to_lowercase(),
            path,
        })
        .collect();
    options.push(SelectEntry::NoResults);

    let render_config = RenderConfig::default()
        .with_highlighted_option_prefix(Styled::new(">").with_fg(Color::Black).with_bg(Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(Color::Black).with_bg(Color::Grey),
        ));

    let scorer = |input: &str, option: &SelectEntry, _display: &str, index: usize| {
        literal_score(input, option, index, &searchable_paths)
    };

    Select::new("Select target directory:", options)
        .with_page_size(list_size)
        .with_render_config(render_config)
        .with_scorer(&scorer)
        .prompt()
        .ok()
        .and_then(|selection| match selection {
            SelectEntry::Directory { path, .. } => Some(path),
            SelectEntry::NoResults => None,
        })
}

#[derive(Debug)]
enum SelectEntry {
    Directory { path: String, searchable: String },
    NoResults,
}

impl Display for SelectEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Directory { path, .. } => path.fmt(formatter),
            Self::NoResults => "No path found matching the filter.".fmt(formatter),
        }
    }
}

fn literal_score(
    input: &str,
    option: &SelectEntry,
    index: usize,
    searchable_paths: &[String],
) -> Option<i64> {
    let filter = input.to_lowercase();

    match option {
        SelectEntry::Directory { searchable, .. } => {
            searchable.contains(&filter).then_some(-(index as i64))
        }
        SelectEntry::NoResults => (!filter.is_empty()
            && !searchable_paths.iter().any(|path| path.contains(&filter)))
        .then_some(i64::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textual_filter_uses_literal_case_insensitive_matching() {
        let paths = vec![
            "/home/user/docs".to_string(),
            "/home/user/linux".to_string(),
        ];
        let option = SelectEntry::Directory {
            path: paths[0].clone(),
            searchable: paths[0].to_lowercase(),
        };

        assert!(literal_score("DOCS", &option, 0, &paths).is_some());
        assert!(literal_score("docss", &option, 0, &paths).is_none());
    }

    #[test]
    fn no_results_message_only_appears_for_an_impossible_nonempty_filter() {
        let paths = vec![
            "/home/user/docs".to_string(),
            "/home/user/linux".to_string(),
        ];

        assert!(literal_score("docss", &SelectEntry::NoResults, 2, &paths).is_some());
        assert!(literal_score("docs", &SelectEntry::NoResults, 2, &paths).is_none());
        assert!(literal_score("", &SelectEntry::NoResults, 2, &paths).is_none());
    }
}
