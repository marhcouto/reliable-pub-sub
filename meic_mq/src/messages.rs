use bson::Bson;
use serde::{Serialize, Deserialize};

pub enum DeserializationErrors {
    IncompatibleMessageType,
    InvalidMessageStructure(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub msg_type: String,
    pub payload: Bson
}

impl Message {
    fn new(req_type: String, payload: Bson) -> Message {
        Message {
            msg_type: req_type,
            payload: payload
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bson::ser::Error> {
        bson::to_vec(self)
    }
}

pub trait NetworkTradeable<T> {
    fn as_message(&self) -> Message;
    fn from_message(message: Message) -> Result<T, DeserializationErrors>;
} 

pub mod put;
pub mod get;
pub mod subscribe;
pub mod error;
pub mod unsubscribe;
