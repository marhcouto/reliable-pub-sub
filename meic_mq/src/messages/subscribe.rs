use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    sub_id: String,
    endpoint: String,
    topic: String
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    sub_id: String,
    topic: String
}

impl Reply {
    pub fn new(sub_id: String, topic: String) -> Reply {
        Reply {
            sub_id: sub_id,
            topic: topic
        }
    }
}
