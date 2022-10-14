use super::{NetworkTradable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

const REQUEST_HEADER: &str = "SUB";
const REPLY_HEADER: &str = "SUB_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    sub_id: String,
    endpoint: String,
    topic: String
}

impl Request {
    pub fn new(sub_id: String, endpoint: String, topic: String) -> Request {
        Request {
            sub_id: sub_id,
            endpoint: endpoint,
            topic: topic
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
    sub_id: String,
    topic: String
}

impl Reply {
    pub fn new(sub_id: String, topic: String) -> Reply {
        Reply {
            sub_id: sub_id,
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
