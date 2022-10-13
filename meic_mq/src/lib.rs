pub mod messages;

struct Context {
    socket: zmq::Socket
}

impl Context {
    pub fn new(endpoint: &String) {
        
    }
}

pub fn get(sub_id: &String, topic: &String) {
    panic!("TODO: implement get")
}

pub fn put(pub_id: &String, topic: &String, message: &Vec<u8>) {
    panic!("TODO: implement put")
}

pub fn subscribe(sub_id: &String, topic: &String) {
    panic!("TODO: implement subscribe")
}

pub fn unsubscribe(sub_id: &String, topic: &String) {
    panic!("TODO: implement unsubscribe")
}
