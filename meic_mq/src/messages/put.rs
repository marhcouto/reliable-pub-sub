use super::{NetworkTradeable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub const REQUEST_HEADER: &str = "PUT";
pub const REPLY_HEADER: &str = "PUT_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub pub_id: String,
    pub topic: String,
    pub message_uuid: String,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>
}

impl Request {
    pub fn new(pub_id: String, topic: String, payload: Vec<u8>) -> Request {
        let uuid: Uuid = Uuid::new_v4();
        Request {
            pub_id: pub_id,
            topic: topic,
            message_uuid: uuid.to_string(),
            payload: payload
        }
    }
}

impl NetworkTradeable<Request> for Request {
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
    pub message_uuid: String,
    pub topic: String,
    pub broker_id: String
}

impl Reply {
    pub fn new(message_uuid: String, topic: String, broker_id: String) -> Reply {
        Reply {
            message_uuid: message_uuid,
            topic: topic,
            broker_id: broker_id
        }
    }

    pub fn match_request(&self, request: &Request) -> bool {
        if self.topic != request.topic {
            return false;
        }
        if self.message_uuid != request.message_uuid {
            return false;
        }

        return true;
    }
}

impl NetworkTradeable<Reply> for Reply {
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
