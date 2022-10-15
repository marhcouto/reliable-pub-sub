use context::{publisher::PublisherContext, subscriber::SubscriberContext};
use messages::{put, NetworkTradable, Message, error, subscribe};

pub mod context;
pub mod messages;
 
pub fn get(sub_id: &String, topic: &String) {
    panic!("TODO: implement get")
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
        return Err("Unexpected reply for request".to_string());
    }

    return Ok(
        SubscriberContext::new(
            request.sub_id.clone(),
            request.topic.clone(),
            repl.broker_id
        )
    )
}

pub fn unsubscribe(sub_id: &String, topic: &String) {
    panic!("TODO: implement unsubscribe")
}
