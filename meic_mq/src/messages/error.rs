use serde::{Serialize, Deserialize};

use super::{NetworkTradeable, Message, DeserializationErrors};

pub const REQUEST_HEADER: &str = "ERR";

#[derive(Debug, Serialize, Deserialize)]
pub enum BrokerErrorType {
    SubscriberNotRegistered,
    SubscriberAlreadyRegistered,
    DuplicateMessage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokerErrorMessage {
    pub error_type: BrokerErrorType,
    pub broker_id: String,
    pub description: String
}

impl BrokerErrorMessage {
    pub fn new(error_type: BrokerErrorType, broker_id: String, description: String) -> BrokerErrorMessage {
        BrokerErrorMessage {
            error_type: error_type,
            broker_id: broker_id,
            description: description
        }
    }
}

impl NetworkTradeable<BrokerErrorMessage> for BrokerErrorMessage {
    fn as_message(&self) -> Message {
        Message {
            req_type: REQUEST_HEADER.to_string(),
            payload: bson::to_bson(self).unwrap()
        }
    }

    fn from_message(message: Message) -> Result<BrokerErrorMessage, DeserializationErrors> {
        if message.req_type != REQUEST_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}
