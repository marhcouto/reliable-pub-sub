#![allow(unused_imports)]
use meic_mq::context::{ subscriber::SubscriberContext, publisher::PublisherContext };

use bson;
use serde::{Serialize, Deserialize};


pub fn subscriber(args: &Vec<String>) {

    // TODO: parse subscriber arguments



    let mut subscriber: SubscriberContext = SubscriberContext {
        sub_id: String::from("ID"),
        topic: String::from("bla"),
        known_broker_id: None,
        next_message_id: 0
    };



}