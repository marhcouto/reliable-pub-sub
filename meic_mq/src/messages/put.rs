use super::{NetworkTradable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

const REQUEST_HEADER: &str = "PUT";
const REPLY_HEADER: &str = "PUT_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub_id: String,
    topic: String,
    message_id: String,
    payload: Vec<u8>
}

impl Request {
    pub fn new(pub_id: String, topic: String, message_id: String, payload: Vec<u8>) -> Request {
        Request {
            pub_id: pub_id,
            topic: topic,
            message_id: message_id,
            payload: payload
        }
    }
}

impl NetworkTradable<Request> for Request {
    fn as_message(&self) -> Message {
        Message::new(REQUEST_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Request, DeserializationErrors> {
        if message.req_type != REQUEST_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    message_id: String,
    topic: String
}

impl Reply {
    pub fn new(message_id: String, topic: String) -> Reply {
        Reply {
            message_id: message_id,
            topic: topic
        }
    }
}

impl NetworkTradable<Reply> for Reply {
    fn as_message(&self) -> Message {
        Message::new(REPLY_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Reply, DeserializationErrors> {
        if message.req_type != REPLY_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}
