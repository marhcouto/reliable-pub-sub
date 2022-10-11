use super::{NetworkTradable, Message, SerializationError};

enum RequestDeserializationSteps {
    ExpectsPublisherId,
    ExpectsTopic,
    ExpectsMessageId,
    Done
}

#[derive(Debug)]
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

impl NetworkTradable<Request> for Request {
    fn serialize(&self) -> Message {
        let message_str: String = format!("PUT\n{pub_id}\n{topic}\n{message_id}\n",
            pub_id = self.pub_id,
            topic = self.topic,
            message_id = self.message_id
        );

        Message::new(message_str, Some(&self.payload))
    }

    fn deserialize(message: Message) -> Result<Request, SerializationError> {
        let mut message_header_lines = message.header.split_terminator('\n');
        let message_type = match message_header_lines.next() {
            Some(val) => val, 
            None => return Err(SerializationError::EmptyMessage)
        };

        if message_type != "PUT" {
            return Err(SerializationError::UnexpectedMessage);
        }

        let mut pub_id: String = String::new();
        let mut topic: String = String::new();
        let mut message_id: String = String::new();
        let mut reading_step = RequestDeserializationSteps::ExpectsPublisherId;
        for line in message_header_lines {
            reading_step = match reading_step {
                RequestDeserializationSteps::ExpectsPublisherId => {
                    pub_id = String::from(line);
                    RequestDeserializationSteps::ExpectsTopic
                }
                RequestDeserializationSteps::ExpectsTopic => {
                    topic = String::from(line);
                    RequestDeserializationSteps::ExpectsMessageId
                }
                RequestDeserializationSteps::ExpectsMessageId => {
                    message_id = String::from(line);
                    RequestDeserializationSteps::Done
                }
                RequestDeserializationSteps::Done => return Err(SerializationError::InvalidFormat),
            };
        }

        match reading_step {
            RequestDeserializationSteps::Done => { }
            _ => return Err(SerializationError::InvalidFormat)
        }

        Ok(Request::new(
            pub_id,
            topic,
            message_id,
            match message.payload {
                Some(val) => val.to_vec(),
                None => return Err(SerializationError::InvalidFormat)
            })
        )
    }
}


enum ReplyDeserializationSteps {
    ExpectsMessageId,
    ExpectsTopic,
    Done
}

#[derive(Debug)]
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

impl NetworkTradable<Reply> for Reply {
    fn serialize(&self) -> Message {
        let message_str: String = format!("PUT_REPL\n{message_id}\n{topic}\n",
            message_id = self.message_id,
            topic = self.topic
        );
        Message::new( 
            message_str,
            Option::None
        )
    }

    fn deserialize(message: Message) -> Result<Reply, SerializationError> {
        if let Some(_) = message.payload {
            return Err(SerializationError::InvalidFormat);
        }

        let mut message_header_lines = message.header.split_terminator('\n');
        let message_type = match message_header_lines.next() {
            Some(value) => value,
            None => return Err(SerializationError::EmptyMessage)
        };

        if message_type != "PUT_REPL" {
            return Err(SerializationError::UnexpectedMessage);
        }

        let mut reading_step = ReplyDeserializationSteps::ExpectsMessageId;
        let mut message_id = String::new();
        let mut topic = String::new();
        for line in message_header_lines {
            reading_step = match reading_step {
                ReplyDeserializationSteps::ExpectsMessageId => {
                    message_id = String::from(line);
                    ReplyDeserializationSteps::ExpectsTopic
                }
                ReplyDeserializationSteps::ExpectsTopic => {
                    topic = String::from(line);
                    ReplyDeserializationSteps::Done
                }
                ReplyDeserializationSteps::Done => return Err(SerializationError::InvalidFormat)
            }
        }

        match reading_step {
            ReplyDeserializationSteps::Done => {}
            _ => return Err(SerializationError::InvalidFormat)
        }

        Ok(Reply::new(message_id, topic))
    }
}
