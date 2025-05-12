use std::ffi::c_long;
use tokio::task;
use std::sync::{Arc, Mutex};
use std::thread;
use futures::future::ok;
use log::{error, info};
use prost::bytes::Buf;
use prost::Message;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use gossipd::gossipd::{Gossipd, GossipdOptions};
use proto::zchronod::zchronod_server::Zchronod;
use proto::zchronod::Event;
use bytes::Bytes;
use tokio::runtime::Runtime;
use tonic::codegen::Body;
use api::RT;

use {
    libp2p::{
        gossipsub,
        gossipsub::Message as mms,
        mdns, noise,
        swarm::{NetworkBehaviour, SwarmEvent},
        tcp, yamux, Multiaddr, PeerId, Swarm,
    },
};
use chronod::Clock;

pub struct GossipServer<T> {
    pub send: Sender<T>,
    pub gossip: Gossipd<T>,
    pub ip: String,
}

impl <T: Into<Vec<u8>>> GossipServer<T> {
    pub fn new(peers: &Vec<String>, listen_address: &str) -> Self {
        let mut gossip_options = GossipdOptions::default();
        gossip_options.listen_addr = listen_address.to_string();
        for peer in peers {
            info!("add peer {}", peer);
            gossip_options.add_peer(peer.to_string());// format as /ip4/192.168.0.1/tcp/80
        }
        let mut gossip: Gossipd<T> = Gossipd::new(gossip_options);
        // gossip.with_handler(|peer_id, message| {
        //     println!("{peer_id}: {}", String::from_utf8_lossy(&message.data))
        // });
        GossipServer { send: gossip.create_sender(), gossip, ip: listen_address.to_string() }
    }


    pub fn register_receive(&mut self, f: std::sync::mpsc::Sender<(PeerId,mms)>) {
        self.gossip.register_distributor(f);
    }

    // pub async fn send(&self, msg: Clock) -> Result<(), String> {
    //     println!("step2 gossip");
    //     match self.send.send(msg).await {
    //         Ok(()) => {
    //             Ok(())
    //         }
    //         Err(_) => {
    //             error!("failed to send msg");
    //             Err("faild to send".to_string())
    //         }
    //     }
    // }


    pub async fn start(&mut self) {
        info!("[{}] start gossip listen on {}",module_path!(),self.ip);
        println!("[{}] start gossip listen on {}", module_path!(), self.ip);
        // thread::spawn(move || {
        //     let rt = Runtime::new().unwrap();
        //     rt.spawn(self.gossip.start());
        //     loop {}
        // });
        // tokio::spawn(async move { self.gossip.start().await });
        self.gossip.start().await;
        println!("gossip start");
    }
}