use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub dxvk: Option<String>,
    pub vkd3d: Option<String>,
}

impl Default for Components {
    fn default() -> Self {
        // TODO: Check if file is already available for dxvk and vkd3d
        // - If available, we fill the information for the default
        // - Otherwise, we download the missing dependencies.

        Self {
            dxvk: Default::default(),
            vkd3d: Default::default(),
        }
    }
}
