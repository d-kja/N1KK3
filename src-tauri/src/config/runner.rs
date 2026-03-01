use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{config::paths::Paths, download::client::WINE_RUNNER_SOURCE};

pub const RUNNER_ARCHIVE_RAW: &str = "wine-cachyos-miniloader-fonts-10.0-1-x86_64.tar.xz";
pub const RUNNER_ARCHIVE_EXTRACTED: &str = "wine-cachyos-miniloader-10.0";

pub const RUNNER_NAME: &str = "wine-cachyos";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runner {
    #[serde(flatten)]
    pub runner_type: RunnerType,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            runner_type: Default::default(),
        }
    }
}

impl Runner {
    pub fn binary_path(&self) -> String {
        let paths = Paths::default();

        match self.runner_type {
            RunnerType::Default => format!("{}/runners/{}/bin/wine", paths.local, RUNNER_NAME),
        }
    }

    pub fn download_url(&self) -> String {
        match self.runner_type {
            RunnerType::Default => WINE_RUNNER_SOURCE.into(),
        }
    }

    pub fn is_installed(&self) -> bool {
        let path: PathBuf = self.binary_path().into();
        path.exists()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RunnerType {
    /// CachyOS wine miniloader, installed by default.
    Default,
}

impl Default for RunnerType {
    fn default() -> Self {
        Self::Default
    }
}
