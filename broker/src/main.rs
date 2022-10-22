use meic_mq::messages::get::{ REQUEST_HEADER as GET_REQ_HEAD, ACK_HEADER as GET_ACK_HEAD, Request as GetRequest, Reply as GetReply, Ack as AckRequest, AckReply };
use meic_mq::messages::put::{ REQUEST_HEADER as PUT_REQ_HEAD, Request as PutRequest, Reply as PutReply };
use meic_mq::messages::subscribe::{ REQUEST_HEADER as SUB_REQ_HEAD, Request as SubRequest, Reply as SubReply };
use meic_mq::messages::unsubscribe::{ REQUEST_HEADER as UNSUB_REQ_HEAD, Request as UnsubRequest, Reply as UnsubReply };
use meic_mq::messages::error::{ BrokerErrorMessage, BrokerErrorType };
use meic_mq::messages::{Message, NetworkTradeable};
use uuid::Uuid;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{ Write, BufReader };
use std::env;
use std::fs::File;
use serde::{ Serialize, Deserialize };

const STATE_FILE_PATH: &str = "state.bson";

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

#[derive(Debug, Serialize, Deserialize)]
struct BrokerState {
    broker_uuid: String,
    subs: HashMap<String, SubscriberData>,
    topics: HashMap<String, TopicData>,
    received_uuids: HashSet<String>,
}

impl BrokerState {
    pub fn new() -> BrokerState {
        BrokerState {
            broker_uuid: Uuid::new_v4().to_string(),
            subs: HashMap::new(),
            topics: HashMap::new(),
            received_uuids: HashSet::new()
        }
    }
}

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP).unwrap();

    socket
        .bind("tcp://*:5555") //qual e a porta?
        .expect("failed binding socket");

    let _arguments: Vec<String> = env::args().collect();

    let mut state = recover_state();

    loop {
        handle_requests(&socket, &mut state);
        save_state(&state);
    }

}

fn save_state(state: &BrokerState) {
    let mut state_file: File = File::create(STATE_FILE_PATH).unwrap();
    let state_bytes = bson::to_vec(state).unwrap();
    if let Err(err) = state_file.write_all(state_bytes.as_slice()) {
        eprintln!("Couldn't write subs backup file: {}", err.to_string());
    }
}

fn recover_state() -> BrokerState {
    match File::open(STATE_FILE_PATH) {
        Ok(file) => {
            let state_buffer = BufReader::new(file);
            bson::from_reader(state_buffer).unwrap()
        },
        Err(_) => BrokerState::new()
    }
}

fn handle_get(state: &mut BrokerState, request: GetRequest) -> Message {
    let post_payload: Vec<u8>;
    let post_no: u64;

    if !state.subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, state.broker_uuid.clone()).as_message();
    }

    {
        let subscriber_data = state.subs.get_mut(&request.sub_id).unwrap(); 
        if subscriber_data.topic != request.topic {
            return BrokerErrorMessage::new(BrokerErrorType::TopicMismatch, state.broker_uuid.clone()).as_message();
        }
        if !state.topics.contains_key(&request.topic) {
            return BrokerErrorMessage::new(BrokerErrorType::InhexistantTopic, state.broker_uuid.clone()).as_message();
        }

        post_no = subscriber_data.last_read_post + 1;
        match state.topics.get(&request.topic).unwrap().posts.get(&post_no.to_string()) {
            Some(payload) => post_payload = payload.clone(),
            None => return BrokerErrorMessage::new(BrokerErrorType::NoPostsInTopic, state.broker_uuid.clone()).as_message()
        }
    
        // Change status to waiting for ACK
        subscriber_data.change_status();
    }

    GetReply::new(
        request.sub_id.clone(), 
        post_no, 
        state.broker_uuid.clone(),
        post_payload
    ).as_message()
}

fn handle_put(state: &mut BrokerState, request: PutRequest) -> Message {
    if state.received_uuids.contains(&request.message_uuid) {
        return BrokerErrorMessage::new(BrokerErrorType::DuplicateMessage, state.broker_uuid.clone()).as_message();
    }
    if !state.topics.contains_key(&request.topic)  {
        return BrokerErrorMessage::new(BrokerErrorType::InhexistantTopic, state.broker_uuid.clone()).as_message();
    }
    let new_counter_value = state.topics.get_mut(&request.topic).unwrap().increment_counter();
    state.topics.get_mut(&request.topic).unwrap().posts.insert(new_counter_value.to_string(), request.payload);
    state.received_uuids.insert(request.message_uuid.clone());
    println!("Topics data structure after put: {:?}", state.topics.get(&request.topic).unwrap().posts.keys());

    PutReply::new(request.message_uuid.clone(), request.topic.clone(), state.broker_uuid.clone()).as_message()
}

fn handle_sub(state: &mut BrokerState, request: SubRequest) -> Message {
    // Subscriber already subscribed
    if state.subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberAlreadyRegistered, state.broker_uuid.clone()).as_message();
    } 

    let subs_last_read_post_no;
    if !state.topics.contains_key(&request.topic) {
        state.topics.insert(request.topic.clone(), TopicData::new());
        subs_last_read_post_no = 0;
    } else {
        subs_last_read_post_no = state.topics.get(&request.topic).unwrap().post_counter;
    }
    state.subs.insert(request.sub_id.clone(), SubscriberData::new(request.topic.clone(), subs_last_read_post_no));

    SubReply::new(request.sub_id.clone(), request.topic.clone(), state.broker_uuid.clone(), subs_last_read_post_no + 1).as_message()
}

fn handle_unsub(state: &mut BrokerState, request: UnsubRequest) -> Message {
    // Subscriber not subscribed
    if !state.subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, state.broker_uuid.clone()).as_message();
    }
    state.subs.remove(&request.sub_id);
    
    UnsubReply::new(request.sub_id.clone(), state.broker_uuid.clone()).as_message()
}

fn handle_get_ack(state: &mut BrokerState, request: AckRequest) -> Message {
    if !state.subs.contains_key(&request.sub_id) {
        return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, state.broker_uuid.clone()).as_message();
    }
    let subscriber_data: &mut SubscriberData = state.subs.get_mut(&request.sub_id).unwrap(); 

    if (subscriber_data.last_read_post + 1) != request.message_no {
        return BrokerErrorMessage::new(BrokerErrorType::AckMessageMismatch, state.broker_uuid.clone()).as_message();
    }

    subscriber_data.increment_last_read();
    subscriber_data.change_status();

    let sub_topic = match state.subs.get(&request.sub_id) {
        Some(val) => val.topic.clone(),
        None => return BrokerErrorMessage::new(BrokerErrorType::SubscriberNotRegistered, state.broker_uuid.clone()).as_message()
    };

    // Remove already read messages
    let mut min_last_read_post: u64 = u64::MAX;
    let posts = &mut state.topics.get_mut(&sub_topic).unwrap().posts; 
    for (_, sub_data) in state.subs.iter() {
        if sub_data.topic == sub_topic && sub_data.last_read_post < min_last_read_post {
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
    println!("Topics data structure after acknowledged get: {:?}", posts.keys());

    AckReply::new(request.sub_id.clone(), request.message_no.clone()).as_message()
}

fn handle_requests(socket: &zmq::Socket, state: &mut BrokerState) {
    
    let req_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let req_message: Message = bson::from_slice(req_bytes.as_slice()).unwrap();

    println!("Received new request: {}", req_message.req_type);
    let rep_message = match req_message.req_type.as_str() {
        GET_REQ_HEAD => handle_get(state, bson::from_bson(req_message.payload).unwrap()),
        GET_ACK_HEAD => handle_get_ack(state, bson::from_bson(req_message.payload).unwrap()),
        PUT_REQ_HEAD => handle_put(state, bson::from_bson(req_message.payload).unwrap()),
        SUB_REQ_HEAD => {
            let message = handle_sub(state, bson::from_bson(req_message.payload).unwrap());
            println!("Known subs: {:?}", state.subs.keys());
            message
        }
        UNSUB_REQ_HEAD => {
            let message = handle_unsub(state, bson::from_bson(req_message.payload).unwrap());
            println!("Known subs: {:?}", state.subs.keys());
            message
        }
        _ => BrokerErrorMessage::new(BrokerErrorType::UnknownMessage, state.broker_uuid.clone()).as_message()
    };

    socket.send(rep_message.to_bytes().unwrap(), 0).unwrap();
}