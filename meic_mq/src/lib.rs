use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

pub mod messages;

#[derive(Debug, Serialize, Deserialize)]
struct SubscriberData {
    sub_id: String,
    topic: String,
    known_broker: Option<String>,
    next_message_id: u64
}

impl SubscriberData {
    pub fn new(sub_id: String, topic: String) -> SubscriberData {
        SubscriberData {
            sub_id: sub_id,
            topic: topic,
            known_broker: None,
            next_message_id: 0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PublisherData {
    pub_id: String,
    broker_id: Option<String>,
    published_messages: HashMap<String, HashSet<u64>>
}

impl PublisherData {
    pub fn new(pub_id: String) -> PublisherData {
        PublisherData { 
            pub_id: pub_id,
            broker_id: None,
            published_messages: HashMap::new() 
        }
    }
}

pub fn get(sub_id: &String, topic: &String) {
    panic!("TODO: implement get")
}

pub fn put(pub_id: &String, topic: &String, message: &Vec<u8>) {
    panic!("TODO: implement put")
}

pub fn subscribe(sub_id: &String, topic: &String) {
    panic!("TODO: implement subscribe")
}

pub fn unsubscribe(sub_id: &String, topic: &String) {
    panic!("TODO: implement unsubscribe")
}
