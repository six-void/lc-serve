use serde::{Deserialize, Serialize};

use crate::config::libp2p_config::Libp2pConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LcConfig {
    pub env: String,
    pub libp2p: Libp2pConfig,
}

impl Default for LcConfig {
    fn default() -> Self {
        Self {
            env: "dev".into(),
            libp2p: Libp2pConfig::default(),
        }
    }
}
