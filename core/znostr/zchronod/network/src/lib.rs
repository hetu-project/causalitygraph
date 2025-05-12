extern crate alloc;

use std::sync::Arc;
use log::info;
use tokio::task;
pub use gossip::GossipServer;
pub use rpc::RpcServer;

pub mod gossip;



pub mod rpc;

use api::NetworkInterface;

use proto::zchronod::Event;


// pub struct Network {
//     gossip_server: GossipServer,
//     rpc_server: RpcServer,
// }
//
// pub struct Config {
//     // Add fields for the Tonic RPC server configuration
//     address: String,
//     port: u16,
//     // Add other configuration fields as needed
// }
//
// impl Network {
//     pub fn init(peers: &Vec<String>, rpc: &str, gossip: &str) -> Self {
//         Network {
//             gossip_server: GossipServer::new(peers, gossip),
//             rpc_server: RpcServer::new(rpc),
//         }
//     }
// }
//
//
// impl Network {
//    pub async fn run(&mut self) {
//         // println!("network run");
//         // info!("network run");
//         // self.rpc_server.run().await.expect("failed to run rpc server");
//         // // Start gossip server
//         // unsafe {
//         //     //  let mut gossip = Arc::clone(&self.gossip_server);
//         //
//         //
//         //     self.gossip_server.start().await;
//         // }
//     }
//
//     // fn send(&self, msg: Clock) {
//     //     println!("gossip send");
//     //     self.gossip_server.send(msg);
//     // }
// }

