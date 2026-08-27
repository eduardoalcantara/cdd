use crate::args::AppArgs;
use crate::config::{ListOrder, QueryOrder};
use crate::pattern::{QueryPattern, compile_all};
use jwalk::WalkDir;
use std::path::{Component, Path, PathBuf};

#[cfg(target_os = "windows")]
fn get_root(queries: &mut Vec<String>) -> PathBuf {
    if !queries.is_empty() && queries[0].len() == 1 {
        let first_char = queries[0].chars().next().unwrap();
        if first_char.is_ascii_alphabetic() {
            let drive = format!("{}:\\", first_char.to_ascii_uppercase());
            queries.remove(0);
            return PathBuf::from(drive);
        }
    }

    // Se não, pegue a raiz do diretório atual
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(prefix) = current_dir.components().next() {
            return PathBuf::from(prefix.as_os_str());
        }
    }

    PathBuf::from("C:\\")
}

#[cfg(not(target_os = "windows"))]
fn get_root(_queries: &mut Vec<String>) -> PathBuf {
    PathBuf::from("/")
}

pub fn find_directories(args: &AppArgs) -> Result<Vec<String>, String> {
    let mut queries = args.queries.clone();
    let root = get_root(&mut queries);
    find_directories_from(&root, &queries, args)
}

fn find_directories_from(
    root: &Path,
    queries: &[String],
    args: &AppArgs,
) -> Result<Vec<String>, String> {
    if queries.is_empty() {
        return Ok(vec![root.to_string_lossy().into_owned()]);
    }

    let patterns = compile_all(queries)?;
    let mut matches = Vec::new();
    let exclusion_root = root.to_path_buf();

    let walker = WalkDir::new(root).skip_hidden(true).process_read_dir(
        move |_depth, _path, _state, children| {
            children.retain(|entry| {
                entry
                    .as_ref()
                    .map(|entry| !should_skip(&exclusion_root, &entry.path()))
                    .unwrap_or(true)
            });
        },
    );

    for entry in walker.into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let components: Vec<String> = relative
            .components()
            .filter_map(|component| match component {
                Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
                _ => None,
            })
            .collect();

        if path_matches(&components, &patterns, args.query_order) {
            matches.push(path.to_string_lossy().into_owned());
            if args.lucky_pick {
                break;
            }
        }
    }

    match args.list_order {
        ListOrder::Ascending => matches.sort(),
        ListOrder::Descending => matches.sort_by(|a, b| b.cmp(a)),
        ListOrder::Find => {}
    }

    Ok(matches)
}

fn path_matches(components: &[String], patterns: &[QueryPattern], query_order: QueryOrder) -> bool {
    let Some(destination) = components.last() else {
        return false;
    };

    match query_order {
        QueryOrder::Sequential => matches_in_order(components, patterns.iter()),
        QueryOrder::Inverse => matches_in_order(components, patterns.iter().rev()),
        QueryOrder::Any => {
            patterns.iter().all(|pattern| {
                components
                    .iter()
                    .any(|part| pattern.matches_component(part))
            }) && patterns
                .iter()
                .any(|pattern| pattern.matches_component(destination))
        }
    }
}

fn matches_in_order<'a>(
    components: &[String],
    patterns: impl Iterator<Item = &'a QueryPattern>,
) -> bool {
    let Some(destination) = components.last() else {
        return false;
    };
    let ordered_patterns: Vec<&QueryPattern> = patterns.collect();
    let Some((destination_pattern, ancestor_patterns)) = ordered_patterns.split_last() else {
        return false;
    };

    if !destination_pattern.matches_component(destination) {
        return false;
    }

    let mut next_component = 0;

    for pattern in ancestor_patterns {
        let Some(offset) = components[next_component..components.len() - 1]
            .iter()
            .position(|part| pattern.matches_component(part))
        else {
            return false;
        };
        next_component += offset + 1;
    }

    true
}

#[cfg(not(target_os = "windows"))]
fn should_skip(root: &Path, path: &Path) -> bool {
    if root != Path::new("/") {
        return false;
    }

    let Some(first_component) = path
        .strip_prefix(root)
        .ok()
        .and_then(|relative| relative.components().next())
    else {
        return false;
    };

    matches!(
        first_component.as_os_str().to_str(),
        Some("proc" | "sys" | "dev" | "run")
    )
}

#[cfg(target_os = "windows")]
fn should_skip(_root: &Path, _path: &Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ListOrder, QueryOrder};
    use std::fs;

    fn args(queries: &[&str], query_order: QueryOrder) -> AppArgs {
        AppArgs {
            queries: queries.iter().map(|value| (*value).to_string()).collect(),
            lucky_pick: false,
            list_size: 10,
            list_order: ListOrder::Ascending,
            query_order,
            config_changed: false,
            out_file: None,
            show_help: false,
            active_filters: Vec::new(),
        }
    }

    #[test]
    fn sequential_inverse_and_any_respect_directory_components() {
        let components = vec!["var".to_string(), "www".to_string(), "app1".to_string()];
        let patterns = compile_all(&["www".to_string(), "app".to_string()]).unwrap();

        assert!(path_matches(&components, &patterns, QueryOrder::Sequential));
        assert!(!path_matches(&components, &patterns, QueryOrder::Inverse));
        assert!(path_matches(&components, &patterns, QueryOrder::Any));
    }

    #[test]
    fn wildcard_search_uses_component_names() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join("projects/application1")).unwrap();
        fs::create_dir_all(temp.path().join("projects/application1/docs")).unwrap();
        fs::create_dir_all(temp.path().join("projects/application10")).unwrap();
        fs::create_dir_all(temp.path().join("other/my-application1")).unwrap();

        let app_args = args(&["proj*", "application?"], QueryOrder::Sequential);
        let results = find_directories_from(temp.path(), &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        let expected = std::path::PathBuf::from("projects").join("application1");
        assert!(results[0].ends_with(expected.to_str().unwrap()));
    }

    #[test]
    fn a_matching_ancestor_does_not_make_its_descendants_results() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join("linux-workstation/docs/project")).unwrap();

        let app_args = args(&["linux"], QueryOrder::Sequential);
        let results = find_directories_from(temp.path(), &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("linux-workstation"));
    }

    #[test]
    fn any_order_still_requires_the_destination_to_match_a_query() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join("linux-workstation/docs")).unwrap();

        let app_args = args(&["linux", "work"], QueryOrder::Any);
        let results = find_directories_from(temp.path(), &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("linux-workstation"));
    }

    #[test]
    fn literal_search_remains_partial_and_case_insensitive() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join("Workspace/MyProject")).unwrap();

        let app_args = args(&["work", "PROJECT"], QueryOrder::Sequential);
        let results = find_directories_from(temp.path(), &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        let expected = std::path::PathBuf::from("Workspace").join("MyProject");
        assert!(results[0].ends_with(expected.to_str().unwrap()));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn linux_virtual_filesystems_are_skipped_only_from_root_scan() {
        assert!(should_skip(Path::new("/"), Path::new("/proc/123")));
        assert!(should_skip(Path::new("/"), Path::new("/sys/class")));
        assert!(!should_skip(Path::new("/"), Path::new("/home/user")));
        assert!(!should_skip(
            Path::new("/tmp/fixture"),
            Path::new("/tmp/fixture/proc/example")
        ));
    }
}
