use std::env;
use std::process;

mod subscriber;
mod publisher;
mod parsing;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Incorrect usage of the command line interface: missing arguments.");
        println!("{}", parsing::USAGE_MESSAGE);
        process::exit(1);
    }

    let service: &String = &args[1];
    let service_number: &u16 = &args[2].parse::<u16>().unwrap();

    match service.as_str() {
        "subscriber" => {
            match service_number {
                1 => subscriber::subscriber1(&args),
                _ => panic!("Function subscriber{} does not exist", service_number)
            }
        },
        "publisher" => {
            match service_number {
                1 => publisher::publisher1(&args),
                _ => panic!("Function publisher{} does not exist", service_number)
            }
        },
        _ => panic!("Unknown service string {}", service)
    }
}
