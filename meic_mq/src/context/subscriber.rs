use serde::{Serialize, Deserialize};

use super::{ ContextIOError, SUB_STORAGE_PATH };
use super::super::messages::{ get, unsubscribe, subscribe };

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberContext {
    #[serde(skip)]
    pub sub_id: String,
    pub topic: String,
    pub known_broker_id: Option<String>,
    pub next_message_id: u64
}

impl SubscriberContext {
    pub fn new(sub_id: String, topic: String) -> SubscriberContext {
        SubscriberContext {
            sub_id,
            topic,
            known_broker_id: None,
            next_message_id: 0
        }
    }

    pub fn read(sub_id: String) -> Result<SubscriberContext, ContextIOError> {
        let path: String = format!("{}{}", SUB_STORAGE_PATH, sub_id);
        let mut sub_ctx: SubscriberContext = match super::read(path) {
            Ok(val) => val,
            Err(err) => return Err(err)
        };
        sub_ctx.sub_id = sub_id;
        Ok(sub_ctx)
    }

    pub fn create_get_request(&self) -> get::Request {
        get::Request::new(self.sub_id.clone(), self.topic.clone())
    }

    pub fn create_subscribe_request(&self) -> subscribe::Request {
        subscribe::Request::new(self.sub_id.clone(), self.topic.clone())
    }

    pub fn create_unsubscribe_request(&self) -> unsubscribe::Request {
        unsubscribe::Request::new(self.sub_id.clone())
    }
}