use super::{NetworkTradable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

const REQUEST_HEADER: &str = "GET";
const REPLY_HEADER: &str = "GET_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    sub_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    message_no: u64,
    payload: Vec<u8>,
}

impl Request {
    pub fn new(sub_id: String) -> Request{
        Request {
            sub_id: sub_id,
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
