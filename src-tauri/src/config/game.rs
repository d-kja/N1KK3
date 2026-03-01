use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum State {
    Update,
    Ready,
    Setup,
}

/// Game path structure inside of the prefix:
///
/// - Game root: <prefix-path>/drive_c/NIKKE
/// - Game launcher: <prefix-path>/drive_c/NIKKE/Launcher/nikke_launcher.exe
/// - Game executable: <prefix-path>/drive_c/NIKKE/NIKKE/game/nikke.exe
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(flatten)]
    pub state: State,

    /// Path to original launcher executable
    pub launcher_path: String,

    /// Path to main game executable
    pub game_exe: String,

    /// Extra environment variables
    #[serde(default)]
    pub env_vars: HashMap<String, String>,

    /// Extra arguments to pass
    #[serde(default)]
    pub extra_args: Vec<String>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state: State::Setup,

            launcher_path: "/drive_c/NIKKE/Launcher/nikke_launcher.exe".into(),
            game_exe: "/drive_c/NIKKE/NIKKE/game/nikke.exe".into(),

            env_vars: Default::default(),
            extra_args: Default::default(),
        }
    }
}
