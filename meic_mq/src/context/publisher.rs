use super::{ PUB_STORAGE_PATH, FileWritable, ContextIOError };
use super::super::messages::put;

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

use super::{ FileWritable, ContextIOError, read };

use super::super::messages::put;

const PUB_STORAGE_PATH: &str = "./data/pub/";

#[derive(Serialize, Deserialize)]
pub struct PublisherContext {
    #[serde(skip)]
    pub pub_id: String,
    pub known_broker_id: Option<String>,
    pub published_messages: HashMap<String, HashSet<String>>
}

impl PublisherContext {
    pub fn new(pub_id: String) -> PublisherContext {
        let publisher_path = format!("{}{}", PUB_STORAGE_PATH, pub_id);
        match super::read(publisher_path) {
            Ok(data) => data,
            Err(_) => PublisherContext {
                pub_id,
                known_broker_id: None,
                published_messages: HashMap::new()
            }
        }
    }

    pub fn is_message_new(&self, topic: &String, message_id: &String) -> bool {
        match self.published_messages.get(topic) {
            None => true,
            Some(set) => !set.contains(message_id)
        }
    }

    pub fn reset_context(&mut self) {
        self.known_broker_id = None;
        self.published_messages = HashMap::new()
    }

    pub fn read(pub_id: String) -> Result<PublisherContext, ContextIOError> {
        let path: String = format!("{}{}", PUB_STORAGE_PATH, pub_id);
        let mut pub_ctx: PublisherContext = match super::read(path) {
            Ok(val) => val,
            Err(err) => return Err(err)
        };
        pub_ctx.pub_id = pub_id;
        Ok(pub_ctx)
    }

    pub fn create_put_request(&self, topic: String, payload: Vec<u8>) -> put::Request {
        put::Request::new(self.pub_id.clone(), topic, payload)
    }
}

impl FileWritable<PublisherContext> for PublisherContext {
    fn build_path(&self) -> String {
        return format!("{}{}", PUB_STORAGE_PATH, self.pub_id);
    }

    fn from_file(id: &String) -> Result<PublisherContext, ContextIOError> {
        read(format!("{}{}", Self::build_prefix(), id))
    }

    fn build_prefix() -> &'static str {
        return PUB_STORAGE_PATH;
    }
}

impl Drop for PublisherContext {
    fn drop(&mut self) {
        if let Err(err) = super::save(self) {
            match err {
                ContextIOError::ErrorCreatingDirectory(err) => eprintln!("Couldn't create directory to save {} publisher's context: {}", self.pub_id, err),
                ContextIOError::ErrorWritingToFile(err) => eprintln!("Couldn't write to {} publisher's context: {}", self.pub_id, err),
                _ => eprintln!("Unexpected error while writing {} publisher's state", self.pub_id)
            }
        }
    }
}
