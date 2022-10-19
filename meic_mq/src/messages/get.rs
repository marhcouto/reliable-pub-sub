use super::{NetworkTradeable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

pub const REQUEST_HEADER: &str = "GET";
pub const REPLY_HEADER: &str = "GET_REPL";
pub const ACK_HEADER: &str = "GET_ACK";
pub const ACK_REPLY_HEADER: &str = "GET_ACK_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub sub_id: String,
    pub topic: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    pub sub_id: String,
    pub message_no: u64,
    pub broker_id: String,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ack {
    pub sub_id: String,
    pub message_no: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AckReply {
    pub sub_id: String,
    pub message_no: u64
}

impl Request {
    pub fn new(sub_id: String, topic: String) -> Request {
        Request {
            sub_id,
            topic
        }
    }
}

impl Reply {
    pub fn new(sub_id: String, message_no: u64, broker_id: String, payload: Vec<u8>) -> Reply {
        Reply {
            sub_id,
            message_no,
            broker_id,
            payload
        }
    }

    pub fn match_request(&self, request: &Request) -> bool {
        self.sub_id == request.sub_id
    }
}

impl Ack {
    pub fn new(sub_id: String, message_no: u64) -> Ack {
        Ack {
            sub_id,
            message_no
        }
    }
}

impl AckReply {
    pub fn new(sub_id: String, message_no: u64) -> AckReply {
        AckReply {
            sub_id,
            message_no
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

impl NetworkTradeable<Ack> for Ack {
    fn as_message(&self) -> Message {
        Message::new(REQUEST_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Ack, DeserializationErrors> {
        if message.req_type != ACK_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}

impl NetworkTradeable<AckReply> for AckReply {
    fn as_message(&self) -> Message {
        Message::new(REPLY_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<AckReply, DeserializationErrors> {
        if message.req_type != ACK_REPLY_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}
