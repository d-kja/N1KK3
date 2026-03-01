use std::{fmt::Display, fs, path::PathBuf};

use tokio::process::Child;

use crate::config::paths::Paths;

const WINE_PREFIX_ENV: &str = "WINEPREFIX";
const WINE_PREFIX_ARGS: [&str; 2] = ["wineboot", "--init"];

pub struct Wine {
    pub path: PathBuf,
}

impl Default for Wine {
    fn default() -> Self {
        let paths = Paths::default();

        Self {
            path: PathBuf::from(format!("{}/prefix", paths.local)),
        }
    }
}

type WineBinary = PathBuf;

#[derive(Debug, Clone)]
pub enum WineError {
    RootPathNotFound,

    UnableToCreatePrefixDirectory,
    UnableToInitializeWinePrefix,

    UnableToExecuteCommand,
}

impl Display for WineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WineError::*;

        match self {
            RootPathNotFound => {
                write!(f, "Root path for wine prefix not found")
            }
            UnableToCreatePrefixDirectory => {
                write!(f, "Unable to create wine prefix directory")
            }
            UnableToInitializeWinePrefix => {
                write!(f, "Unable to initialize wine prefix")
            }
            UnableToExecuteCommand => {
                write!(f, "Unable to execute command on wine prefix")
            }
        }
    }
}

pub struct Command {
    pub wine_binary: WineBinary,
    pub game_executable: PathBuf,
    pub args: Vec<String>,
}

impl Wine {
    pub fn exists(&self) -> bool {
        let root_path = &self.path;
        if !root_path.exists() {
            return false;
        }

        let drive_path = root_path.join("drive_c");
        drive_path.exists()
    }

    pub async fn create(&self, wine_binary: WineBinary) -> Result<(), WineError> {
        let root_path = &self.path;

        let result = fs::create_dir_all(root_path);
        if result.is_err() {
            return Err(WineError::UnableToCreatePrefixDirectory);
        }

        let std_result = tokio::process::Command::new(wine_binary)
            .args(WINE_PREFIX_ARGS)
            .env(WINE_PREFIX_ENV, root_path)
            .output()
            .await;

        if std_result.is_err() {
            return Err(WineError::UnableToInitializeWinePrefix);
        }

        let std_result = std_result.unwrap();
        if !std_result.status.success() {
            return Err(WineError::UnableToInitializeWinePrefix);
        }

        Ok(())
    }

    pub async fn run_command(&self, command: Command) -> Result<Child, WineError> {
        let root_path = &self.path;
        if !root_path.exists() {
            return Err(WineError::RootPathNotFound);
        }

        let std_result = tokio::process::Command::new(&command.wine_binary)
            .arg(&command.game_executable)
            .args(&command.args)
            .env(WINE_PREFIX_ENV, root_path)
            .spawn();

        if std_result.is_err() {
            return Err(WineError::UnableToExecuteCommand);
        }

        Ok(std_result.unwrap())
    }
}
