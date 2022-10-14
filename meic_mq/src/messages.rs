use bson::Bson;
use serde::{Serialize, Deserialize};

pub enum DeserializationErrors {
    IncompatibleMessageType,
    InvalidMessageStructure(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    req_type: String,
    payload: Bson
}

impl Message {
    fn new(req_type: String, payload: Bson) -> Message {
        Message {
            req_type: req_type,
            payload: payload
        }
    }
}

pub trait NetworkTradable<T> {
    fn as_message(&self) -> Message;
    fn from_message(message: Message) -> Result<T, DeserializationErrors>;
} 

pub mod put;
pub mod get;
pub mod subscribe;
