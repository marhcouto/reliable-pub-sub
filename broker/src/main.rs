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
    static ref SUBS: Mutex<HashMap<String, HashMap<String, u64>>> = Mutex::new(HashMap::new());
    static ref TOPICS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
    static ref QUEUE: Mutex<HashMap<String, VecDeque<String>>> = Mutex::new(HashMap::new());
}

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::ROUTER).unwrap();
    socket.set_router_mandatory(true).unwrap(); 

    socket
        .bind("tcp://*:5555") //qual é a porta?
        .expect("failed binding socket");

    let _arguments: Vec<String> = env::args().collect();

    recover_state();

    let st_pool = scheduled_thread_pool::ScheduledThreadPool::new(1);

    st_pool.execute_at_fixed_rate(time::Duration::from_secs(5), time::Duration::from_secs(5), || {
        let subs_file: File = File::create("subs.json").unwrap();
        serde_json::to_writer(&subs_file, &*SUBS).unwrap();

        let topics_file: File = File::create("topics.json").unwrap();
        serde_json::to_writer(&topics_file, &*TOPICS).unwrap();

        let queue_file: File = File::create("queue.json").unwrap();
        serde_json::to_writer(&queue_file, &*QUEUE).unwrap();
    });

    loop {
        let (req_id, req) = rcv_msg(&socket);

        parse_request(&socket, &req_id, req);
    }
    
}


fn recover_state(){

    let file = File::open("topics.json");

    if file.is_ok() {
        *TOPICS.lock().unwrap() = serde_json::from_reader(file.unwrap()).unwrap();
    } else {
        println!("Topics state not found! Creating a new one...")
    }

    let file = File::open("queue.json");

    if file.is_ok() {
        *QUEUE.lock().unwrap() = serde_json::from_reader(file.unwrap()).unwrap();
    } else {
        println!("Queue state not found! Creating a new one...")
    }

    let file = File::open("subs.json");

    if file.is_ok() {
        *SUBS.lock().unwrap() = serde_json::from_reader(file.unwrap()).unwrap();
    } else {
        println!("Subsvcribers state not found! Creating a new one...")
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



fn parse_request(socket: &zmq::Socket, client_id: &String, request: String) {
    let mut subs_list = SUBS.lock().unwrap();
    let mut topics_list = TOPICS.lock().unwrap();
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
            if subs_list.contains_key(client_id){
                send_msg(&socket, &client_id, "AS"); //already subs- see erros structs
                }
            else{
                if topics_list.contains_key(topic) {
                    let mut set = topics_list.get_mut(topic).unwrap();
                    set.insert(String::from(client_id));
                }
                subs_list.insert(String::from(client_id), String::from(topic));
                queue.insert(client_id.to_string(),VecDeque::new());
            }
            send_msg(&socket, &client_id, "OK");
        },
        "UNSUB" => {
            if subs_list.contains_key(client_id){
                subs_list.remove(client_id);
                queue.remove(client_id);

                send_msg(&socket, &client_id, "OK");

                let set = topics_list.get_mut(topic).unwrap();
                set.remove(client_id);

            }
            else {
                send_msg(&socket, &client_id, "NOK");
            }
        },
        "GET" => {
            if subs_list.contains_key(client_id){
                let tp = subs_list.get_mut(client_id).unwrap();

                if topic == tp{
                    let first = queue.get_mut(client_id).unwrap().pop_front();
                    if first == None {
                        send_msg(&socket, &client_id, "SRY");
                    }
                    else{
                        let v = first.unwrap();
                        if send_msg(&socket, &client_id, format!("OK {}", v).as_str()) == -1 {
                            queue.get_mut(client_id).unwrap().push_front(v);
                        }
                    }
                    
                }
                else{
                    send_msg(&socket, &client_id, "NS");
                }

            }
            else {
                send_msg(&socket, &client_id, "NF");
            }

        },
        "PUT" => {
            if topics_list.contains_key(topic) {
                let set = topics_list.get_mut(topic).unwrap();
                let msg = &split[1][end +1..];

                for id in set.iter() {
                    let value = queue.get_mut(id).unwrap();
                    value.push_back(String::from(msg.trim()))
                }
            }

        },
        _ => {
            send_msg(&socket, &client_id, "NOK");
        },
    }

}