use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

use super::{ PUB_STORAGE_PATH, FileWritable, ContextIOError };

#[derive(Serialize, Deserialize)]
pub struct PublisherContext {
    #[serde(skip)]
    pub_id: String,
    pub known_broker_id: Option<String>,
    published_messages: HashMap<String, HashSet<String>>
}

impl PublisherContext {
    pub fn new(pub_id: String) -> PublisherContext {
        PublisherContext {
            pub_id,
            known_broker_id: None,
            published_messages: HashMap::new() 
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
}

impl FileWritable for PublisherContext {
    fn build_path(&self) -> String {
        return format!("{}{}", PUB_STORAGE_PATH, self.pub_id);
    }
}
