use std::env::var;

use serde::{Deserialize, Serialize};

const LAUNCHER_ALIAS: &str = "N1KK3";

/// Paths for the program/launcher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paths {
    /// $HOME folder (~/)
    pub home: String,

    pub cache: String,
    pub local: String,
    pub config: String,
}

impl Default for Paths {
    fn default() -> Self {
        let home_path: String = var("HOME")
            .expect("HOME environment variable not found, please check your environment variables to ensure it's properly set.");

        Self {
            home: home_path.clone(),

            cache: format!("{}/.cache/{}", home_path, LAUNCHER_ALIAS),
            local: format!("{}/.local/share/{}", home_path, LAUNCHER_ALIAS),
            config: format!("{}/.config/{}", home_path, LAUNCHER_ALIAS),
        }
    }
}

impl Paths {}
