use std::path::PathBuf;
use jwalk::WalkDir;
use crate::args::AppArgs;
use crate::config::{ListOrder, QueryOrder};

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

pub fn find_directories(args: &AppArgs) -> Vec<String> {
    let mut queries = args.queries.clone();
    let root = get_root(&mut queries);

    if queries.is_empty() {
        return vec![root.to_string_lossy().into_owned()];
    }

    // Convert queries to lowercase for case-insensitive matching
    let queries_lower: Vec<String> = queries.iter().map(|s| s.to_lowercase()).collect();

    let mut matches = Vec::new();

    // Iterate over directories
    for entry in WalkDir::new(root).skip_hidden(true).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let path_str = path.to_string_lossy().to_lowercase();
        
        let is_match = match args.query_order {
            QueryOrder::Sequential => {
                let mut current_pos = 0;
                let mut found = true;
                for q in &queries_lower {
                    if let Some(pos) = path_str[current_pos..].find(q) {
                        current_pos += pos + q.len();
                    } else {
                        found = false;
                        break;
                    }
                }
                found
            }
            QueryOrder::Inverse => {
                let mut current_pos = 0;
                let mut found = true;
                for q in queries_lower.iter().rev() {
                    if let Some(pos) = path_str[current_pos..].find(q) {
                        current_pos += pos + q.len();
                    } else {
                        found = false;
                        break;
                    }
                }
                found
            }
            QueryOrder::Any => {
                queries_lower.iter().all(|q| path_str.contains(q))
            }
        };

        if is_match {
            matches.push(entry.path().to_string_lossy().into_owned());
        }
    }

    match args.list_order {
        ListOrder::Ascending => matches.sort(),
        ListOrder::Descending => matches.sort_by(|a, b| b.cmp(a)),
        ListOrder::Find => { /* Already in find order */ }
    }

    matches
}
