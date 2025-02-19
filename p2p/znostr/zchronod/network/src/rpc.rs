use std::sync::{Arc, RwLock};
use std::thread;
use tonic::{transport::Server, Request, Response, Status, IntoRequest};
use log::{debug, error, info};
use log::kv::ToKey;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use api::{CONTEXT, RT};
use proto::zchronod::zchronod_server::{Zchronod, ZchronodServer};
use proto::zchronod::{Empty, Event, EventMeta, PollEventState, PollItem, PollListResponse, QueryEventRequest, QueryPollEventRequest, ZchronodRequest, ZchronodResp};
use chronod::Clock;
use chronod::clock::ZMessage;
use storage::ZchronodDb;

#[derive(Clone)]
pub struct RpcServer {
    pub port: String,
}

impl RpcServer {
    pub fn new(self_address: &str) -> Self {
        RpcServer {
            port: self_address.to_string(),
        }
    }

    pub fn run(&self, zc: Sender<ZMessage>, event_handle: std::sync::mpsc::Sender<Event>, db: Arc<RwLock<ZchronodDb>>) -> Result<(), Box<dyn std::error::Error>> {
        println!("rpc run");
        info!("[{}] start rpc listen on {}",module_path!(),self.port);
        println!("[{}] start rpc listen on {}", module_path!(), self.port);
        let addr = self.port.parse()?;
        //  let addr = "127.0.0.1:10020";
        let server = Server::builder()
            .add_service(ZchronodServer::new(init(zc, event_handle, db)))
            .serve(addr);

        tokio::spawn(server);


        Ok(())
    }
}


pub struct ZchronodService {
    send: Sender<ZMessage>,
    cons: std::sync::mpsc::Sender<Event>,
    db: Arc<RwLock<ZchronodDb>>,
}


pub fn init(zc: Sender<ZMessage>, consensus_clone: std::sync::mpsc::Sender<Event>, db: Arc<RwLock<ZchronodDb>>) -> ZchronodService {
    ZchronodService {
        send: zc,
        cons: consensus_clone,
        db: db,
    }
}

#[tonic::async_trait]
impl Zchronod for ZchronodService {
    async fn send(&self, request: Request<ZchronodRequest>) -> Result<Response<ZchronodResp>, Status> {
        println!("Got a request: {:?}", request);
        //  info!("[{}] recv request from {:?}",module_path!(),request.get_ref().msg.as_ref().unwrap().id);
        // self.send.send(Event {
        //     id: vec![],
        //     pubkey: vec![],
        //     created_at: 0,
        //     kind: 0,
        //     tags: vec![],
        //     content: "".to_string(),
        //     sig: vec![],
        // }).await.expect("failed to send");
        unsafe {
            // CONTEXT.as_ref().unwrap().get_network().send(Event{
            //     id: vec![],
            //     pubkey: vec![],
            //     created_at: 0,
            //     kind: 0,
            //     tags: vec![],
            //     content: "".to_string(),
            //     sig: vec![],
            // });
        }
        //todo verify
        self.cons.send(request.into_inner().msg.unwrap()).expect("failed to send cons");
        // if let Some(mut ctx) = unsafe { CONTEXT.as_ref() } {
        //     println!("send msg");
        //     ctx.get_network().send(Event{
        //         id: vec![],
        //         pubkey: vec![],
        //         created_at: 0,
        //         kind: 0,
        //         tags: vec![],
        //         content: "".to_string(),
        //         sig: vec![],
        //     })
        //
        //     //  ctx.get_network().send(request.get_ref().msg.clone().unwrap());
        // } else {
        //     println!("cant get ctx");
        // }


        Ok(Response::new(ZchronodResp {
            resp: None,
        }))
    }

    async fn query_poll_list(&self, request: Request<Empty>) -> Result<Response<PollListResponse>, Status> {
        println!("query_poll_list here");
        let poll_list = self.db.read().unwrap().query_all_event_id().unwrap();
        let mut poll_items: Vec<PollItem> = Vec::new();
        for inner_vec in poll_list {
            let poll_item = PollItem {
                poll_item: inner_vec,
            };
            poll_items.push(poll_item);
        }
        Ok(Response::new(PollListResponse {
            item: poll_items,
        }))
    }

    async fn query_poll_event_state(&self, request: Request<QueryPollEventRequest>) -> Result<Response<PollEventState>, Status> {
        println!("query_poll_event_stat here");
        let state = self.db.read().unwrap().query_poll_event_state(request.into_inner().eventid).unwrap();
        let mut string_vec: Vec<String> = Vec::new();
        for (string_val, int_val) in state {
            let int_as_string = int_val.to_string();
            string_vec.push(string_val);
            string_vec.push(int_as_string);
        }
        Ok(Response::new(PollEventState {
            state: string_vec,
        }))
    }

    async fn query_by_event_id(&self, request: Request<QueryEventRequest>) -> Result<Response<EventMeta>, Status> {
        println!("query_by_event_id here");
        info!("query_by_event_id here");
        return match self.db.read().unwrap().query_by_event_id(request.into_inner().eventid.clone()) {
            Ok(e) => {
                println!("event is [{:?}]", e.clone());
                Ok(Response::new(EventMeta {
                    event: Some(e),
                }))
            }
            Err(_) => {
                println!("event not saved, return err");
                error!("event id saved, return err");
                Err(Status::invalid_argument("event id is invalid"))
            }
        }
    }
}
