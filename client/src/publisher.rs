#![allow(unused_imports)]
use meic_mq::context::{ subscriber::SubscriberContext, publisher::PublisherContext };
use meic_mq::messages::put::{ Request, Reply };
use meic_mq::put;

use bson;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::clone;


pub fn publisher1() {

    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    let mut publisher: PublisherContext = PublisherContext::new(String::from("PubG"));
    let payload: Vec<u8> = vec![1, 2, 3, 4, 5];
    let request: Request = Request::new(publisher.pub_id.clone(), String::from("Guns"), String::from("SHOULD BE A UUID"), payload);

    match put(&socket, &mut publisher, request) {
        Ok(_) => println!("Published successfully"),
        Err(val) => panic!("Get request failed with error {}", val)
    }
}