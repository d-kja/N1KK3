pub mod components;
pub mod game;
pub mod paths;
pub mod restart;
pub mod runner;

use core::fmt;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::components::Components;
use crate::config::game::Game;
use crate::config::paths::Paths;
use crate::config::restart::Restart;
use crate::config::runner::Runner;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    /// Paths configuration - root of the whole project
    pub paths: Paths,

    /// Launcher/Game settings
    pub game: Game,

    /// Wine/Proton runner configuration
    pub runner: Runner,

    /// Restart behavior
    pub restart: Restart,

    /// Component versions
    pub components: Components,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::default(),
            paths: Paths::default(),
            runner: Runner::default(),
            restart: Restart::default(),
            components: Components::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppError {
    UnableToSaveConfigurationErr,
    UnableToLoadFromPathErr,
    UnableToParseConfigErr,

    UnableToCreateConfigDirectoryErr,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::UnableToSaveConfigurationErr => write!(f, "Unable to save app configuration"),
            AppError::UnableToLoadFromPathErr => write!(f, "Unable to load app configuration"),
            AppError::UnableToParseConfigErr => write!(f, "Unable to parse app configuration"),
            AppError::UnableToCreateConfigDirectoryErr => write!(
                f,
                "Unable to create configuration direction for the launcher"
            ),
        }
    }
}

fn get_config_path() -> PathBuf {
    let paths = Paths::default();

    PathBuf::from(format!("{}/config.json", &paths.config))
}

impl App {
    pub fn load() -> Result<Self, AppError> {
        let configuration_path = get_config_path();

        if configuration_path.exists() {
            let configuration_str = fs::read_to_string(configuration_path);
            if configuration_str.is_err() {
                return Err(AppError::UnableToLoadFromPathErr);
            }

            let configuration_str = configuration_str.unwrap();
            let configuration = serde_json::from_str::<App>(&configuration_str);

            if configuration.is_err() {
                return Err(AppError::UnableToParseConfigErr);
            }

            return Ok(configuration.unwrap());
        }

        Ok(Self::default())
    }

    pub fn save(&self) -> Result<(), AppError> {
        let configuration_path = get_config_path();
        let parent_directory = configuration_path.parent()
            .expect("Unable to retrieve parent directory from the configuration path, shouldn't happen at all.");

        // Setup parent directory to avoid issues when saving the configuration
        let result = fs::create_dir_all(parent_directory);
        if let Err(_) = result {
            return Err(AppError::UnableToCreateConfigDirectoryErr);
        }

        let configuration_json = serde_json::to_string_pretty(self);
        if configuration_json.is_err() {
            return Err(AppError::UnableToParseConfigErr);
        }

        let configuration_json = configuration_json.unwrap();

        match fs::write(configuration_path, configuration_json) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(AppError::UnableToSaveConfigurationErr),
        }
    }
}
