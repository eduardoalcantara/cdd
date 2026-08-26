use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListOrder {
    Ascending,
    Descending,
    Find,
}

impl Default for ListOrder {
    fn default() -> Self {
        ListOrder::Find
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryOrder {
    Sequential,
    Inverse,
    Any,
}

impl Default for QueryOrder {
    fn default() -> Self {
        QueryOrder::Sequential
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub lucky_pick: bool,
    pub list_size: u32,
    pub list_order: ListOrder,
    pub query_order: QueryOrder,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lucky_pick: false,
            list_size: 10,
            list_order: ListOrder::default(),
            query_order: QueryOrder::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| {
            dirs::home_dir().expect("Não foi possível encontrar a pasta Home do usuário.")
        });
        path.push("cdd");
        path.push("cdd.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
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
}
