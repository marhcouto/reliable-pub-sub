#![allow(unused_imports)]
use meic_mq::context::{ subscriber::SubscriberContext, publisher::PublisherContext };
use meic_mq::messages::subscribe::{ Request as SubRequest, Reply as SubReply };
use meic_mq::messages::unsubscribe::{ Request as UnsubRequest, Reply as UnsubReply };
use meic_mq::messages::get::{ Request as GetRequest, Reply as GetReply, Ack as GetAck, AckReply as GetAckReply };
use meic_mq::{ get, subscribe, unsubscribe };

use bson;
use serde::{ Serialize, Deserialize };
use std::clone;


pub fn subscriber1() {

    let sub_request: SubRequest = SubRequest::new(String::from("SubD"), String::from("Guns"));
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    let mut sub_ctx: SubscriberContext;
    match subscribe(&socket, &sub_request) {
        Ok(ctx) => {
            sub_ctx = ctx;
            println!("Subscribed successfully");
        },
        Err(val) => panic!("Subscribe failed with error {}", val)
    }

    let get_request: GetRequest = GetRequest::new(sub_ctx.sub_id.clone());
    match get(&socket, &mut sub_ctx, get_request) {
        Ok(payload) => println!("Payload: {:?}", payload),
        Err(val) => panic!("Get request failed with error {}", val)
    }

    let unsub_request: UnsubRequest = UnsubRequest::new(sub_ctx.sub_id.clone());
    match unsubscribe(&socket, &unsub_request) {
        Ok(_) => println!("Unsubscribed successfully"),
        Err(val) => panic!("Get request failed with error {}", val)
    }
}