use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LcConfig {
    pub env: String,
}

impl Default for LcConfig {
    fn default() -> Self {
        Self { env: "dev".into() }
    }
}
