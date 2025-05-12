use std::println;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, SyncSender};
use async_trait::async_trait;
use lazy_static::lazy::Lazy;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use proto::zchronod::Event;


pub static mut CONTEXT: Option<Node> = None;

lazy_static! {
    pub static ref RT: Runtime = Runtime::new().unwrap();
}
pub struct Node {
    network: Box<dyn NetworkInterface>,
    //consensus: Box<dyn ConsensusInterface>,
}

impl Node {
    pub fn new(network: Box<dyn NetworkInterface>) -> Self {
        Node {
            network,
        }
    }
    pub fn set_network(&mut self, network: Box<dyn NetworkInterface>) {
        self.network = network;
    }

    // pub fn set_consensus(&mut self, consensus: Box<dyn ConsensusInterface>) {
    //     self.consensus = consensus;
    // }
    pub fn get_network(&self) -> &Box<dyn NetworkInterface> {
        &self.network
    }
    pub async fn run(&mut self) {
        self.network.run().await;
    }

    // pub  fn send(&mut self) {
    //     self.network.send();
    // }
}


pub struct NodeForTest {
    network: Box<dyn NetworkInterfaceForTest>,
    //consensus: Box<dyn ConsensusInterface>,
}

struct EmptyNetwork {}

pub trait NetworkInterfaceForTest {
    fn run(&mut self);
    fn send(&self, msg: Event);
}

impl NetworkInterfaceForTest for EmptyNetwork {
    fn run(&mut self) {
        todo!()
    }

    fn send(&self, msg: Event) {
        todo!()
    }
}


#[async_trait]
pub trait NetworkInterface {
    async fn run(&mut self);
    fn send(&self, msg: Event);
}



