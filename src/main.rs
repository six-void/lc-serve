pub(crate) mod config;
mod key;

use std::{time::Duration, u64};

use crate::config::LcConfig;

use multiaddr::multiaddr;

use futures::StreamExt;
use libp2p::{multiaddr, noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr};
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

    let location_addr = multiaddr!(Ip4([127, 0, 0, 1]), Tcp(10500u16));

    dbg!(&location_addr);

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on(location_addr)?;

    if let Some(addr_remote) = std::env::args().nth(1) {
        let remote: Multiaddr = addr_remote.parse()?;
        let _ = swarm.dial(remote);
        println!("dialed {addr_remote}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }

    // Ok(())
}
