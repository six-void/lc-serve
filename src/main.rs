pub(crate) mod config;
mod key;

use std::{
    env,
    io::{self, BufRead},
    path::PathBuf,
    time::Duration,
    u64,
};

use crate::config::LcConfig;

use multiaddr::multiaddr;

use futures::StreamExt;
use libp2p::{
    kad::{self, store::MemoryStore, Mode},
    multiaddr, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr,
};
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
    let key = "LC_CONFIG_DIR";

    let cfg: LcConfig;

    match env::var(key) {
        Ok(path) => {
            let mut file: PathBuf = PathBuf::new();
            file.push(path);
            file.push("lc.toml");
            dbg!("custon location", &file);
            cfg = confy::load_path(file)?;
        }
        Err(_) => {
            cfg = confy::load("lc", Some("lc"))?;
        }
    }

    // let cfg: LcConfig = confy::load("lc", Some("lc"))?;
    // dbg!(&cfg);
    info!("Started with these settings {:#?}", &cfg);

    let key_path = &cfg.libp2p.key_location;

    dbg!(key_path);

    let keypair = key::get_key(key_path);

    let location_addr = multiaddr!(Ip4([127, 0, 0, 1]), Tcp(10500u16));

    dbg!(&location_addr);

    #[derive(NetworkBehaviour)]
    struct Behaviour {
        kademlia: kad::Behaviour<MemoryStore>,
        ping: ping::Behaviour,
    }

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            Ok(Behaviour {
                kademlia: kad::Behaviour::new(
                    key.public().to_peer_id(),
                    MemoryStore::new(key.public().to_peer_id()),
                ),
                ping: ping::Behaviour::default(),
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.behaviour_mut().kademlia.set_mode(Some(Mode::Server));

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    swarm.listen_on(location_addr)?;

    if let Some(addr_remote) = std::env::args().nth(1) {
        let remote: Multiaddr = addr_remote.parse()?;
        let _ = swarm.dial(remote);
        println!("dialed {addr_remote}")
    }

    loop {
        futures::select! {


        event = swarm.select_next_some() => match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening in {address:?}");
            },
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
                _ => {}
                }
            }

        // Ok(())
    }
}
