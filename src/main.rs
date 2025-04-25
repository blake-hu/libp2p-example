use clap::Parser;
use libp2p::Multiaddr;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'c', long = "config-path")]
    config_path: std::path::PathBuf,

    #[arg(short = 'p', long = "peer-idx")]
    peer_index: u64,
}

#[derive(Deserialize)]
struct TomlConfig {
    peers: Vec<String>,
}

#[derive(Debug)]
struct NetworkConfig {
    our_index: u64,
    addresses: Vec<Multiaddr>,
}

impl Display for NetworkConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let addr = &self.addresses;
        let our_index = self.our_index;
        let our_addr = &self.addresses[self.our_index as usize];
        write!(
            f,
            "All addresses: {addr:?} | We are peer {our_index}: {our_addr}"
        )
    }
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    // try to read config file into a String
    let config_str = std::fs::read_to_string(args.config_path.as_path()).unwrap_or_else(|_| {
        panic!(
            "Failed to read config file {}",
            args.config_path.to_str().unwrap()
        )
    });

    // try to parse string as TOML
    let toml_config =
        toml::from_str::<TomlConfig>(config_str.as_str()).expect("Could not parse config string");

    let address_vec: Vec<_> = toml_config
        .peers
        .iter()
        .map(|s| Multiaddr::from_str(s).expect("Could not parse address string as Multiaddr"))
        .collect();

    if args.peer_index >= address_vec.iter().count() as u64 {
        panic!("Peer index greater than number of peer addresses available");
    }

    let network_config = NetworkConfig {
        our_index: args.peer_index,
        addresses: address_vec,
    };

    println!("{network_config}");
}
