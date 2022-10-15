use serde::{Serialize, Deserialize};

use super::{ ContextIOError, SUB_STORAGE_PATH };

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberContext {
    #[serde(skip)]
    pub sub_id: String,
    pub topic: String,
    pub known_broker_id: String,
    pub next_message_id: u64
}

impl SubscriberContext {
    pub fn new(sub_id: String, topic: String, broker_id: String) -> SubscriberContext {
        SubscriberContext {
            sub_id: sub_id,
            topic: topic,
            known_broker_id: broker_id,
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
}