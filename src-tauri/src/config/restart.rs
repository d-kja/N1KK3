use serde::{Deserialize, Serialize};

const DEFAULT_RESTARTS: u32 = 15;
const DEFAULT_COOLDOWN: u64 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restart {
    pub patterns: Vec<String>,

    #[serde(default = "gen_default_cooldown")]
    pub cooldown_seconds: u64,

    #[serde(default = "gen_default_restarts")]
    pub max_restarts: u32,
}

impl Default for Restart {
    fn default() -> Self {
        // TODO: Idenfity the default patterns for the regex to match the pattern that requires
        // restart.

        Self {
            patterns: Vec::new(),

            max_restarts: DEFAULT_RESTARTS,
            cooldown_seconds: DEFAULT_COOLDOWN,
        }
    }
}

fn gen_default_restarts() -> u32 {
    DEFAULT_RESTARTS
}
fn gen_default_cooldown() -> u64 {
    DEFAULT_COOLDOWN
}
