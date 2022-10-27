//! Decentralized Chat Client with libp2p
use libp2p::identity;
use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

pub async fn run() -> Result<()> {
    let key = identity::Keypair::generate_ed25519();
    println!("Public Key: {:?}", key.public());
    let peer_id = libp2p::PeerId::from(key.public());
    println!("PeerId: {peer_id}");
    let transport = libp2p::development_transport(key).await?;
    println!("Transport: {transport:?}");
    Ok(())
}
