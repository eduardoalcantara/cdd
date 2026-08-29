use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseSensitivity {
    Sensitive,
    Insensitive,
}

impl Default for CaseSensitivity {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        {
            CaseSensitivity::Insensitive
        }
        #[cfg(not(target_os = "windows"))]
        {
            CaseSensitivity::Sensitive
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListOrder {
    Ascending,
    Descending,
    #[default]
    Find,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryOrder {
    #[default]
    Sequential,
    Inverse,
    Any,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StickyState {
    pub lucky_pick: bool,
    pub list_size: bool,
    pub list_order: bool,
    pub query_order: bool,
    pub case_sensitivity: bool,
    pub use_index: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub lucky_pick: bool,
    pub list_size: u32,
    pub list_order: ListOrder,
    pub query_order: QueryOrder,
    pub case_sensitivity: CaseSensitivity,
    pub use_index: bool,
    pub sticky: StickyState,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lucky_pick: false,
            list_size: 10,
            list_order: ListOrder::default(),
            query_order: QueryOrder::default(),
            case_sensitivity: CaseSensitivity::default(),
            use_index: true,
            sticky: StickyState::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| {
            dirs::home_dir().expect("Could not find user's Home directory.")
        });
        path.push("cdd");
        path.push("cdd.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists()
            && let Ok(content) = std::fs::read_to_string(path)
            && let Ok(mut config) = serde_json::from_str::<Config>(&content)
        {
            let has_sticky_metadata = serde_json::from_str::<serde_json::Value>(&content)
                .ok()
                .and_then(|value| {
                    value
                        .as_object()
                        .map(|object| object.contains_key("sticky"))
                })
                .unwrap_or(false);
            if !has_sticky_metadata {
                config.infer_legacy_sticky_state();
            }
            return config;
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn active_filter_labels(&self) -> Vec<String> {
        let mut labels = Vec::new();

        if self.sticky.lucky_pick {
            labels.push("-1=on".to_string());
        }
        if self.sticky.list_size {
            labels.push(format!("-{}=on", self.list_size));
        }
        if self.sticky.list_order {
            let flag = match self.list_order {
                ListOrder::Ascending => "-oa",
                ListOrder::Descending => "-od",
                ListOrder::Find => "-of",
            };
            labels.push(format!("{flag}=on"));
        }
        if self.sticky.query_order {
            let flag = match self.query_order {
                QueryOrder::Sequential => "-qs",
                QueryOrder::Inverse => "-qi",
                QueryOrder::Any => "-qa",
            };
            labels.push(format!("{flag}=on"));
        }
        if self.sticky.case_sensitivity {
            let flag = match self.case_sensitivity {
                CaseSensitivity::Insensitive => "-ci",
                CaseSensitivity::Sensitive => "-cr",
            };
            labels.push(format!("{flag}=on"));
        }
        if self.sticky.use_index {
            labels.push("-ix=on".to_string());
        }

        labels
    }

    fn infer_legacy_sticky_state(&mut self) {
        self.sticky.lucky_pick = self.lucky_pick;
        self.sticky.list_size = self.list_size != Config::default().list_size;
        self.sticky.list_order = self.list_order != ListOrder::default();
        self.sticky.query_order = self.query_order != QueryOrder::default();
        self.sticky.case_sensitivity = self.case_sensitivity != CaseSensitivity::default();
        self.sticky.use_index = !self.use_index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_config_values_can_be_migrated_to_sticky_metadata() {
        let json = r#"{
            "lucky_pick": true,
            "list_size": 15,
            "list_order": "Ascending",
            "query_order": "Any",
            "case_sensitivity": "Insensitive"
        }"#;

        let mut config: Config = serde_json::from_str(json).unwrap();
        config.infer_legacy_sticky_state();

        assert!(config.lucky_pick);
        assert_eq!(config.list_size, 15);
        
        let expected_ci = if CaseSensitivity::default() == CaseSensitivity::Sensitive {
            vec!["-1=on", "-15=on", "-oa=on", "-qa=on", "-ci=on"]
        } else {
            vec!["-1=on", "-15=on", "-oa=on", "-qa=on"]
        };
        
        assert_eq!(config.active_filter_labels(), expected_ci);
    }

    #[test]
    fn active_filter_labels_include_explicit_default_values() {
        let config = Config {
            list_size: 10,
            list_order: ListOrder::Find,
            query_order: QueryOrder::Sequential,
            case_sensitivity: CaseSensitivity::default(),
            sticky: StickyState {
                list_size: true,
                list_order: true,
                query_order: true,
                case_sensitivity: true,
                ..StickyState::default()
            },
            ..Config::default()
        };

        let default_case_flag = match CaseSensitivity::default() {
            CaseSensitivity::Insensitive => "-ci=on",
            CaseSensitivity::Sensitive => "-cr=on",
        };

        assert_eq!(
            config.active_filter_labels(),
            vec!["-10=on", "-of=on", "-qs=on", default_case_flag]
        );
    }
}
