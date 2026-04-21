use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Libp2pConfig {
    is_server: bool,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self { is_server: true }
    }
}
