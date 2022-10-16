use std::env;
use std::process;
use serde::{Serialize, Deserialize};

mod subscriber;
mod publisher;
mod broker;
mod parsing;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Incorrect usage of the command line interface: missing arguments.");
        println!("{}", parsing::USAGE_MESSAGE);
        process::exit(1);
    }

    let service: &String = &args[1];

    match service.as_str() {
        "subscriber" => subscriber::subscriber(&args),
        "publisher" => publisher::publisher(&args),
        "broker" => broker::broker(&args),
        _ => panic!("Unknown service string {}", service)
    }
}
