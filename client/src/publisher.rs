#![allow(unused_imports)]
use meic_mq::context::{ subscriber::SubscriberContext, publisher::PublisherContext };

use bson;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



pub fn publisher1(args: &Vec<String>) {

    let mut publisher: PublisherContext = PublisherContext::new(String::from("PubG"));

}