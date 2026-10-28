use crate::args::AppArgs;
use crate::config::{ListOrder, QueryOrder};
use crate::index::DirectoryIndex;
use crate::pattern::{QueryPattern, compile_all};
use crate::wait_animation::WaitAnimation;
use jwalk::WalkDir;
use std::path::{Component, Path, PathBuf};

pub fn path_to_components(path: &Path, root: &Path) -> Vec<String> {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}

#[cfg(target_os = "windows")]
fn windows_drive_root_from(path: &Path) -> Option<PathBuf> {
    let text = path.to_string_lossy();
    if text.len() >= 2 {
        let bytes = text.as_bytes();
        if bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            let letter = bytes[0].to_ascii_uppercase() as char;
            return Some(PathBuf::from(format!("{letter}:\\")));
        }
    }
    None
}

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

    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(root) = windows_drive_root_from(&current_dir) {
            return root;
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
    let disk_root = get_root(&mut queries);
    let cwd = std::env::current_dir().ok();
    find_directories_prefer_local(cwd.as_deref(), &disk_root, &queries, args)
}

fn find_directories_prefer_local(
    cwd: Option<&Path>,
    disk_root: &Path,
    queries: &[String],
    args: &AppArgs,
) -> Result<Vec<String>, String> {
    if queries.is_empty() {
        return Ok(vec![disk_root.to_string_lossy().into_owned()]);
    }

    let patterns = compile_all(queries, args.case_sensitivity)?;

    if !args.rebuild_index {
        if let Some(cwd) = cwd {
            if is_strict_descendant(cwd, disk_root) {
                if let Some(local) = search_under_cwd(cwd, disk_root, &patterns, args)? {
                    return Ok(sort_matches(local, args.list_order));
                }
            }
        }
    }

    search_from_root(disk_root, disk_root, &patterns, args, true)
}

#[cfg(test)]
fn find_directories_from(
    root: &Path,
    queries: &[String],
    args: &AppArgs,
) -> Result<Vec<String>, String> {
    if queries.is_empty() {
        return Ok(vec![root.to_string_lossy().into_owned()]);
    }

    let patterns = compile_all(queries, args.case_sensitivity)?;
    search_from_root(root, root, &patterns, args, true)
}

fn search_under_cwd(
    cwd: &Path,
    disk_root: &Path,
    patterns: &[QueryPattern],
    args: &AppArgs,
) -> Result<Option<Vec<String>>, String> {
    if args.use_index {
        let index = DirectoryIndex::load();
        let cached = index.search_under(disk_root, cwd, patterns, args.query_order, args.lucky_pick);
        if !cached.is_empty() {
            return Ok(Some(cached));
        }
    }

    let (matches, discovered) = scan_disk(cwd, disk_root, patterns, args)?;
    if matches.is_empty() {
        return Ok(None);
    }

    if args.use_index {
        let mut index = DirectoryIndex::load();
        index.merge_paths(disk_root, discovered);
        index.save()?;
    }

    Ok(Some(matches))
}

fn search_from_root(
    walk_root: &Path,
    match_root: &Path,
    patterns: &[QueryPattern],
    args: &AppArgs,
    animate: bool,
) -> Result<Vec<String>, String> {
    let _wait_animation = if animate {
        Some(WaitAnimation::start())
    } else {
        None
    };

    if args.use_index && !args.rebuild_index {
        let index = DirectoryIndex::load();
        let cached = index.search(match_root, patterns, args.query_order, args.lucky_pick);
        if !cached.is_empty() {
            return Ok(sort_matches(cached, args.list_order));
        }
    }

    let (matches, discovered) = scan_disk(walk_root, match_root, patterns, args)?;

    if args.use_index || args.rebuild_index {
        let mut index = DirectoryIndex::load();
        if args.rebuild_index {
            index.clear_root(match_root);
        }
        index.merge_paths(match_root, discovered);
        if args.rebuild_index {
            index.prune_missing(match_root);
        }
        index.save()?;
    }

    Ok(sort_matches(matches, args.list_order))
}

fn scan_disk(
    walk_root: &Path,
    match_root: &Path,
    patterns: &[QueryPattern],
    args: &AppArgs,
) -> Result<(Vec<String>, Vec<String>), String> {
    let mut matches = Vec::new();
    let mut discovered = Vec::new();
    let exclusion_root = walk_root.to_path_buf();

    let walker = WalkDir::new(walk_root).skip_hidden(true).process_read_dir(
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
        let path_text = path.to_string_lossy().into_owned();
        discovered.push(path_text.clone());

        let components = path_to_components(&path, match_root);
        if path_matches(&components, patterns, args.query_order) {
            matches.push(path_text);
            if args.lucky_pick {
                break;
            }
        }
    }

    Ok((matches, discovered))
}

pub fn is_strict_descendant(path: &Path, ancestor: &Path) -> bool {
    let path_key = normalize_path_key(path);
    let ancestor_key = normalize_path_key(ancestor);
    if path_key == ancestor_key {
        return false;
    }
    Path::new(&path_key)
        .strip_prefix(Path::new(&ancestor_key))
        .is_ok()
}

fn normalize_path_key(path: &Path) -> String {
    let mut text = path.to_string_lossy().replace('/', std::path::MAIN_SEPARATOR_STR);

    #[cfg(windows)]
    {
        text = text.replace('/', "\\");
        if text.len() >= 2 && text.as_bytes()[1] == b':' {
            let mut chars: Vec<char> = text.chars().collect();
            chars[0] = chars[0].to_ascii_uppercase();
            text = chars.into_iter().collect();
        }
        if text.len() > 3 && text.ends_with('\\') {
            text.pop();
        }
        text = text.to_lowercase();
    }

    #[cfg(not(windows))]
    {
        if text.len() > 1 && text.ends_with('/') {
            text.pop();
        }
    }

    text
}

fn sort_matches(mut matches: Vec<String>, list_order: ListOrder) -> Vec<String> {
    match list_order {
        ListOrder::Ascending => matches.sort(),
        ListOrder::Descending => matches.sort_by(|a, b| b.cmp(a)),
        ListOrder::Find => {}
    }
    matches
}

pub fn path_matches(
    components: &[String],
    patterns: &[QueryPattern],
    query_order: QueryOrder,
) -> bool {
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
            case_sensitivity: crate::config::CaseSensitivity::Insensitive,
            use_index: false,
            rebuild_index: false,
            config_changed: false,
            out_file: None,
            show_help: false,
            active_filters: Vec::new(),
        }
    }

    #[test]
    fn sequential_inverse_and_any_respect_directory_components() {
        let components = vec!["var".to_string(), "www".to_string(), "app1".to_string()];
        let patterns = compile_all(
            &["www".to_string(), "app".to_string()],
            crate::config::CaseSensitivity::Insensitive,
        )
        .unwrap();

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

    #[test]
    fn prefers_matches_under_cwd_over_elsewhere_on_disk() {
        let temp = tempfile::tempdir().unwrap();
        let disk = temp.path();
        let cwd = disk.join("here");
        fs::create_dir_all(cwd.join("target-dir")).unwrap();
        fs::create_dir_all(disk.join("elsewhere/target-dir")).unwrap();

        let app_args = args(&["target-dir"], QueryOrder::Sequential);
        let results =
            find_directories_prefer_local(Some(&cwd), disk, &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        let expected = cwd.join("target-dir");
        assert!(results[0].ends_with(expected.file_name().unwrap().to_str().unwrap()));
        assert!(results[0].contains("here"));
        assert!(!results[0].contains("elsewhere"));
    }

    #[test]
    fn falls_back_to_disk_when_cwd_has_no_match() {
        let temp = tempfile::tempdir().unwrap();
        let disk = temp.path();
        let cwd = disk.join("here");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(disk.join("elsewhere/target-dir")).unwrap();

        let app_args = args(&["target-dir"], QueryOrder::Sequential);
        let results =
            find_directories_prefer_local(Some(&cwd), disk, &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].contains("elsewhere"));
        assert!(results[0].ends_with("target-dir"));
    }

    #[test]
    fn searches_whole_root_when_cwd_is_the_disk_root() {
        let temp = tempfile::tempdir().unwrap();
        let disk = temp.path();
        fs::create_dir_all(disk.join("a/target-dir")).unwrap();
        fs::create_dir_all(disk.join("b/target-dir")).unwrap();

        let app_args = args(&["target-dir"], QueryOrder::Sequential);
        let results =
            find_directories_prefer_local(Some(disk), disk, &app_args.queries, &app_args).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn strict_descendant_rejects_the_path_itself() {
        let path = PathBuf::from("/home/user");
        assert!(!is_strict_descendant(&path, &path));
        assert!(is_strict_descendant(
            &path.join("project"),
            &path
        ));
        assert!(!is_strict_descendant(
            Path::new("/var/log"),
            Path::new("/home")
        ));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_drive_root_uses_backslash_root_not_current_directory() {
        let deep = PathBuf::from(r"G:\projects\linux\scripts");
        assert_eq!(
            windows_drive_root_from(&deep),
            Some(PathBuf::from(r"G:\"))
        );
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
