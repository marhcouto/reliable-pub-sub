use context::{ subscriber::SubscriberContext };
use messages::{ put, get, NetworkTradeable, Message, error, subscribe, unsubscribe };

use lazy_static::lazy_static;
use zmq;
use std::sync::Mutex;

pub mod context;
pub mod messages;

lazy_static! {
    static ref SOCKET: Mutex<zmq::Socket> = Mutex::new(zmq::Context::new().socket(zmq::REQ).unwrap());
}

pub fn get(sub_ctx: &mut SubscriberContext, request: &get::Request) -> Result<Vec<u8>, String> {
    let socket: &zmq::Socket = &SOCKET.lock().unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    _get(&socket, sub_ctx, request)
}

fn _get(socket: &zmq::Socket, sub_ctx: &mut SubscriberContext, request: &get::Request) -> Result<Vec<u8>, String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let repl_message: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    // Error Message
    if repl_message.msg_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_message.payload).unwrap();
        match &sub_ctx.known_broker_id {
            Some(val) => {
                if *val != error_struct.broker_id {
                    return Err("The broker has wiped out its data, need to subscribe again".to_owned())
                }
            },
            _ => {},
        }
        match error_struct.error_type {
            error::BrokerErrorType::InhexistantTopic => panic!("Inexistant topic in get reply"),
            _ => {}
        }
        return Err(error_struct.description);
    }

    // Unexpected Message Type
    if repl_message.msg_type != get::REPLY_HEADER {
        panic!("Unexpected message type '{}' expecting '{}'", repl_message.msg_type, put::REPLY_HEADER);
    }

    // New broker
    let repl: get::Reply = bson::from_bson(repl_message.payload).unwrap();
    match &sub_ctx.known_broker_id {
        Some(known_broker_id) => {
            if known_broker_id != &repl.broker_id {
                return Err("The broker has wiped out its data, need to subscribe again".to_owned());
            }
        } 
        None => {
            sub_ctx.known_broker_id = Some(repl.broker_id.clone());
        }
    }

    // Acknowledgement
    let ack: get::Ack = get::Ack {
        sub_id: repl.sub_id,
        message_no: repl.message_no
    };

    socket.send(ack.as_message().to_bytes().unwrap(), 0).unwrap();
    let ack_repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let ack_repl_message: Message = bson::from_slice(&ack_repl_bytes.as_slice()).unwrap();

    // Error message
    if ack_repl_message.msg_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(ack_repl_message.payload).unwrap();
        match &sub_ctx.known_broker_id {
            Some(val) => {
                if *val != error_struct.broker_id {
                    return Err("The broker has wiped out its data, need to subscribe again".to_owned())
                }
            },
            _ => {},
        }
        match error_struct.error_type {
            error::BrokerErrorType::SubscriberNotRegistered => panic!("Subscriber not registered in ack reply"),
            error::BrokerErrorType::AckMessageMismatch => panic!("Ack message mismatch: ack message is not on the same message number"),
            error::BrokerErrorType::NotExpectingAck => panic!("Broker was not expecting ack message"),
            _ => {}
        }
        return Err(error_struct.description);
    }

    // Unexpected Message Type
    if ack_repl_message.msg_type != get::ACK_REPLY_HEADER {
        panic!("Unexpected message type '{}' expecting '{}'", repl_message.msg_type, get::ACK_REPLY_HEADER);
    }

    // If the message received was not the desired one
    if sub_ctx.next_post_no > repl.message_no {
        return _get(socket, sub_ctx, request);
    } else if sub_ctx.next_post_no < repl.message_no {
        panic!("Message number is higher than the one expected to receive");
    }

    sub_ctx.increment_next_post_no();
    Ok(repl.payload)
}

pub fn put(request: &put::Request) -> Result<(), String> {
    let socket: &zmq::Socket = &SOCKET.lock().unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    _put(&socket, request)
}

fn _put(socket: &zmq::Socket, request: &put::Request) -> Result<(), String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes = socket.recv_bytes(0).unwrap();
    let repl_message: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    // Error message
    if repl_message.msg_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_message.payload).unwrap();
        return Err(error_struct.description);
    }

    // Unexpected message type
    if repl_message.msg_type != put::REPLY_HEADER {
        panic!("Unexpected message type '{}' expecting '{}'", repl_message.msg_type, put::REPLY_HEADER);
    } 
    
    Ok(())
}

pub fn subscribe(sub_ctx: &mut SubscriberContext, request: &subscribe::Request) -> Result<(), String> {
    let socket: &zmq::Socket = &SOCKET.lock().unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    _subscribe(&socket, sub_ctx, request)
}

fn _subscribe(socket: &zmq::Socket, sub_ctx: &mut SubscriberContext, request: &subscribe::Request) -> Result<(), String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes = socket.recv_bytes(0).unwrap();
    let repl_msg: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    // Error message
    if repl_msg.msg_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_msg.payload).unwrap();
        return Err(error_struct.description);
    }

    // Unexpected message type
    if repl_msg.msg_type != subscribe::REPLY_HEADER {
        panic!("Unexpected message type '{}' expecting '{}'", repl_msg.msg_type, subscribe::REPLY_HEADER);
    }

    let repl: subscribe::Reply = bson::from_bson(repl_msg.payload).unwrap();

    sub_ctx.known_broker_id = Some(repl.broker_id);
    sub_ctx.next_post_no = repl.post_offset;

    Ok(())
}

pub fn unsubscribe(sub_ctx: &mut SubscriberContext, request: &unsubscribe::Request) -> Result<(), String> {
    let socket: &zmq::Socket = &SOCKET.lock().unwrap();
    assert!(socket.connect("tcp://localhost:5555").is_ok());

    _unsubscribe(&socket, sub_ctx, request)
}

fn _unsubscribe(socket: &zmq::Socket, sub_ctx: &mut SubscriberContext, request: &unsubscribe::Request) -> Result<(), String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let repl_msg: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    // Error message
    if repl_msg.msg_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_msg.payload).unwrap();
        match error_struct.error_type {
            error::BrokerErrorType::SubscriberNotRegistered => return Ok(()),
            _ => return Err(error_struct.description)
        }
    }

    // Unexpected message type
    if repl_msg.msg_type != unsubscribe::REPLY_HEADER {
        panic!("Unexpected message type '{}' expecting '{}'", repl_msg.msg_type, unsubscribe::REPLY_HEADER);
    }

    sub_ctx.known_broker_id = None;

    Ok(())
}
