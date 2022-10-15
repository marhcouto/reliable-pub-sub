use bson::Bson;
use serde::{Serialize, Deserialize};

pub enum DeserializationErrors {
    IncompatibleMessageType,
    InvalidMessageStructure(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub req_type: String,
    pub payload: Bson
}

impl Message {
    fn new(req_type: String, payload: Bson) -> Message {
        Message {
            req_type: req_type,
            payload: payload
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bson::ser::Error> {
        bson::to_vec(self)
    }
}

pub trait NetworkTradable<T> {
    fn as_message(&self) -> Message;
    fn from_message(message: Message) -> Result<T, DeserializationErrors>;
} 

pub mod put;
pub mod get;
pub mod subscribe;
pub mod error;
