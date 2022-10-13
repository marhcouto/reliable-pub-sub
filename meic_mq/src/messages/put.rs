use serde::{Serialize, Deserialize};

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
