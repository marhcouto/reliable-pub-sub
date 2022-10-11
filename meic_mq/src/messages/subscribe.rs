use super::{Message, NetworkTradable, SerializationError};

pub struct Request {
    sub_id: String,
    endpoint: String,
    topic: String
}


enum RequestDeserializationSteps {
    ExpectsSubscriberId,
    ExpectsEndpoint,
    ExpectsTopic,
    Done
}

impl Request {
    pub fn new(sub_id: String, endpoint: String, topic: String) -> Request {
        Request {
            sub_id: sub_id,
            endpoint: endpoint,
            topic: topic
        }
    }
}

impl NetworkTradable<Request> for Request {
    fn serialize(&self) -> Message {
        let header_str: String = format!("SUB\n{sub_id}\n{endpoint}\n{topic}\n", 
            sub_id=self.sub_id,
            endpoint=self.endpoint,
            topic=self.topic
        );
        Message::new(header_str, None)
    }

    fn deserialize(message: Message) -> Result<Request, SerializationError> {
        if let Some(_) = message.payload {
            return Err(SerializationError::InvalidFormat);
        }

        let mut message_header_lines = message.header.split_terminator('\n');
        let message_type = match message_header_lines.next() {
            Some(val) => val,
            None => return Err(SerializationError::EmptyMessage)
        };

        if message_type != "SUB" {
            return Err(SerializationError::UnexpectedMessage);
        }
        
        let mut reading_step = RequestDeserializationSteps::ExpectsSubscriberId;
        let mut subscriber_id = String::new();
        let mut endpoint = String::new();
        let mut topic = String::new();
        for line in message_header_lines {
            let line_str = String::from(line);
            reading_step = match reading_step {
                RequestDeserializationSteps::ExpectsSubscriberId => {
                    subscriber_id = line_str;
                    RequestDeserializationSteps::ExpectsEndpoint
                }
                RequestDeserializationSteps::ExpectsEndpoint => {
                    endpoint = line_str;
                    RequestDeserializationSteps::ExpectsTopic
                }
                RequestDeserializationSteps::ExpectsTopic => {
                    topic = line_str;
                    RequestDeserializationSteps::Done
                }
                RequestDeserializationSteps::Done => return Err(SerializationError::InvalidFormat)
            }
        }

        match reading_step {
            RequestDeserializationSteps::Done => { }
            _ => return Err(SerializationError::InvalidFormat)
        }

        Ok(Request::new(subscriber_id, endpoint, topic))
    }
}

pub struct Reply {
    sub_id: String,
    topic: String
}

enum ReplyDeserializationSteps {
    ExpectsSubscriberId,
    ExpectsTopic,
    Done
}

impl Reply {
    pub fn new(sub_id: String, topic: String) -> Reply {
        Reply {
            sub_id: sub_id,
            topic: topic
        }
    }
}

impl NetworkTradable<Reply> for Reply {
    fn serialize(&self) -> Message {
        let header_str: String = format!("SUB_REPL\n{sub_id}\n{topic}\n",
            sub_id=self.sub_id,
            topic=self.topic
        );

        Message::new(header_str, None)
    }

    fn deserialize(message: Message) -> Result<Reply, SerializationError> {
        if let Some(_) = message.payload {
            return Err(SerializationError::InvalidFormat);
        }

        let mut message_header_lines = message.header.split_terminator('\n');
        let message_type = match message_header_lines.next() {
            Some(val) => val,
            None => return Err(SerializationError::EmptyMessage)
        };

        if message_type != "SUB_REPL" {
            return Err(SerializationError::UnexpectedMessage);
        }

        let mut reading_step = ReplyDeserializationSteps::ExpectsSubscriberId;
        let mut subscriber_id = String::new();
        let mut topic = String::new();
        for line in message_header_lines {
            let line_str = String::from(line);
            reading_step = match reading_step {
                ReplyDeserializationSteps::ExpectsSubscriberId => {
                    subscriber_id = line_str;
                    ReplyDeserializationSteps::ExpectsTopic
                }
                ReplyDeserializationSteps::ExpectsTopic => {
                    topic = line_str;
                    ReplyDeserializationSteps::Done
                }
                ReplyDeserializationSteps::Done => return Err(SerializationError::InvalidFormat)
            }
        }

        match reading_step {
            ReplyDeserializationSteps::Done => { }
            _ => return Err(SerializationError::InvalidFormat)
        }

        Ok(Reply::new(subscriber_id, topic))
    }
}
