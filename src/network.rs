use crate::video::VideoMetadata;
use futures::StreamExt;
use libp2p::{Multiaddr, PeerId, StreamProtocol, Swarm, identity, kad, noise, request_response, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Transport};
use serde::{Deserialize, Serialize};

const KNAPSACK_PROTOCOL: StreamProtocol = StreamProtocol::new("/knapsack/1.0.0");

pub struct NetworkManager {
    swarm: Swarm<KnapSackBehaviour>,
}

impl NetworkManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());

        // Use the tokio-specific TCP transport
        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&keypair)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = KnapSackBehaviour::new(peer_id);
        let swarm = Swarm::new(transport, behaviour, peer_id, libp2p::swarm::Config::with_tokio_executor());

        Ok(Self { swarm })
    }

    pub async fn run(&mut self) {
        // Listen on all interfaces
        self.swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();

        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(event) => {
                    // Handle network events
                    println!("Network event: {:?}", event);
                }
                _ => {}
            }
        }
    }
}

#[derive(NetworkBehaviour)]
pub struct KnapSackBehaviour {
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub request_response: request_response::cbor::Behaviour<KnapRequest, KnapResponse>,
}

impl KnapSackBehaviour {
    fn new(local_peer_id: PeerId) -> Self {
        let kademlia =
            kad::Behaviour::new(local_peer_id, kad::store::MemoryStore::new(local_peer_id));

        let request_response = request_response::cbor::Behaviour::new(
            [(KNAPSACK_PROTOCOL, request_response::ProtocolSupport::Full)],
            request_response::Config::default(),
        );

        Self {
            kademlia,
            request_response,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KnapRequest {
    Metadata(String), // Video hash
    Chunk(String),    // Chunk hash
    Search(String),   // Search query
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KnapResponse {
    Metadata(Vec<u8>),
    Chunk(Vec<u8>),
    SearchResults(Vec<VideoMetadata>),
    NotFound,
}