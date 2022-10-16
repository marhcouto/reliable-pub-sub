#![allow(unused_imports)]
use meic_mq::messages::subscribe::{ Request, Reply };
use meic_mq::{ get, subscribe, unsubscribe };

use bson;
use serde::{Serialize, Deserialize};


pub fn subscriber1(args: &Vec<String>) {


    let mut request: Request = Request::new(String::from("SubD"), String::from("Guns"));
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    subscribe(&socket, &request);


}