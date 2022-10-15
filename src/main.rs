#![allow(unused_imports)]
use meic_mq::messages;
use meic_mq::messages::NetworkTradeable;
use meic_mq::context::subscriber;
use meic_mq::context::publisher;

use bson;
use::std::env;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {

//    let usage_message: String = String::from("USAGE:\ncargo run <service>\nservice: name of the service to run. Can be one of subscriber, publisher or broker.");
//    let args: Vec<String> = env::args().collect();
//
//    if args.len() < 2 {
//        println!("Incorrect usage of the command line interface: missing arguments.");
//        println!("{}", usage_message);
//    }



    

    /*let person = Person {
        name: "Francisco Oliveira".to_string(),
        age: 18
    };
    let data = bson::to_vec(&person).unwrap();
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(&data).unwrap();
    file = File::open("foo.txt").unwrap();
    let file_data: Person = bson::from_reader(BufReader::new(file)).unwrap();
    dbg!(file_data);*/
}
