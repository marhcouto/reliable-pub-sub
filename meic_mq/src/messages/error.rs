use serde::{Serialize, Deserialize};

use super::{NetworkTradeable, Message, DeserializationErrors};

pub const REQUEST_HEADER: &str = "ERR";

#[derive(Debug, Serialize, Deserialize)]
pub enum BrokerErrorType {
    SubscriberNotRegistered,
    SubscriberAlreadyRegistered,
    InhexistantTopic,
    DuplicateMessage,
    TopicMismatch,
    AckMessageMismatch,
    UnknownMessage,
    NoPostsInTopic
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokerErrorMessage {
    pub error_type: BrokerErrorType,
    pub broker_id: String,
    pub description: String
}

impl BrokerErrorMessage {
    pub fn new(error_type: BrokerErrorType, broker_id: String) -> BrokerErrorMessage {
        match error_type {
            BrokerErrorType::SubscriberAlreadyRegistered => BrokerErrorMessage { error_type, broker_id,
                description: format!("You are already subscribed to a topic") },
            BrokerErrorType::SubscriberNotRegistered => BrokerErrorMessage { error_type, broker_id,
                description: format!("You are not subscribed to that topic") },
            BrokerErrorType::InhexistantTopic => BrokerErrorMessage { error_type, broker_id,
                description: format!("The topic mentioned in the request does not exist") },
            BrokerErrorType::DuplicateMessage => BrokerErrorMessage { error_type, broker_id,
                description: format!("The message you sent is a duplicate message") },
            BrokerErrorType::TopicMismatch => BrokerErrorMessage { error_type, broker_id,
                description: format!("The topic you are subscribed is different from the one you submitted") },
            BrokerErrorType::AckMessageMismatch => BrokerErrorMessage { error_type, broker_id,
                description: format!("The ack message id did not match with the last read post") },
            BrokerErrorType::UnknownMessage => BrokerErrorMessage {error_type, broker_id, 
                description: format!("Couldn't recognize the type of your message") },
            BrokerErrorType::NoPostsInTopic => BrokerErrorMessage {error_type, broker_id, 
                description: format!("There are still no posts in that topic") }
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
