pub mod clock;

pub use clock::Clock;

use api::*;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::mpsc::sync_channel;
use proto::zchronod::Event;

pub struct ConsensusTest {
  //  pub buf: Vec<u8>,
   pub receive: Receiver<Event>,
    pub send: Sender<Event>,
}

pub fn init() -> ConsensusTest {
    let (sender, receiver) = mpsc::channel();
    ConsensusTest { receive: receiver, send: sender }
}

impl ConsensusTest {

    pub fn receive(&self) -> Sender<Event> {
        self.send.clone()
    }
}