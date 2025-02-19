use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::task::yield_now;

use {
    futures::stream::StreamExt,
    libp2p::{
        gossipsub,
        gossipsub::Message,
        mdns, noise,
        swarm::{NetworkBehaviour, SwarmEvent},
        tcp, yamux, Multiaddr, PeerId, Swarm,
    },
    tokio::{
        io, select,
        sync::mpsc::{Receiver, Sender},
    },
};

#[derive(NetworkBehaviour)]
struct GossipdBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[derive(Clone, Debug)]
pub struct GossipdOptions {
    pub listen_addr: String,
    pub peers: Vec<String>,
}

impl Default for GossipdOptions {
    fn default() -> GossipdOptions {
        GossipdOptions {
            listen_addr: String::from("/ip4/127.0.0.1/tcp/0"),
            peers: vec![],
        }
    }
}

impl GossipdOptions {
    pub fn add_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }
}

pub struct Gossipd<T> {
    // todo: refactor transport.
    // Connection Manager with mdns and gossipsub.
    transport: Swarm<GossipdBehaviour>,
    channel: (Sender<T>, Receiver<T>),
    handler: Option<fn(PeerId, Message)>,
    options: GossipdOptions,
    distributor: Option<std::sync::mpsc::Sender<(PeerId,Message)>>,
}

impl<T: Into<Vec<u8>>> Gossipd<T> {
    // todo: create transport as new struct.
    pub fn new(options: GossipdOptions) -> Gossipd<T> {
        let mut transport = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .expect("failed to create swarm with tcp")
            .with_quic()
            .with_behaviour(|key| {
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_conf = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))
                    .expect("failed to build gossipsub config");

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_conf,
                )
                    .expect("failed to create gossipsub behaviour");

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(GossipdBehaviour { gossipsub, mdns })
            })
            .expect("failed to initialize behaviour")
            .build();

        // default topic
        let topic = gossipsub::IdentTopic::new("gossipd");
        transport
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)
            .expect("failed to subscript topic");

        let channel = tokio::sync::mpsc::channel(4096);

        Gossipd {
            transport,
            channel,
            handler: None,
            options,
            distributor: None,
        }
    }

    pub fn listen(&mut self) {
        self.transport
            .listen_on(self.options.listen_addr.parse().expect("invalid addr"))
            .expect(format!("failed to listen on {}", self.options.listen_addr).as_str());
    }

    pub fn register_distributor(&mut self, distribute: std::sync::mpsc::Sender<(PeerId,Message)>) {
        self.distributor= Option::from(distribute);
    }

    pub async fn start(&mut self) {
        self.listen();
        for peer in &self.options.peers {
            self.transport
                .dial(peer.parse::<Multiaddr>().expect("failed to parse addr"))
                .expect("failed to dial");
            println!("Dialed {peer}")
        }

        loop {
            select! {
                op = self.channel.1.recv() => match op {
                    Some(message) => {
                        if let Err(err) = self.transport.behaviour_mut()
                            .gossipsub.publish(gossipsub::IdentTopic::new("gossipd"), message) {
                            println!("{err}")
                        }
                    },
                    None => println!("none"),
                },
                event = self.transport.select_next_some() => match event {
                    SwarmEvent::Behaviour(GossipdBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _) in list {
                            println!("new peer: {peer_id}");
                            self.transport.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(GossipdBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _) in list {
                            println!("peer has expired: {peer_id}");
                            self.transport.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(GossipdBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: _id,
                        message,
                    })) => {
                        if let Some(distribute) = self.distributor.clone() {
                            distribute.send((peer_id,message)).expect("failed to send")
                        }
                        // if let Some(handler) = self.handler {
                        //     handler(peer_id, message);
                        // }
                    }
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Chronosd is listening on {address}");
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn create_sender(&self) -> Sender<T> {
        self.channel.0.clone()
    }

    pub fn with_handler(&mut self, h: fn(PeerId, Message)) -> &mut Gossipd<T> {
        self.handler = Some(h);

        self
    }
}
