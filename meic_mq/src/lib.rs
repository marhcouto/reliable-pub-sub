use context::{publisher::PublisherContext, subscriber::SubscriberContext};
use messages::{put, get, NetworkTradeable, Message, error, subscribe, unsubscribe};

pub mod context;
pub mod messages;
 
pub fn get(socket: &zmq::Socket, sub_ctx: &mut SubscriberContext, request: get::Request) -> Result<Vec<u8>, String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let repl_message: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    if repl_message.req_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_message.payload).unwrap();
        return Err(error_struct.description);
    }

    if repl_message.req_type != get::REPLY_HEADER {
        return Err(format!("Unexpected message type '{}' expecting '{}'", repl_message.req_type, put::REPLY_HEADER));
    }

    let repl: get::Reply = bson::from_bson(repl_message.payload).unwrap();
    match &sub_ctx.known_broker_id {
        Some(known_broker_id) => {
            if known_broker_id != &repl.broker_id {
                println!("New Broker in GET");
                // TODO: what is to do when broker is new
            }
            sub_ctx.known_broker_id = Some(repl.broker_id.clone());
        } 
        None => {
            sub_ctx.known_broker_id = Some(repl.broker_id.clone());
        }
    }

    if !repl.match_request(&request) {
        return Err("Unexpected reply for get request".to_string());
    }

    let ack: get::Ack = get::Ack {
        sub_id: repl.sub_id,
        message_no: repl.message_no
    };

    socket.send(ack.as_message().to_bytes().unwrap(), 0).unwrap();
    let ack_repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let ack_repl_message: Message = bson::from_slice(&ack_repl_bytes.as_slice()).unwrap();

    if ack_repl_message.req_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(ack_repl_message.payload).unwrap();
        return Err(error_struct.description);
    }

    if repl_message.req_type != get::ACK_REPLY_HEADER {
        return Err(format!("Unexpected message type '{}' expecting '{}'", repl_message.req_type, put::REPLY_HEADER));
    }

    Ok(repl.payload)
}

pub fn put(socket: &zmq::Socket, pub_ctx: &mut PublisherContext, request: put::Request) -> Result<(), String> {
    if pub_ctx.is_message_new(&request.topic, &request.message_id) {
        println!("Publisher claims that the message was already sent");
    }
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes = socket.recv_bytes(0).unwrap();

    let repl_message: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();
    if repl_message.req_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_message.payload).unwrap();
        return Err(error_struct.description);
    }
    if repl_message.req_type != put::REPLY_HEADER {
        return Err(format!("Unexpected message type '{}' expecting '{}'", repl_message.req_type, put::REPLY_HEADER));
    } 

    let repl: put::Reply = bson::from_bson(repl_message.payload).unwrap(); 
    match &pub_ctx.known_broker_id {
        Some(known_broker_id) => {
            if known_broker_id != &repl.broker_id {
                pub_ctx.reset_context();
                return put(socket, pub_ctx, request);
            }
        }
        None => pub_ctx.known_broker_id = Some(repl.broker_id.clone())
    }

    if !repl.match_request(&request) {
        return Err("Unexpected reply for request".to_string());
    }
    
    Ok(())
}

pub fn subscribe(socket: &zmq::Socket, request: &subscribe::Request) -> Result<SubscriberContext, String>{
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes = socket.recv_bytes(0).unwrap();
    let repl_msg: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    if repl_msg.req_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_msg.payload).unwrap();
        match error_struct.error_type {
            error::BrokerErrorType::SubscriberAlreadyRegistered => return Ok(
                SubscriberContext::new(
                    request.sub_id.clone(),
                    request.topic.clone(),
                    error_struct.broker_id
                )
            ),
            _ => return Err(error_struct.description)
        }
    }

    if repl_msg.req_type != subscribe::REPLY_HEADER {
        return Err(format!("Unexpected message type '{}' expecting '{}'", repl_msg.req_type, subscribe::REPLY_HEADER));
    }

    let repl: subscribe::Reply = bson::from_bson(repl_msg.payload).unwrap();

    if !repl.match_request(&request) {
        return Err("Unexpected reply for subscribe request".to_string());
    }

    return Ok(
        SubscriberContext::new(
            request.sub_id.clone(),
            request.topic.clone(),
            repl.broker_id
        )
    )
}

pub fn unsubscribe(socket: &zmq::Socket, request: &unsubscribe::Request) -> Result<(), String> {
    socket.send(request.as_message().to_bytes().unwrap(), 0).unwrap();
    let repl_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let repl_msg: Message = bson::from_slice(repl_bytes.as_slice()).unwrap();

    if repl_msg.req_type == error::REQUEST_HEADER {
        let error_struct: error::BrokerErrorMessage = bson::from_bson(repl_msg.payload).unwrap();
        match error_struct.error_type {
            error::BrokerErrorType::SubscriberNotRegistered => return Ok(()),
            _ => return Err(error_struct.description)
        }
    }

    if repl_msg.req_type != unsubscribe::REPLY_HEADER {
        return Err(format!("Unexpected message type '{}' expecting '{}'", repl_msg.req_type, unsubscribe::REPLY_HEADER));
    }

    let repl: unsubscribe::Reply = bson::from_bson(repl_msg.payload).unwrap();

    if !repl.match_request(&request) {
        return Err("Unexpected reply for unsubscribe request".to_string());
    }

    Ok(())
}
