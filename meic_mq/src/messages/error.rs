use serde::{Serialize, Deserialize};

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
