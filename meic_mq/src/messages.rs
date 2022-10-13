use bson::Bson;

#[derive(Debug)]
pub struct Message<'a> {
    header: Bson,
    payload: Option<&'a Vec<u8>>
}

impl Message<'_> {
    pub fn new<'a>(header: Bson, payload: Option<&'a Vec<u8>> ) -> Message {
        Message {
            header: header,
            payload: payload
        }
    }
    
    pub fn header(&self) -> &Bson {
        &self.header
    }

    pub fn payload(&self) -> Option<&Vec<u8>> {
        self.payload
    }
}

pub mod put;
pub mod subscribe;
