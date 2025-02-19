use crate::{message::*, setting::SettingWrapper, Reader, Subscriber, Writer};
use actix::prelude::*;
use nostr_db::{CheckEventResult, Db, Event};
use std::{collections::HashMap, env, sync::Arc, thread};
use std::fmt::Error;
use std::future::Future;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use tonic::Status;
use tonic::transport::Channel;
use tracing::info;
use crate::zchronod::zchronod_client::ZchronodClient;
use crate::zchronod::{Empty, QueryEventRequest, QueryPollEventRequest, TagArray, ZchronodRequest, ZchronodResp};
use crate::zchronod::Event as c_Event;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::message::IncomingMessage::{Query, QueryEventMeta, QueryPollList};
use tokio::runtime::Runtime;
use serde_json::{json, Value};

/// Server
#[derive(Debug)]
pub struct Server {
    id: usize,
    writer: Addr<Writer>,
    reader: Addr<Reader>,
    subscriber: Addr<Subscriber>,
    sessions: HashMap<usize, Recipient<OutgoingMessage>>,
    zchronod_ip: String,
}

impl Server {
    pub fn create_with(db: Arc<Db>, setting: SettingWrapper) -> Addr<Server> {
        let r = setting.read();
        let num = if r.thread.reader == 0 {
            num_cpus::get()
        } else {
            r.thread.reader
        };
        drop(r);

        Server::create(|ctx| {
            let writer = Writer::new(Arc::clone(&db), ctx.address().recipient()).start();
            let subscriber = Subscriber::new(ctx.address().recipient(), setting.clone()).start();
            let addr = ctx.address().recipient();
            info!("starting {} reader workers", num);
            let zchronod_ip = setting.clone().read().zchronod.ip.clone();
            let reader = SyncArbiter::start(num, move || {
                Reader::new(Arc::clone(&db), addr.clone(), setting.clone())
            });

            Server {
                id: 0,
                writer,
                reader,
                subscriber,
                sessions: HashMap::new(),
                zchronod_ip: zchronod_ip,
            }
        })
    }

    fn send_to_client(&self, id: usize, msg: OutgoingMessage) {
        if let Some(addr) = self.sessions.get(&id) {
            addr.do_send(msg);
        }
    }
}

/// Make actor from `Server`
impl Actor for Server {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(10000);
        info!("Actor server started");
    }
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        info!("receive here");
        if self.id == usize::MAX {
            self.id = 0;
        }
        self.id += 1;
        self.sessions.insert(self.id, msg.addr);
        // send id back
        self.id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        // remove address
        self.sessions.remove(&msg.id);

        // clear subscriptions
        self.subscriber.do_send(Unsubscribe {
            id: msg.id,
            sub_id: None,
        });
    }
}

fn get_current_system_time() -> i64 {
    let current_time = SystemTime::now();
    let duration = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = duration.as_secs() as i64;
    timestamp
}

impl Server {
    fn query_event_meta(&self, event_id: String) -> String {
        println!("query event meta here");

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            //let zi = self.zchronod_ip.clone();
            let zi = "127.0.0.1:10020".to_string();


            // let sender = Arc::new(sender);
            rt.block_on(async move {
                let grpc_server_addr = format!("http://{}", zi);

                let mut client = ZchronodClient::connect(grpc_server_addr).await.unwrap();


                let request = tonic::Request::new(QueryEventRequest {
                    eventid: event_id,
                });

                match client.query_by_event_id(request).await {
                    Ok(response) => {
                        let pl = response.into_inner().event;
                        println!("Received response: {:?}", &pl);
                        sender.send(pl).unwrap();
                    }
                    Err(_) => {
                        println!("failed to send")
                    }
                }
            });
        });

        let received = receiver.recv().unwrap().unwrap();
        serde_json::to_string(&received).unwrap()
    }

    fn query_poll_list(&self) -> Vec<Vec<String>> {
        println!("query poll list here");

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            //let zi = self.zchronod_ip.clone();
            let zi = "127.0.0.1:10020".to_string();


            // let sender = Arc::new(sender);
            rt.block_on(async move {
                let grpc_server_addr = format!("http://{}", zi);

                let mut client = ZchronodClient::connect(grpc_server_addr).await.unwrap();


                let request = tonic::Request::new(Empty {});

                match client.query_poll_list(request).await {
                    Ok(response) => {
                        let pl = response.into_inner().item;
                        println!("Received response: {:?}", &pl);
                        sender.send(pl).unwrap();
                    }
                    Err(_) => {
                        println!("failed to send")
                    }
                }
            });
        });

        let received_vec = receiver.recv().unwrap();
        let mut vec_string = vec![];
        for i in received_vec {
            vec_string.push(i.poll_item);
        }
        // drop(rt);
        vec_string
    }
    async fn send_to_chronod_event(&self, e: Event) {
        println!("send to chronod_event here");
        let mut rt = Runtime::new().unwrap();
        //let zi = self.zchronod_ip.clone();
        let zi = "127.0.0.1:10020".to_string();

        println!("in send to chronod future");
        let e_c = e.clone();
        let grpc_server_addr = format!("http://{}", zi);

        let mut client = ZchronodClient::connect(grpc_server_addr).await.unwrap();

        let t: Vec<TagArray> = e_c.tags().into_iter()
            .map(|inner_vec| TagArray { values: inner_vec.clone() })
            .collect();
        let request = tonic::Request::new(ZchronodRequest {
            msg: Some(c_Event {
                id: vec![1],
                pubkey: vec![6, 7],
                created_at: 2,
                kind: 2,
                tags: vec![],
                content: "hello12333".to_string(),
                sig: vec![7, 8, 9, 10],
            })
        });
        info!("should send");
        // let request = tonic::Request::new(ZchronodRequest {
        //     msg: Some(c_Event {
        //         id: Vec::from(e_c.id()),
        //         pubkey: Vec::from(e_c.pubkey()),
        //         created_at: get_current_system_time(),
        //         kind: e_c.kind() as u32,
        //         tags: t,
        //         content: e_c.content().clone(),
        //         sig: Vec::from(e_c.sig()),
        //     })
        // });
        let response = client.send(request).await.unwrap().into_inner();
        println!("Received response: {:?}", response);
        // match client.send(request).await {
        //     Ok(response) => {
        //         println!("Received response: {:?}", response.into_inner());
        //     }
        //     Err(_) => {
        //         println!("failed to send")
        //     }
        // }

        //drop(rt);
    }
}

async fn query_poll_state(tx: Sender<Vec<String>>, eventid: String) {
    //  let mut rt = Runtime::new().unwrap();
    //let zi = self.zchronod_ip.clone();
    let zi = "127.0.0.1:10020".to_string();

    // let sender = Arc::new(sender);

    let grpc_server_addr = format!("http://{}", zi);

    let mut client = ZchronodClient::connect(grpc_server_addr).await.unwrap();


    let request = tonic::Request::new(QueryPollEventRequest {
        eventid: eventid,
    });

    let resp = client.query_poll_event_state(request).await.unwrap().into_inner().state;

    // let resp= client.query_poll_list(request).await.unwrap().into_inner().poll_list;
    // let json_string = serde_json::to_string(&resp).unwrap();

    tx.send(resp).expect("failed to send");
    //     match client.query_poll_list(request).await {
    //         Ok(response) => {
    //             let pl = response.into_inner().poll_list;
    //             println!("Received response: {:?}", &pl);
    //             sender.send(pl).unwrap();
    //         }
    //         Err(_) => {
    //             println!("failed to send")
    //         }
    // });
    // let received_vec = receiver.recv().unwrap();
    // drop(rt);
    // received_vec
}

async fn send_to_chronod_event(e: Event) {
    println!("send to chronod_event here");
    //let zi = self.zchronod_ip.clone();
    let zi = "127.0.0.1:10020".to_string();

    println!("in send to chronod future");
    let e_c = e.clone();
    let grpc_server_addr = format!("http://{}", zi);
    println!("{:?}", e_c.id());
    let mut client = ZchronodClient::connect(grpc_server_addr).await.unwrap();

    let t: Vec<TagArray> = e_c.tags().into_iter()
        .map(|inner_vec| TagArray { values: inner_vec.clone() })
        .collect();
    // let request = tonic::Request::new(ZchronodRequest {
    //     msg: Some(c_Event {
    //         id: vec![1],
    //         pubkey: vec![6, 7],
    //         created_at: 2,
    //         kind: 2,
    //         tags: vec![],
    //         content: "hello12333".to_string(),
    //         sig: vec![7, 8, 9, 10],
    //     })
    // });
    info!("should send");
    println!("{:?}", e_c.id());
    let hex_string_back: String = hex::encode(e_c.id());
    println!("here string id back {:?}", hex_string_back);
    let request = tonic::Request::new(ZchronodRequest {
        msg: Some(c_Event {
            id: Vec::from(e_c.id()),
            pubkey: Vec::from(e_c.pubkey()),
            created_at: get_current_system_time(),
            kind: e_c.kind() as u32,
            tags: t,
            content: e_c.content().clone(),
            sig: Vec::from(e_c.sig()),
        })
    });
    let response = client.send(request).await.unwrap().into_inner();
    println!("Received response: {:?}", response);
    // match client.send(request).await {
    //     Ok(response) => {
    //         println!("Received response: {:?}", response.into_inner());
    //     }
    //     Err(_) => {
    //         println!("failed to send")
    //     }
    // }
}

fn transfer_query(json_str: String) -> String {
    let json_value: Value = serde_json::from_str(&*json_str).unwrap();
    let mut event_id = "".to_string();
    if let Value::Array(json_array) = json_value {
        if let Some(query_value) = json_array.get(0) {
            if let Some(query_string) = query_value.as_str() {
                println!("first act is : {}", query_string);
                if query_string != "QUERY" {
                    return "".to_string();
                }
            }
        }

        if let Some(string_value) = json_array.get(1) {
            if let Some(string) = string_value.as_str() {
                println!("String: {}", string);
                event_id = string.to_string();
            }
        }
    }

    event_id
}

/// Handler for Message message.
impl Handler<ClientMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        println!("sglk: receive here {:?}", msg);
        let msg_c = msg.clone();
        // let event_id = transfer_query(msg_c.text);
        // if event_id !="" {
        //     info!("receive query poll here");
        //     println!("receive query poll here");
        //     let (tx, recv) = mpsc::channel();
        //     let cs = event_id.clone();
        //     thread::spawn(move || {
        //         let mut rt = Runtime::new().unwrap();
        //         rt.block_on(async { query_poll_state(tx, cs).await; })
        //     });
        //
        //     let poll_list = recv.recv().unwrap();
        //     let json_string = json!(poll_list);
        //     //  let json_string = serde_json::to_string(&poll_list).unwrap();
        //     self.send_to_client(msg.id, OutgoingMessage {
        //         0: json_string.to_string(),
        //     });
        // }
        match msg.msg {
            QueryEventMeta(s) => {
                info!("receive query event meta here");
                println!("receive query event meta here");
                let e = self.query_event_meta(s);
                println!("query event meta[{:?}]", e);
                self.send_to_client(msg.id, OutgoingMessage {
                    0: e,
                });
            }
            QueryPollList(_) => {
                info!("receive query poll list here");
                println!("receive query poll list here");
                let poll_list = self.query_poll_list();
                // key is 3041_event-id_state
                println!("receive poll_list");
                //  let mut list = vec!();
                // for element in poll_list {
                //     let hex_string_back: String = hex::encode(element);
                //     println!("{:?}", &hex_string_back);
                //     list.push(hex_string_back);
                // }
                let json_string = json!(poll_list);
                self.send_to_client(msg.id, OutgoingMessage {
                    0: json_string.to_string(),
                });
            }
            Query(s) => {
                info!("receive query poll here");
                println!("receive query poll here");
                let (tx, recv) = mpsc::channel();
                let cs = s.clone();
                thread::spawn(move || {
                    let mut rt = Runtime::new().unwrap();
                    rt.block_on(async { query_poll_state(tx, cs.id).await; })
                });

                let poll_state = recv.recv().unwrap();
                println!("query poll state, result is [{:?}]", poll_state);
                let json_string = json!(poll_state);
                //  let json_string = serde_json::to_string(&poll_list).unwrap();
                self.send_to_client(msg.id, OutgoingMessage {
                    0: json_string.to_string(),
                });
            }
            IncomingMessage::Event(event) => {
                println!("receive event here");
                // save all event
                // save ephemeral for check duplicate, disconnection recovery, will be deleted
                let cp = event.clone();
                let handle1 = thread::spawn(move || {
                    let cpp = cp.clone();
                    let mut rt = Runtime::new().unwrap();
                    rt.block_on(send_to_chronod_event(cpp))
                });
                handle1.join().unwrap();
                self.send_to_client(msg.id, OutgoingMessage {
                    0: "zchronod has received".to_string(),
                });
                println!("should send back [{:?}]", msg.id);
                //self.send_to_chronod_event(event.clone());
                self.writer.do_send(WriteEvent { id: msg.id, event });
                // test query
                // let poll_list = self.query_poll_list();
                // // key is 3041_event-id_state
                // println!("receive poll_list");
                // for element in poll_list {
                //     //    let hex_string_back: String = hex::encode(element);
                //     println!("{:?}", &element);
                // }

                // test poll state
                // let (tx, recv) = mpsc::channel();
                // thread::spawn(move || {
                //     let mut rt = Runtime::new().unwrap();
                //     rt.block_on(async { query_poll_state(tx, "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98".to_string()).await; })
                // });
                //
                // let poll_state = recv.recv().unwrap();
                // println!("receive poll_state");
                // println!("{:?}", poll_state);
            }
            IncomingMessage::Close(id) => self.subscriber.do_send(Unsubscribe {
                id: msg.id,
                sub_id: Some(id),
            }),
            IncomingMessage::Req(subscription) => {
                let session_id = msg.id;
                let read_event = ReadEvent {
                    id: msg.id,
                    subscription: subscription.clone(),
                };
                self.subscriber
                    .send(Subscribe {
                        id: msg.id,
                        subscription,
                    })
                    .into_actor(self)
                    .then(move |res, act, _ctx| {
                        match res {
                            Ok(res) => match res {
                                Subscribed::Ok => {
                                    act.reader.do_send(read_event);
                                }
                                Subscribed::Overlimit => {
                                    act.send_to_client(
                                        session_id,
                                        OutgoingMessage::notice(
                                            "Number of subscriptions exceeds limit",
                                        ),
                                    );
                                }
                                Subscribed::InvalidIdLength => {
                                    act.send_to_client(
                                        session_id,
                                        OutgoingMessage::notice("Subscription id should be non-empty string of max length 64 chars"),
                                    );
                                }
                            },
                            Err(_err) => {
                                act.send_to_client(
                                    session_id,
                                    OutgoingMessage::notice("Something is wrong"),
                                );
                            }
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            _ => {
                self.send_to_client(msg.id, OutgoingMessage::notice("Unsupported message"));
            }
        }
    }
}

impl Handler<WriteEventResult> for Server {
    type Result = ();
    fn handle(&mut self, msg: WriteEventResult, _: &mut Self::Context) {
        match msg {
            WriteEventResult::Write { id, event, result } => {
                let event_id = event.id_str();
                let out_msg = match &result {
                    CheckEventResult::Ok(_num) => OutgoingMessage::ok(&event_id, true, ""),
                    CheckEventResult::Duplicate => {
                        OutgoingMessage::ok(&event_id, true, "duplicate: event exists")
                    }
                    CheckEventResult::Invald(msg) => {
                        OutgoingMessage::ok(&event_id, false, &format!("invalid: {}", msg))
                    }
                    CheckEventResult::Deleted => {
                        OutgoingMessage::ok(&event_id, false, "deleted: user requested deletion")
                    }
                    CheckEventResult::ReplaceIgnored => {
                        OutgoingMessage::ok(&event_id, false, "replaced: have newer event")
                    }
                };
                self.send_to_client(id, out_msg);
                // dispatch event to subscriber
                if let CheckEventResult::Ok(_num) = result {
                    self.subscriber.do_send(Dispatch { id, event });
                }
            }
            WriteEventResult::Message { id, event: _, msg } => {
                self.send_to_client(id, msg);
            }
        }
    }
}

impl Handler<ReadEventResult> for Server {
    type Result = ();
    fn handle(&mut self, msg: ReadEventResult, _: &mut Self::Context) {
        self.send_to_client(msg.id, msg.msg);
    }
}

impl Handler<SubscribeResult> for Server {
    type Result = ();
    fn handle(&mut self, msg: SubscribeResult, _: &mut Self::Context) {
        self.send_to_client(msg.id, msg.msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{temp_data_path, Setting};
    use actix_rt::time::sleep;
    use anyhow::Result;
    use parking_lot::RwLock;
    use std::time::Duration;

    #[derive(Default)]
    struct Receiver(Arc<RwLock<Vec<OutgoingMessage>>>);

    impl Actor for Receiver {
        type Context = Context<Self>;
    }

    impl Handler<OutgoingMessage> for Receiver {
        type Result = ();
        fn handle(&mut self, msg: OutgoingMessage, _ctx: &mut Self::Context) {
            self.0.write().push(msg);
        }
    }

    #[actix_rt::test]
    async fn message() -> Result<()> {
        let db = Arc::new(Db::open(temp_data_path("server")?)?);
        let note = r#"
        {
            "content": "Good morning everyone 😃",
            "created_at": 1680690006,
            "id": "332747c0fab8a1a92def4b0937e177be6df4382ce6dd7724f86dc4710b7d4d7d",
            "kind": 1,
            "pubkey": "7abf57d516b1ff7308ca3bd5650ea6a4674d469c7c5057b1d005fb13d218bfef",
            "sig": "ef4ff4f69ac387239eb1401fb07d7a44a5d5d57127e0dc3466a0403cf7d5486b668608ebfcbe9ff1f8d3b5d710545999fe08ee767284ec0b474e4cf92537678f",
            "tags": [["t", "nostr"]]
          }
        "#;
        let ephemeral_note = r#"
        {
            "content": "Good morning everyone 😃",
            "created_at": 1680690006,
            "id": "332747c0fab8a1a92def4b0937e177be6df4382ce6dd7724f86dc4710b7d4d78",
            "kind": 20000,
            "pubkey": "7abf57d516b1ff7308ca3bd5650ea6a4674d469c7c5057b1d005fb13d218bfef",
            "sig": "ef4ff4f69ac387239eb1401fb07d7a44a5d5d57127e0dc3466a0403cf7d5486b668608ebfcbe9ff1f8d3b5d710545999fe08ee767284ec0b474e4cf92537678f",
            "tags": [["t", "nostr"]]
          }
        "#;

        let receiver = Receiver::default();
        let messages = receiver.0.clone();
        let receiver = receiver.start();
        let addr = receiver.recipient();

        let server = Server::create_with(db, Setting::default().into());

        let id = server.send(Connect { addr }).await?;
        assert_eq!(id, 1);

        // Unsupported
        {
            let text = r#"["UNKNOWN"]"#.to_owned();
            let msg = serde_json::from_str::<IncomingMessage>(&text)?;
            let client_msg = ClientMessage { id, text, msg };
            server.send(client_msg).await?;
            sleep(Duration::from_millis(50)).await;
            {
                let mut w = messages.write();
                assert_eq!(w.len(), 1);
                assert!(w.get(0).unwrap().0.contains("Unsupported"));
                w.clear();
            }
        }

        // Subscribe
        {
            let text = r#"["REQ", "1", {}]"#.to_owned();
            let msg = serde_json::from_str::<IncomingMessage>(&text)?;
            let client_msg = ClientMessage { id, text, msg };
            server.send(client_msg).await?;
            sleep(Duration::from_millis(50)).await;
            {
                let mut w = messages.write();
                assert_eq!(w.len(), 1);
                assert!(w.get(0).unwrap().0.contains("EOSE"));
                w.clear();
            }

            // write
            let text = format!(r#"["EVENT", {}]"#, note);
            let msg = serde_json::from_str::<IncomingMessage>(&text)?;
            let client_msg = ClientMessage { id, text, msg };
            server.send(client_msg.clone()).await?;
            sleep(Duration::from_millis(200)).await;
            {
                let mut w = messages.write();
                assert_eq!(w.len(), 2);
                assert!(w.get(0).unwrap().0.contains("OK"));
                // subscription message
                assert!(w.get(1).unwrap().0.contains("EVENT"));
                w.clear();
            }
            // repeat write
            server.send(client_msg.clone()).await?;
            sleep(Duration::from_millis(200)).await;
            {
                let mut w = messages.write();
                assert_eq!(w.len(), 1);
                assert!(w.get(0).unwrap().0.contains("OK"));
                // No subscription message because the message is duplicated
                w.clear();
            }

            // ephemeral event
            {
                let text = format!(r#"["EVENT", {}]"#, ephemeral_note);
                let msg = serde_json::from_str::<IncomingMessage>(&text)?;
                let client_msg = ClientMessage { id, text, msg };
                server.send(client_msg.clone()).await?;
                sleep(Duration::from_millis(200)).await;
                {
                    let mut w = messages.write();
                    assert_eq!(w.len(), 2);
                    assert!(w.get(0).unwrap().0.contains("OK"));
                    // subscription message
                    assert!(w.get(1).unwrap().0.contains("EVENT"));
                    w.clear();
                }
                // repeat
                server.send(client_msg.clone()).await?;
                sleep(Duration::from_millis(200)).await;
                {
                    let mut w = messages.write();
                    assert_eq!(w.len(), 1);
                    assert!(w.get(0).unwrap().0.contains("OK"));
                    // No subscription message because the message is duplicated
                    w.clear();
                }
            }

            // unsubscribe

            let text = r#"["CLOSE", "1"]"#.to_owned();
            let msg = serde_json::from_str::<IncomingMessage>(&text)?;
            let client_msg = ClientMessage { id, text, msg };
            server.send(client_msg).await?;
            sleep(Duration::from_millis(50)).await;
            {
                let mut w = messages.write();
                // assert_eq!(w.len(), 1);
                // assert!(w.get(0).unwrap().0.contains("EOSE"));
                w.clear();
            }
        }

        // get
        {
            let text = r#"["REQ", "1", {}]"#.to_owned();
            let msg = serde_json::from_str::<IncomingMessage>(&text)?;
            let client_msg = ClientMessage { id, text, msg };
            server.send(client_msg).await?;
            sleep(Duration::from_millis(50)).await;
            {
                let mut w = messages.write();
                assert_eq!(w.len(), 3);
                assert!(w.get(0).unwrap().0.contains("EVENT"));
                assert!(w.get(1).unwrap().0.contains("EVENT"));
                assert!(w.get(2).unwrap().0.contains("EOSE"));
                w.clear();
            }
        }

        Ok(())
    }
}
