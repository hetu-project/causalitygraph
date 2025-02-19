use std::collections::HashMap;
use std::str::Utf8Error;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use log::info;
use serde::{Deserialize, Serialize};
//use log::kv::ToKey;
use serde::de::Unexpected::Str;

use cache::Cache;
use nostr_kv::{Error, lmdb::{*, Db as Lmdb, Iter as LmdbIter}, scanner::{Group, GroupItem, MatchResult, Scanner}};
use nostr_kv::lmdb::Db;
use proto::zchronod::Event;

mod bloomfilter;
mod cache;

pub struct ZchronodDb {
    inner: Db,
    state: Tree,
    cache: Arc<Mutex<Cache>>,
}

#[derive(Serialize, Deserialize)]
pub struct OptionState {
    // map: HashMap<String, i32>,
    option_vec: Vec<(String, i32)>,
    // option_name : vote_num
    event: Event,
}

type Result<T, E = Error> = core::result::Result<T, E>;

const TREE_NAME: &str = "3041";

impl ZchronodDb {
    // kind_301_poll is to init kv
    // kind_309_vote is to update value
    // k : 3041_event-id_state
    // v:  event_state
    // event_state is  map[option_name]vote_num, event
    pub fn new(db_path: String) -> Result<Self> {
        let lmdb = Db::open(db_path)?;
        let state = lmdb.open_tree(Some(TREE_NAME), 0)?;
        let cache = Arc::new(Mutex::new(
            Cache::new(&lmdb, &state)
        ));
        Ok(ZchronodDb {
            inner: lmdb,
            state,
            cache,
        })
    }

    pub fn writer(&self) -> Result<Writer> {
        Ok(self.inner.writer()?)
    }

    pub fn reader(&self) -> Result<Reader> {
        Ok(self.inner.reader()?)
    }


    // let my_path = "./my_file_sglk";
    // let db = Db::open(my_path)?;
    // // let _t = db.open_tree(None, 0)?;
    // let t1 = db.open_tree(Some("t2"), 0)?;
    //
    // // let mut writer = db.writer()?;
    // // writer.put(&t1,b"k1", b"v1")?;
    // let reader = db.reader()?;
    // let _v2 = reader.get(&t1,"k1")?.unwrap();
    // println!("{:?}",std::str::from_utf8(_v2));
    // init kv

    // all poll_id in db is poll_id=event id, vec<string>

    //   let event_id_str = String::from_utf8_lossy(&event_id);

    // fn get_vote_null_option(e: Event) -> HashMap<String, i32> {
    //
    // }
    pub fn poll_write(&self, key: String, e: Event) -> Result<(), Error> {
        println!("poll write key is {:?}", key.clone());
        let reader = self.inner.reader()?;
        if reader.get(&self.state, key.clone())?.is_none() {
            let mut writer = self.inner.writer()?;
            // convert option_state to json, and write as bytes
            if e.tags.len() != 1 {
                println!("tag len != 1, should be panic");
                panic!()
            }
            let poll_tag = e.tags.get(0).unwrap().clone().values;
            // option start with index 5
            // let mut option_hmap: HashMap<String, i32> = HashMap::new();
            let mut option_vec: Vec<(String, i32)> = vec![];
            // ["poll", "single", "0","1707294126","1707294126", "I'm a title!","This a demo survey!" "Option 1", "Option 2", "Option 3"]
            for i in 7..=poll_tag.len() - 1 {
                //option_hmap.insert(poll_tag.get(i).unwrap().to_string(), 0);
                option_vec.push((poll_tag.get(i).unwrap().to_string(), 0));
                println!("insert index {} , which is {}", i, poll_tag.get(i).unwrap().to_string());
            }
            let title = poll_tag.get(5).unwrap().to_string();
            let info = poll_tag.get(6).unwrap().to_string();
            let o_s = OptionState {
                // map: option_hmap,    // to generate option with 0
                option_vec,
                event: e.clone(),

            };
            let option_state = serde_json::to_string(&o_s).unwrap();
            writer.put(&self.state, key.clone(), option_state);
            match reader.get(&self.state, "poll_id".to_string())? {
                Some(t) => {
                    let mut poll_id_list: Vec<Vec<String>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                    println!("get poll_id, {:?}", &poll_id_list);
                    // transfer u8 to hex
                    let s_poll_id: String = hex::encode(e.id.clone());
                    let mut s_poll = vec![];
                    s_poll.extend([s_poll_id, title, info]);
                    poll_id_list.push(s_poll);
                    println!("after update, get poll_id, {:?}", &poll_id_list);
                    // poll_id_list.push(e.id.clone());
                    writer.put(&self.state, "poll_id".to_string(), serde_json::to_string(&poll_id_list).unwrap());
                }
                None => {
                    let mut poll_id_list: Vec<Vec<String>> = Vec::new();
                    let mut s_poll = vec![];
                    let s_poll_id: String = hex::encode(e.id.clone());
                    s_poll.extend([s_poll_id, title, info]);
                    poll_id_list.push(s_poll);
                    let json_write = serde_json::to_string(&poll_id_list).unwrap();
                    println!("write poll_id, {:?}", &json_write);
                    writer.put(&self.state, "poll_id".to_string(), json_write);
                }
            }
            self.cache.lock().unwrap().set_poll_event(key);
            writer.commit()?;
        } else {
            println!("poll write key which is {:?} has saved", key);
        }

        // println!("query test here");
        // drop(reader);
        // let q_result = self.query_all_event_id().unwrap();
        // println!("query poll_id, {:?}", q_result);
        Ok(())
    }

    pub fn event_write(&self, e: Event) -> Result<(), Error> {
        let key: String = hex::encode(e.id.clone());
        let reader = self.inner.reader()?;
        let mut writer = self.inner.writer()?;
        if reader.get(&self.state, key.clone())?.is_none() {
            println!("need to save event [{:?}]", key.clone());
            let event_bytes = serde_json::to_vec(&e).unwrap();
            writer.put(&self.state, key.clone(), event_bytes);
            writer.commit()?;
            // let serialized = serde_json::to_vec(&event_1).unwrap();
            // let _v2 = reader.get(&t2,"k4")?.unwrap();
            // let deserialized: Event = serde_json::from_slice(&_v2).unwrap();
        } else {
            println!("event id has been saved, dont need to write event id which is [{:?}]", key.clone());
            return Err(Error::Message("event id has saved".to_string()));
        }

        Ok(())
    }
    pub fn vote_write(&self, e: Event) -> Result<(), Error> {
        // construct key
        let mut vote_tag = e.tags.clone();
        let mut event_id = "".to_string();
        let mut option_vote: Vec<String> = vec![];
        let event_symbol = "e".to_string();
        let reader = self.inner.reader()?;
        // should be once in item
        for item in &mut vote_tag {
            if item.values.get(0).unwrap().to_string() == event_symbol {
                event_id = item.values.get(1).unwrap().to_string();
            }
            if item.values.get(0).unwrap().to_string() == "poll_r".to_string() {
                // option start with 1
                for i in 1..=item.values.len() - 1 {
                    option_vote.push(item.values.get(i).unwrap().to_string());
                    println!("insert poll_r index {} , which is {}", i, item.values.get(i).unwrap().to_string());
                }
            }
        }

        // check single mutil poll, single option vote len should be 1
        let poll_event = match reader.get(&self.state, event_id.clone())? {
            Some(t) => {
                let event: Event = serde_json::from_slice(t).unwrap();
                event
            }
            None => {
                println!("find none in query_by_event_id");
                info!("find none in query_by_event_id");
                Event::default()
            }
        };

        if poll_event.id.len() == 0 {
            return Err(Error::Message("poll event id not found".to_string()));
        }

        if poll_event.tags[0].values[1].to_string() == "single".to_string() && option_vote.len() != 1 {
            println!("vote option len should be 1 in single option vote, whose len is {:?}", option_vote.len());
            info!("vote option len should be 1 in single option vote, whose len is {:?}", option_vote.len());
            return Err(Error::Message("single option vote len should be 1".to_string()));
        }

        let key = format!("3041_{}_state", event_id.clone());
        println!("vote write key is {:?}, should find this poll_key in db", key.clone());
        // read state, update, write

        let state = std::str::from_utf8(reader.get(&self.state, key.to_string())?.unwrap()).unwrap();
        let mut op_read_state: OptionState = serde_json::from_str(state).unwrap();

        // update
        for vote in &option_vote {
            let vote_index: usize = vote.parse().unwrap();
            if let Some(mut tuple) = op_read_state.option_vec.get_mut(vote_index) {
                tuple.1 += 1;
                println!("{:?}", tuple);
            }
        }

        println!("after update vote {:?}", &op_read_state.option_vec);

        // write
        let mut writer = self.inner.writer()?;
        let wirte_json_string = serde_json::to_string(&op_read_state).unwrap();
        writer.put(&self.state, key.to_string(), wirte_json_string).expect("failed to put vote state");
        writer.commit()?;

        // test query poll_event state
        drop(reader);
        let state_poll = self.query_poll_event_state(event_id).unwrap();
        println!("vote test state_poll is [{:?}]", state_poll);
        Ok(())
    }

    pub fn query_poll_event_state(&self, event_id: String) -> Result<Vec<(String, i32)>, Error> {
        // construct key
        let key = format!("3041_{}_state", event_id);
        println!("vote write key is {:?}", key.clone());

        // bloom query
        if !self.cache.lock().unwrap().validate_poll_event(key.clone()) {
            info!("query_poll_event_state via bloom filter is none which id is [{:?}]",event_id.clone());
            return Ok(vec![]);
        }
        let reader = self.inner.reader()?;
        if reader.get(&self.state, key.clone())?.is_none() {
            info!("query_poll_event_state is none which id is [{:?}]",event_id);
            println!("query_poll_event_state is none which id is [{:?}]", event_id);
            return Ok(vec![]);
        }
        let state = std::str::from_utf8(reader.get(&self.state, key.to_string())?.unwrap()).unwrap();

        let op_state: OptionState = serde_json::from_str(state).unwrap();
        let mut result: Vec<(String, i32)> = vec![];
        for element in op_state.option_vec {
            result.push(element);
        }
        Ok(result)
    }
    pub fn query_all_event_id(&self) -> Result<Vec<Vec<String>>, Error> {
        let reader = self.inner.reader()?;
        return match reader.get(&self.state, "poll_id".to_string())? {
            Some(t) => {
                let poll_id_list: Vec<Vec<String>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                println!("query all poll_event_id, {:?}", &poll_id_list);
                Ok(poll_id_list)
            }
            None => {
                println!("find none in query_all_event_id");
                info!("find none in query_all_event_id");
                Ok(vec![vec![]])
            }
        };
    }

    fn write_3041_db(&self, key: &str, option_state: HashMap<String, i32>) -> Result<(), Error> {
        let reader = self.inner.reader()?;
        let mut op_state = "".to_string();
        match reader.get(&self.state, key.to_string())? {
            None => {
                return Err(Error::Message("failed to get state in db".to_string()));
            }
            Some(t) => {
                let state_bytes = reader.get(&self.state, key.to_string());
                match reader.get(&self.state, key.to_string()) {
                    Ok(s) => {
                        match std::str::from_utf8(s.unwrap()) {
                            Ok(i) => { op_state = i.to_string() }
                            Err(_) => {
                                return Err(Error::Message("failed to transfer to string".to_string()));
                            }
                        }
                    }
                    Err(_) => { return Err(Error::Message("failed to get state in db".to_string())); }
                }
                //  op_state = std::str::from_utf8(state_bytes);
            }
        }


        Ok(())
    }

    pub fn query_by_event_id(&self, event_id: String) -> Result<Event, Error> {
        let reader = self.inner.reader()?;
        return match reader.get(&self.state, event_id)? {
            Some(t) => {
                let event: Event = serde_json::from_slice(t).unwrap();
                Ok(event)
            }
            None => {
                println!("find none in query_by_event_id");
                info!("find none in query_by_event_id");
                Ok(Default::default())
            }
        };
    }
}