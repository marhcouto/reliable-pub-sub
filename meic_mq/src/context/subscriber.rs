use serde::{Serialize, Deserialize};

use super::{ ContextIOError, FileWritable, read };
use super::super::messages::{ get, unsubscribe, subscribe };

const SUB_STORAGE_PATH: &str = "./data/sub/";

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

impl FileWritable<SubscriberContext> for SubscriberContext {
    fn build_path(&self) -> String {
        return format!("{}{}", SUB_STORAGE_PATH, self.sub_id);
    }

    fn from_file(id: &String) -> Result<SubscriberContext, ContextIOError> {
        read(format!("{}{}", Self::build_prefix(), id))
    }

    fn build_prefix() -> &'static str {
        return SUB_STORAGE_PATH;
    }
}

impl Drop for SubscriberContext {
    fn drop(&mut self) {
        if let Err(err) = super::save(self) {
            match err {
                ContextIOError::ErrorCreatingDirectory(err) => eprintln!("Couldn't create directory to save {} subscriber's context: {}", self.sub_id, err),
                ContextIOError::ErrorWritingToFile(err) => eprintln!("Couldn't write to {} subscriber's context: {}", self.sub_id, err),
                _ => eprintln!("Unexpected error while writing {} subscriber's state", self.sub_id)
            }
        }
    }
}


