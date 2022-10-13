use super::{NetworkTradable, Message, SerializationError};

pub struct Request {
    sub_id: String,
}

#[derive(Debug)]
pub struct Reply {
    message_no: u32,
    payload: Vec<u8>,
}

impl Request {
    pub fn new(sub_id: String) -> Request{
        Requset {
            sub_id: sub_id,
        }
    }
}
