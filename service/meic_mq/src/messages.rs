#[derive(Debug)]
pub enum SerializationError {
    InvalidFormat,
    UnexpectedMessage,
    EmptyMessage
}

pub trait NetworkTradable<T> {
    fn serialize(&self) -> Message;
    fn deserialize(message: Message) -> Result<T, SerializationError>;
}

#[derive(Debug)]
pub struct Message<'a> {
    header: String,
    payload: Option<&'a Vec<u8>>
}

impl Message<'_> {
    pub fn new<'a>(header: String, payload: Option<&'a Vec<u8>> ) -> Message {
        Message {
            header: header,
            payload: payload
        }
    }
    
    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn payload(&self) -> Option<&Vec<u8>> {
        self.payload
    }
}

pub mod put;
pub mod subscribe;
