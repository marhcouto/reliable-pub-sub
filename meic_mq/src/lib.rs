use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

pub mod context;
pub mod messages;

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
