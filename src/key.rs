use std::path::PathBuf;

use libp2p::identity::Keypair;

pub fn get_key(key_path: &PathBuf) -> Keypair {
    match std::fs::read(key_path) {
        Ok(k) => Keypair::from_protobuf_encoding(&k).expect("failed to load key from file?"),
        Err(_) => {
            let key = Keypair::generate_ed25519();
            let bytes = key.to_protobuf_encoding().expect("encode keypair");
            std::fs::write(key_path, bytes).expect("failed to write keypair");
            key
        }
    }
}
