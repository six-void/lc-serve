pub(crate) mod config;
mod key;

use crate::config::LcConfig;

use tracing::info;
//use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_ansi(true)
        .with_target(true)
        // .with_env_filter(EnvFilter::from_default_env())
        // .with_thread_ids(true)
        .try_init();

    info!("Started lc");

    let cfg: LcConfig = confy::load("lc", Some("lc"))?;
    // dbg!(&cfg);
    info!("Started with these settings {:#?}", &cfg);

    let key_path = &cfg.libp2p.key_location;

    dbg!(key_path);

    let keypair = key::get_key(key_path);

    dbg!(&keypair);

    Ok(())
}
