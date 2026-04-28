use serde::{Deserialize, Serialize};

// / Keys will be absolute path, defaulting to the config directory
// /
// /
#[derive(Debug, Serialize, Deserialize)]
pub struct Libp2pConfig {
    is_server: bool,
    key_location: String,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self {
            is_server: true,
            key_location: "~/.config/lc".into(),
        }
    }
}
