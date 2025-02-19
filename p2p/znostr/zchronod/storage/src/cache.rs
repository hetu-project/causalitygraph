use std::collections::HashMap;
use log::info;
use crate::bloomfilter::BloomFilter;
use nostr_kv::lmdb::{Db, Transaction, Tree};
use proto::zchronod::Event;

pub struct Cache {
    query_bloom: BloomFilter<String>,
    //poll_state: HashMap<String,(String,i32)> // key: poll_state
}

impl Cache {
    pub fn new(db: &Db, tree: &Tree) -> Cache {
        let reader = db.reader().unwrap();
        let mut bloom = BloomFilter::with_capacity(10240);
        let poll_id_l = match reader.get(tree, "poll_id".to_string()).unwrap() {
            Some(t) => {
                let poll_id_list: Vec<Vec<String>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                println!("query all poll_event_id, {:?}", &poll_id_list);
                poll_id_list
            }
            None => {
                println!("find none in query_all_event_id");
                info!("find none in query_all_event_id");
                vec![vec![]]
            }
        };
        if poll_id_l.len() == 0 {
            return Cache {
                query_bloom: bloom,
            };
        }
        for poll_event_id in &poll_id_l {
            if poll_event_id.len() == 0 {
                continue;
            }
            let event_id = poll_event_id[0].clone();
            bloom.set_item(&event_id);
            // match reader.get(&tree, event_id).unwrap() {
            //     Some(t) => {
            //         let event: Event = serde_json::from_slice(t).unwrap();
            //         if event.kind==301 {
            //
            //         }
            //     }
            //     None => {
            //         println!("find none in query_by_event_id");
            //         info!("find none in query_by_event_id");
            //     }
            // };
        }

        Cache {
            query_bloom: bloom,
            // poll_state: Default::default(),
        }
    }

    pub fn validate_poll_event(&self, poll_event_id: String) -> bool {
        return self.query_bloom.might_contain(&poll_event_id);
    }

    pub fn set_poll_event(&mut self, poll_event_id: String) {
        self.query_bloom.set_item(&poll_event_id);
    }
}