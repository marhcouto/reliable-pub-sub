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

lazy_static!{
    static ref TOPICS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
    static ref REQUESTS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
    static ref QUEUE: Mutex<HashMap<String, VecDeque<String>>> = Mutex::new(HashMap::new());
}

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::ROUTER).unwrap();
    socket.set_router_mandatory(true).unwrap(); 

    socket
        .bind("tcp://*:5555") //qual é a porta?
        .expect("failed binding socket");

    let arguments: Vec<String> = env::args().collect();

    if !(arguments.len() == 2 && (arguments[1] == "-r" || arguments[1] == "--reset-status")) {
        //TODO : recover state
    }

    let st_pool = scheduled_thread_pool::ScheduledThreadPool::new(1);

    st_pool.execute_at_fixed_rate(time::Duration::from_secs(5), time::Duration::from_secs(5), || {
        let topics_file: File = File::create("topics.json").unwrap();
        serde_json::to_writer(&topics_file, &*TOPICS).unwrap();

        let requests_file: File = File::create("requests.json").unwrap();
        serde_json::to_writer(&requests_file, &*REQUESTS).unwrap();

        let queue_file: File = File::create("queue.json").unwrap();
        serde_json::to_writer(&queue_file, &*QUEUE).unwrap();
    });

    loop {
        let (req_id, req) = rcv_msg(&socket);

        parse_request(&socket, &req_id, req);
    }
    
}


fn rcv_msg(socket: &zmq::Socket) -> (String, String){
    let res = socket.recv_multipart(0);
    let message_id;
    let message_bytes;

    match res {
        Ok(payload) => {
            match str::from_utf8(&payload[0]) { // String::from utf not useful, so str version instead
                Ok(string_msg) => message_id = String::from(string_msg),
                Err(conv_err) => {
                    println!("Message id convesion from utf8 to string error ({0})", conv_err);
                    message_id = "".to_string(); // when err, set empty string
                }
            };

            match str::from_utf8(&payload[1]) {
                Ok(string_msg) => message_bytes = String::from(string_msg),
                Err(conv_err) => {
                    println!("Message bytes convesion from utf8 to string error ({0})", conv_err);
                    message_bytes = "".to_string();
                }
            };
            
            return (message_id, message_bytes);
        },
        Err(err) => {
            println!("Error reading message ({0})", err);
            return ("".to_string(), "".to_string());
        }
    }
}

fn send_msg(socket: &zmq::Socket, message_id: &String, message_bytes: &str) -> i32{
    let res = socket.send_multipart([message_id.as_bytes(), message_bytes.as_bytes()], 0);

    match res {
        Ok(_) => return 0,
        Err(err) => {
            println!("Error sending message ({0})", err);
            return 1;
        }
    }
}



fn parse_request(socket: &zmq::Socket, request_id: &String, request: String) {
    let mut topic_list = TOPICS.lock().unwrap();
    let mut queue = QUEUE.lock().unwrap();

    let split: Vec<_> = request.splitn(2," ").collect();
    
    let request_type = split[0];

    let start;
    let mut end = 0; 
    let mut topic = "";
    
    if split.len() >= 2 {
        start = split[1].find("[").unwrap_or(0) + 1;
        end = split[1].find("]").unwrap_or(split[1].len());
        topic = &split[1][start..end];
    }

    match request_type {
        "SUB" => {
            if topic_list.contains_key(topic){
                let tmap = topic_list.get_mut(topic).unwrap(); 
                
                if !tmap.contains(request_id){
                    tmap.insert(String::from(request_id));
                    queue.insert(request_id.to_string(),VecDeque::new());
                }
            else{
                let mut tmap: HashSet<String> = HashSet::new();
                tmap.insert(String::from(request_id));
                topic_list.insert(topic.to_string(), tmap);
                queue.insert(request_id.to_string(),VecDeque::new());
            }
            send_msg(&socket, &request_id, "OK");
            }
        },
        "UNSUB" => {
            if topic_list.contains_key(topic){
                let tmap = topic_list.get_mut(topic).unwrap();
                
                if tmap.contains(request_id){
                    tmap.remove(&String::from(request_id));
                    queue.remove(request_id);

                    if tmap.len() == 0 {
                        topic_list.remove(topic);
                    }
                }


            send_msg(&socket, &request_id, "OK");

            }
        },
        "GET" => {
            if topic_list.contains_key(topic){
                let tmap = topic_list.get_mut(topic).unwrap();

                if tmap.contains(request_id){
                    let first = queue.get_mut(request_id).unwrap().pop_front();
                    if first == None {
                        add_pend_req(String::from(topic),String::from(request_id));
                    }
                    else{
                        let v = first.unwrap();
                        if send_msg(&socket, &request_id, format!("OK {}", v).as_str()) == -1 {
                            queue.get_mut(request_id).unwrap().push_front(v);
                        }
                    }
                    
                }
                else{
                    send_msg(&socket, &request_id, "NS");
                }

            }
            else {
                send_msg(&socket, &request_id, "NF");
            }

        },
        "PUT" => {
            if topic_list.contains_key(topic) {
                let set = topic_list.get_mut(topic).unwrap();
                let msg = &split[1][end +1..];

                for id in set.iter() {
                    let value = queue.get_mut(id).unwrap();
                    value.push_back(String::from(msg.trim()))
                }

                drop(topic_list);

                check_pend_req(String::from(topic), socket);
            }

        },
        _ => {
            send_msg(&socket, &request_id, "NOK");
        },
    }
}

fn add_pend_req(topic: String, request_id: String) {
    let mut req = REQUESTS.lock().unwrap();

    if req.contains_key(&topic) {
        req.get_mut(&topic).unwrap().insert(request_id);
    }
    else{
        let mut s : HashSet<String> = HashSet::new();
        s.insert(request_id);
        req.insert(topic,s);
    }
}

fn check_pend_req(topic: String, socket: &zmq::Socket) {
    let mut topics = TOPICS.lock().unwrap();
    let mut req = REQUESTS.lock().unwrap();
    let mut queue = QUEUE.lock().unwrap();

    if req.contains_key(&topic) {
        let pend_req = req.get_mut(&topic).unwrap();
        let tmap = topics.get_mut(&topic).unwrap();

        for id in pend_req.iter() {
            let v = queue.get_mut(id).unwrap().pop_front().unwrap();

            if send_msg(&socket, &id, format!("OK {}", v).as_str()) == -1 {
                queue.get_mut(id).unwrap().push_front(v);
            }
        }

        req.remove(&topic);
    }
}