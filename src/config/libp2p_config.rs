use std::{env::home_dir, path::PathBuf};

use serde::{Deserialize, Serialize};

// / Keys will be absolute path, defaulting to the config directory
// /
// /
#[derive(Debug, Serialize, Deserialize)]
pub struct Libp2pConfig {
    pub is_server: bool,
    pub key_location: PathBuf,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self {
            is_server: true,
            key_location: Self::find_path(),
        }
    }
}

impl Libp2pConfig {
    fn find_path() -> PathBuf {
        let mut path = home_dir().expect("Could not get home directory");
        path.push(".config");
        path.push("lc");
        path.push("id_ed25519");

        path
    }
}
