use meic_mq::messages::Message;
use meic_mq::messages::get::{ REQUEST_HEADER as GET_REQ_HEAD, REPLY_HEADER as GET_REP_HEAD, ACK_HEADER as GET_ACK_HEAD, ACK_REPLY_HEADER as GET_ACK_REPL_HEAD, Request as GetRequest, Reply as GetReply, Ack as AckRequest, AckReply };
use meic_mq::messages::put::{ REQUEST_HEADER as PUT_REQ_HEAD, REPLY_HEADER as PUT_REP_HEAD, Request as PutRequest, Reply as PutReply };
use meic_mq::messages::subscribe::{ REQUEST_HEADER as SUB_REQ_HEAD, REPLY_HEADER as SUB_REP_HEAD, Request as SubRequest, Reply as SubReply };
use meic_mq::messages::unsubscribe::{ REQUEST_HEADER as UNSUB_REQ_HEAD, REPLY_HEADER as UNSUB_REP_HEAD, Request as UnsubRequest, Reply as UnsubReply };
use meic_mq::messages::error::{ REQUEST_HEADER as ERR_REQ_HEAD, BrokerErrorMessage, BrokerErrorType };

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::str;
use std::env;
use std::time;
use std::fs::File;
use serde_json;
use scheduled_thread_pool;
use lazy_static::lazy_static;
use lazy_static::__Deref;

const SUBS_FILE_PATH: &str = "subs.bson";
const TOPICS_FILE_PATH: &str = "topics.bson";

enum SubscriberStatus {
    WAITING_ACK,
    WAITING_GET
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscriberData {
    topic: String,
    status: SubscriberStatus,
    last_read_message: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct TopicData {
    #[serde(with = "serde_bytes")]
    messages: Vec<Vec<u8>>
    message_counter: u64
}

lazy_static!{
    static ref SUBS: Mutex<HashMap<String, SubscriberData>> = Mutex::new(HashMap::new());
    static ref TOPIC_QUEUE: Mutex<HashMap<String, TopicData>> = Mutex::new(HashMap::new());
    static ref RECEIVED_MESSAGES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP).unwrap();

    socket
        .bind("tcp://*:5555") //qual é a porta?
        .expect("failed binding socket");

    let _arguments: Vec<String> = env::args().collect();

    recover_state();

    let st_pool = scheduled_thread_pool::ScheduledThreadPool::new(1);

    st_pool.execute_at_fixed_rate(time::Duration::from_secs(5), time::Duration::from_secs(5), || {
        let subs_file: File = File::create(SUBS_FILE_PATH).unwrap();
        let subs_bytes = bson::to_vec(SUBS.lock().unwrap().deref()).unwrap();
        subs_file.write(subs_bytes);

        let topics_file: File = File::create(TOPICS_FILE_PATH).unwrap();
        let topics_bytes = bson::to_vec(TOPIC_QUEUE.lock().unwrap().deref()).unwrap();
        topics_file.write(topics_bytes);

        RECEIVED_MESSAGES.lock().unwrap().deref().drain();
    });

    loop {
        parse_request(&socket, rcv_msg(&socket));
    }
    
}


fn recover_state(){
    match File.open(SUBS_FILE_PATH) {
        Ok(file) => *TOPICS.lock().unwrap() = serde_json::from_reader(file.unwrap()).unwrap();
        Err(err) => eprintln!("Couldn't read Topics state! Creating a new one in the next backup...{}", err.to_string)
    }

    match File.open(TOPIC_QUEUE) {
        Ok(file) => *SUBS.lock().unwrap() = serde_json::from_reader(file.unwrap()).unwrap();
        Err(err) => eprintln!("Couldn't read Subscriber state! Creating a new one in the next backup...{}", err.to_string)
    }
}


fn rcv_msg(socket: &zmq::Socket) -> Message {
    let req_bytes: Vec<u8> = socket.recv_bytes(0);
    let req_message: Message = bson::from_slice(req_bytes.as_slice().unwrap());

    req_message
}

fn send_msg(socket: &zmq::Socket, message: Message) {

    socket.send(message, 0)

}

fn handle_get(socket: &zmq::Socket, request: GetRequest)  {
    
}

fn handle_put(socket: &zmq::Socket, request: PutRequest)  {
    
}

fn handle_sub(socket: &zmq::Socket, request: SubRequest)  {

    let mut subs_list = SUBS.lock().unwrap().deref();

    if subs_list.contains_key(client_id) {
        let error_reply: BrokerErrorMessage = BrokerErrorMessage::new(BrokerErrorType::SubscriberAlreadyRegistered, 
                                                "asd", "asda");
    } else {
        if topics_list.contains_key(topic) {
            let mut set = topics_list.get_mut(topic).unwrap();
            set.insert(String::from(client_id));
        }
        subs_list.insert(String::from(client_id), String::from(topic));
        queue.insert(client_id.to_string(),VecDeque::new());
    }
    send_msg(&socket, &client_id, "OK");
}

fn handle_unsub(socket: &zmq::Socket, request: UnsubRequest)  {
    
}

fn handle_get_ack(socket: &zmq::Socket, request: AckRequest)  {
    
}

fn handle_error_message(socket: &zmq::Socket, request: AckRequest) {
    
}

fn print_error(error_string: String) {
    println!("ERROR: {}", error_string);
}



fn handle_requests(socket: &zmq::Socket) {
    
    let req_bytes: Vec<u8> = socket.recv_bytes(0).unwrap();
    let req_message: Message = bson::from_slice(req_bytes.as_slice().unwrap());

    match req_message.req_type {
        GET_REQ_HEAD => handle_get(socket, bson::from_bson(req_msg.payload).unwrap()),
        GET_ACK_HEAD => handle_get_ack(socket, bson::from_bson(req_msg.payload).unwrap()),
        PUT_REQ_HEAD => handle_put(socket, bson::from_bson(req_msg.payload).unwrap()),
        SUB_REQ_HEAD => handle_sub(socket, bson::from_bson(req_msg.payload).unwrap()),
        UNSUB_REQ_HEAD => handle_unsub(socket, bson::from_bson(req_msg.payload).unwrap()),
        ERR_REQ_HEAD => panic!("TODO: Error"),
        _ => print_error(String::from("Unrecognized message header"))
    }

    
    // let mut subs_list = SUBS.lock().unwrap();
    // let mut topics_list = TOPICS.lock().unwrap();
    // let mut queue = QUEUE.lock().unwrap();

    // let split: Vec<_> = request.splitn(2," ").collect();
    
    // let request_type = split[0];

    // let start;
    // let mut end = 0; 
    // let mut topic = "";
    
    // if split.len() >= 2 {
    //     start = split[1].find("[").unwrap_or(0) + 1;
    //     end = split[1].find("]").unwrap_or(split[1].len());
    //     topic = &split[1][start..end];
    // }

    // match request_type {
    //     "SUB" => {
    //         if subs_list.contains_key(client_id){
    //             send_msg(&socket, &client_id, "AS"); //already subs- see erros structs
    //             }
    //         else{
    //             if topics_list.contains_key(topic) {
    //                 let mut set = topics_list.get_mut(topic).unwrap();
    //                 set.insert(String::from(client_id));
    //             }
    //             subs_list.insert(String::from(client_id), String::from(topic));
    //             queue.insert(client_id.to_string(),VecDeque::new());
    //         }
    //         send_msg(&socket, &client_id, "OK");
    //     },
    //     "UNSUB" => {
    //         if subs_list.contains_key(client_id){
    //             subs_list.remove(client_id);
    //             queue.remove(client_id);

    //             send_msg(&socket, &client_id, "OK");

    //             let set = topics_list.get_mut(topic).unwrap();
    //             set.remove(client_id);

    //         }
    //         else {
    //             send_msg(&socket, &client_id, "NOK");
    //         }
    //     },
    //     "GET" => {
    //         if subs_list.contains_key(client_id){
    //             let tp = subs_list.get_mut(client_id).unwrap();

    //             if topic == tp{
    //                 let first = queue.get_mut(client_id).unwrap().pop_front();
    //                 if first == None {
    //                     send_msg(&socket, &client_id, "SRY");
    //                 }
    //                 else{
    //                     let v = first.unwrap();
    //                     if send_msg(&socket, &client_id, format!("OK {}", v).as_str()) == -1 {
    //                         queue.get_mut(client_id).unwrap().push_front(v);
    //                     }
    //                 }
                    
    //             }
    //             else{
    //                 send_msg(&socket, &client_id, "NS");
    //             }

    //         }
    //         else {
    //             send_msg(&socket, &client_id, "NF");
    //         }

    //     },
    //     "PUT" => {
    //         if topics_list.contains_key(topic) {
    //             let set = topics_list.get_mut(topic).unwrap();
    //             let msg = &split[1][end +1..];

    //             for id in set.iter() {
    //                 let value = queue.get_mut(id).unwrap();
    //                 value.push_back(String::from(msg.trim()))
    //             }
    //         }

    //     },
    //     _ => {
    //         send_msg(&socket, &client_id, "NOK");
    //     },
    // }

}