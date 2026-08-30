use crate::pattern::QueryPattern;
use crate::search::{path_matches, path_to_components};
use crate::config::QueryOrder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const INDEX_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct RootIndex {
    updated_at: u64,
    paths: Vec<String>,
}

impl Default for RootIndex {
    fn default() -> Self {
        Self {
            updated_at: 0,
            paths: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DirectoryIndex {
    version: u32,
    roots: std::collections::BTreeMap<String, RootIndex>,
}

impl Default for DirectoryIndex {
    fn default() -> Self {
        Self {
            version: INDEX_VERSION,
            roots: std::collections::BTreeMap::new(),
        }
    }
}

impl DirectoryIndex {
    pub fn index_path() -> PathBuf {
        let mut path = crate::config::Config::config_path();
        path.pop();
        path.push("index.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::index_path();
        if path.exists()
            && let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(index) = serde_json::from_str::<DirectoryIndex>(&content)
        {
            return index;
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::index_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("could not create index directory: {error}"))?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|error| format!("could not serialize index: {error}"))?;
        std::fs::write(path, content).map_err(|error| format!("could not write index: {error}"))
    }

    pub fn root_key(root: &Path) -> String {
        let mut key = root.to_string_lossy().to_string();
        #[cfg(target_os = "windows")]
        {
            key = key.replace('/', "\\");
            if key.len() == 2 && key.as_bytes().get(1) == Some(&b':') {
                key.push('\\');
            }
        }
        key
    }

    pub fn clear_root(&mut self, root: &Path) {
        self.roots.remove(&Self::root_key(root));
    }

    pub fn merge_paths(&mut self, root: &Path, new_paths: impl IntoIterator<Item = String>) {
        let key = Self::root_key(root);
        let entry = self.roots.entry(key).or_default();
        let mut known: HashSet<String> = entry.paths.iter().cloned().collect();

        for path in new_paths {
            if known.insert(path.clone()) {
                entry.paths.push(path);
            }
        }

        entry.updated_at = now_unix();
        self.version = INDEX_VERSION;
    }

    pub fn prune_missing(&mut self, root: &Path) -> usize {
        let key = Self::root_key(root);
        let Some(entry) = self.roots.get_mut(&key) else {
            return 0;
        };

        let before = entry.paths.len();
        entry.paths.retain(|path| Path::new(path).is_dir());
        entry.updated_at = now_unix();
        before.saturating_sub(entry.paths.len())
    }

    pub fn search(
        &self,
        root: &Path,
        patterns: &[QueryPattern],
        query_order: QueryOrder,
        lucky_pick: bool,
    ) -> Vec<String> {
        let key = Self::root_key(root);
        let Some(entry) = self.roots.get(&key) else {
            return Vec::new();
        };

        let mut matches = Vec::new();

        for path in &entry.paths {
            let components = path_to_components(Path::new(path), root);
            if path_matches(&components, patterns, query_order) {
                matches.push(path.clone());
                if lucky_pick {
                    break;
                }
            }
        }

        matches.retain(|path| Path::new(path).is_dir());
        matches
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::compile_all;
    use crate::config::CaseSensitivity;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn merge_paths_deduplicates_entries() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let mut index = DirectoryIndex::default();

        index.merge_paths(root, vec!["a".to_string(), "b".to_string()]);
        index.merge_paths(root, vec!["b".to_string(), "c".to_string()]);

        let entry = index.roots.get(&DirectoryIndex::root_key(root)).unwrap();
        assert_eq!(entry.paths, vec!["a", "b", "c"]);
    }

    #[test]
    fn search_uses_cached_paths_before_disk() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("projects/app1")).unwrap();

        let mut index = DirectoryIndex::default();
        let target = root.join("projects/app1");
        index.merge_paths(root, vec![target.to_string_lossy().into_owned()]);

        let patterns = compile_all(&["app".to_string()], CaseSensitivity::Insensitive).unwrap();
        let matches = index.search(root, &patterns, QueryOrder::Sequential, false);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].ends_with("app1"));
    }

    #[test]
    fn search_skips_stale_paths_without_touching_every_entry_on_disk() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("projects/app1")).unwrap();

        let mut index = DirectoryIndex::default();
        let live = root.join("projects/app1");
        index.merge_paths(
            root,
            vec![
                live.to_string_lossy().into_owned(),
                root.join("ghost/removed")
                    .to_string_lossy()
                    .into_owned(),
            ],
        );

        let patterns = compile_all(&["app".to_string()], CaseSensitivity::Insensitive).unwrap();
        let matches = index.search(root, &patterns, QueryOrder::Sequential, false);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].ends_with("app1"));
    }
}
