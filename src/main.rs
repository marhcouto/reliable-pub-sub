#![allow(unused_imports)]
use meic_mq::messages;
use meic_mq::messages::NetworkTradable;
use bson;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {
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
