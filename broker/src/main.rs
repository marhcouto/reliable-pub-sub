use meic_mq::messages::get::{ REQUEST_HEADER as GET_REQ_HEAD, ACK_HEADER as GET_ACK_HEAD, Request as GetRequest, Reply as GetReply, Ack as AckRequest, AckReply };
use meic_mq::messages::put::{ REQUEST_HEADER as PUT_REQ_HEAD, Request as PutRequest, Reply as PutReply };
use meic_mq::messages::subscribe::{ REQUEST_HEADER as SUB_REQ_HEAD, Request as SubRequest, Reply as SubReply };
use meic_mq::messages::unsubscribe::{ REQUEST_HEADER as UNSUB_REQ_HEAD, Request as UnsubRequest, Reply as UnsubReply };
use meic_mq::messages::error::{ BrokerErrorMessage, BrokerErrorType };
use meic_mq::messages::{Message, NetworkTradeable};

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::sync::Mutex;
use std::str;
use std::env;
use std::time;
use std::fs::File;
use scheduled_thread_pool;
use serde::{ Serialize, Deserialize };
use lazy_static::lazy_static;
use lazy_static::__Deref;

const SUBS_FILE_PATH: &str = "subs.bson";
const TOPICS_FILE_PATH: &str = "topics.bson";

#[derive(Debug, Serialize, Deserialize)]
enum SubscriberStatus {
    WaitingAck,
    WaitingGet
}


#[derive(Debug, Serialize, Deserialize)]
struct SubscriberData {
    topic: String,
    status: SubscriberStatus,
    last_read_post: u64
}

impl SubscriberData {
    fn new(topic: String, last_read_post: u64) -> SubscriberData {
        SubscriberData {
            topic,
            status: SubscriberStatus::WaitingGet,
            last_read_post
        }
    }

    fn increment_last_read(&mut self) -> u64 {
        self.last_read_post += 1;
        self.last_read_post
    }

    fn change_status(&mut self) {
        match self.status {
            SubscriberStatus::WaitingAck => self.status = SubscriberStatus::WaitingGet,
            SubscriberStatus::WaitingGet => self.status = SubscriberStatus::WaitingAck
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TopicData {
    posts: HashMap<String, Vec<u8>>,
    post_counter: u64
}

impl TopicData {
    fn new() -> TopicData {
        TopicData { posts: HashMap::new(), post_counter: 0 }
    }

    fn increment_counter(&mut self) -> u64 {
        self.post_counter += 1;
        self.post_counter
    }
}

lazy_static!{
    static ref SUBS: Mutex<HashMap<String, SubscriberData>> = Mutex::new(HashMap::new());
    static ref TOPICS: Mutex<HashMap<String, TopicData>> = Mutex::new(HashMap::new());
    static ref RECEIVED_UUIDS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref BROKER_UUID: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP).unwrap();

    socket
        .bind("tcp://*:5555") //qual e a porta?
        .expect("failed binding socket");

    let _arguments: Vec<String> = env::args().collect();

    recover_state();

    let st_pool = scheduled_thread_pool::ScheduledThreadPool::new(1);

    st_pool.execute_at_fixed_rate(time::Duration::from_secs(5), time::Duration::from_secs(5), || {
        let mut subs_file: File = File::create(SUBS_FILE_PATH).unwrap();
        let subs_bytes = bson::to_vec(SUBS.lock().unwrap().deref()).unwrap();
        if let Err(err) = subs_file.write(subs_bytes.as_slice()) {
            eprintln!("Couldn't write subs backup file: {}", err.to_string());
        }

        let mut topics_file: File = File::create(TOPICS_FILE_PATH).unwrap();
        let topics_bytes = bson::to_vec(TOPICS.lock().unwrap().deref()).unwrap();
        if let Err(err) = topics_file.write(topics_bytes.as_slice()) {
            eprintln!("Couldn't write topic backup file: {}", err.to_string());   
        }

    });

    loop {
        handle_requests(&socket);
    }

}

fn recover_state(){
    match File::open(TOPICS_FILE_PATH) {
        Ok(file) => *TOPICS.lock().unwrap() = bson::from_reader(file).unwrap(),
        Err(err) => println!("Couldn't read Topics state! Creating a new one in the next backup...{}", err.to_string())
    }

    match File::open(SUBS_FILE_PATH) {
        Ok(file) => *SUBS.lock().unwrap() = bson::from_reader(file).unwrap(),
        Err(err) => println!("Couldn't read Subscriber state! Creating a new one in the next backup...{}", err.to_string())
    }
}

fn handle_get(request: GetRequest) -> Message {
    let subs = &mut *SUBS.lock().unwrap();
    let topics = &mut *TOPICS.lock().unwrap();
    let post_payload: Vec<u8>;
    let post_no: u64;


    if !subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, String::from("TODO: brokerid")).as_message();
    }

    {
        let subscriber_data = subs.get_mut(&request.sub_id).unwrap(); 
        if subscriber_data.topic != request.topic {
            return BrokerErrorMessage::new(BrokerErrorType::TopicMismatch, String::from("TODO: brokerid")).as_message();
        }
        if !topics.contains_key(&request.topic) {
            return BrokerErrorMessage::new(BrokerErrorType::InhexistantTopic, String::from("TODO: brokerid")).as_message();
        }

        post_no = subscriber_data.last_read_post + 1;
        match topics.get(&request.topic).unwrap().posts.get(&post_no.to_string()) {
            Some(payload) => post_payload = payload.clone(),
            None => return BrokerErrorMessage::new(BrokerErrorType::NoPostsInTopic, String::from("TODO: brokerid")).as_message()
        }
    
        // Change status to waiting for ACK
        subscriber_data.change_status();
    }

    // Remove already read messages
    let mut min_last_read_post: u64 = u64::MAX;
    let posts = &mut topics.get_mut(&request.topic).unwrap().posts; 
    for (_, sub_data) in subs {
        if sub_data.topic == request.topic && sub_data.last_read_post < min_last_read_post {
            min_last_read_post = sub_data.last_read_post;
        }
    }
    let mut keys_to_remove: HashSet<String> = HashSet::new();
    for (post_no, _) in posts.iter() {
        if post_no.parse::<u64>().unwrap() <= min_last_read_post {
            keys_to_remove.insert(post_no.clone());
        }
    }
    posts.retain(|k, _| !keys_to_remove.contains(k));

    GetReply::new(request.sub_id.clone(), post_no, 
            String::from("TODO: brokerid"), post_payload).as_message()
}

fn handle_put(request: PutRequest) -> Message {
    let topics = &mut *TOPICS.lock().unwrap();
    let received_uuids = &mut *RECEIVED_UUIDS.lock().unwrap();

    if received_uuids.contains(&request.message_uuid) {
        return BrokerErrorMessage::new(BrokerErrorType::DuplicateMessage, String::from("TODO: brokerid")).as_message();
    }
    if !topics.contains_key(&request.topic)  {
        return BrokerErrorMessage::new(BrokerErrorType::InhexistantTopic, String::from("TODO: brokerid")).as_message();
    }
    let new_counter_value = topics.get_mut(&request.topic).unwrap().increment_counter();
    topics.get_mut(&request.topic).unwrap().posts.insert(new_counter_value.to_string(), request.payload);
    received_uuids.insert(request.message_uuid.clone());

    PutReply::new(request.message_uuid.clone(), request.topic.clone(), String::from("TODO: brokerid")).as_message()
}

fn handle_sub(request: SubRequest) -> Message {

    let subs = &mut *SUBS.lock().unwrap();
    let topics = &mut *TOPICS.lock().unwrap();

    // Subscriber already subscribed
    if subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberAlreadyRegistered, String::from("TODO: brokerid")).as_message();
    } 

    let subs_last_read_post_no;
    if !topics.contains_key(&request.topic) {
        topics.insert(request.topic.clone(), TopicData::new());
        subs_last_read_post_no = 0;
    } else {
        subs_last_read_post_no = topics.get(&request.topic).unwrap().post_counter;
    }
    subs.insert(request.sub_id.clone(), SubscriberData::new(request.topic.clone(), subs_last_read_post_no));
    
    SubReply::new(request.sub_id.clone(), request.topic.clone(), String::from("TODO: brokerid")).as_message()
}

fn handle_unsub(request: UnsubRequest) -> Message {
    let subs = &mut *SUBS.lock().unwrap();

    // Subscriber not subscribed
    if !subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, String::from("TODO: brokerid")).as_message();
    }
    subs.remove(&request.sub_id);
    
    UnsubReply::new(request.sub_id.clone(), String::from("TODO: brokerid")).as_message()
}

fn handle_get_ack(request: AckRequest) -> Message {
    let subs = &mut *SUBS.lock().unwrap();

    if !subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, String::from("TODO: brokerid")).as_message();
    }
    let subscriber_data: &mut SubscriberData = subs.get_mut(&request.sub_id).unwrap(); 

    if (subscriber_data.last_read_post + 1) != request.message_no {
        return BrokerErrorMessage::new(BrokerErrorType::AckMessageMismatch, String::from("TODO: brokerid")).as_message();
    }

    subscriber_data.increment_last_read();
    subscriber_data.change_status();

    AckReply::new(request.sub_id.clone(), request.message_no.clone()).as_message()
}

fn handle_requests(socket: &zmq::Socket) {
    
    let req_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let req_message: Message = bson::from_slice(req_bytes.as_slice()).unwrap();

    let rep_message = match req_message.req_type.as_str() {
        GET_REQ_HEAD => handle_get(bson::from_bson(req_message.payload).unwrap()),
        GET_ACK_HEAD => handle_get_ack(bson::from_bson(req_message.payload).unwrap()),
        PUT_REQ_HEAD => handle_put(bson::from_bson(req_message.payload).unwrap()),
        SUB_REQ_HEAD => handle_sub(bson::from_bson(req_message.payload).unwrap()),
        UNSUB_REQ_HEAD => handle_unsub(bson::from_bson(req_message.payload).unwrap()),
        _ => BrokerErrorMessage::new(BrokerErrorType::UnknownMessage, String::from("TODO: brokerid")).as_message()
    };

    socket.send(rep_message.to_bytes().unwrap(), 0).unwrap();
}