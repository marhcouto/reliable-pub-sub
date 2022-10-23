use super::{NetworkTradeable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

pub const REQUEST_HEADER: &str = "SUB";
pub const REPLY_HEADER: &str = "SUB_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub sub_id: String,
    pub topic: String
}

impl Request {
    pub fn new(sub_id: String, topic: String) -> Request {
        Request {
            sub_id: sub_id,
            topic: topic
        }
    }
}

impl NetworkTradeable<Request> for Request {
    fn as_message(&self) -> Message {
        Message::new(REQUEST_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Request, DeserializationErrors> {
        if message.msg_type != REQUEST_HEADER {
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
    pub sub_id: String,
    pub topic: String,
    pub broker_id: String,
    pub post_offset: u64
}

impl Reply {
    pub fn new(sub_id: String, topic: String, broker_id: String, post_offset: u64) -> Reply {
        Reply {
            sub_id: sub_id,
            topic: topic,
            broker_id: broker_id,
            post_offset: post_offset
        }
    }
}

impl NetworkTradeable<Reply> for Reply {
    fn as_message(&self) -> Message {
        Message::new(REPLY_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Reply, DeserializationErrors> {
        if message.msg_type != REPLY_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}
