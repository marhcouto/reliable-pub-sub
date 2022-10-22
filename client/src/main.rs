use std::env;
use std::process;

mod subscriber;
mod publisher;
mod slow_subscriber_scenario;
mod late_sub_scen;

pub const USAGE_MESSAGE: &str = "USAGE:\ncargo run <service> <service_number> \nservice: name of the service to run. Can be one of subscribe or publisher\n
service_number: number of the function to be executed.";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Incorrect usage of the command line interface: missing arguments.");
        println!("{}", USAGE_MESSAGE);
        process::exit(1);
    }

    let service: &String = &args[1];
    let service_number: &str = &args[2];

    match service.as_str() {
        "subscriber" => {
            match service_number {
                "1" => subscriber::subscriber1(),
                "2" => subscriber::subscriber2(),
                "3" => subscriber::subscriber3(),
                _ => panic!("Function subscriber{} does not exist", service_number)
            }
        },
        "publisher" => {
            match service_number {
                "1" => publisher::publisher1(),
                _ => panic!("Function publisher{} does not exist", service_number)
            }
        },
        "scenario" => { 
            match service_number {
                "slow_sub" => slow_subscriber_scenario::run(),
                "late_sub" => late_sub_scen::run(),
                _ => panic!("Function scenario {} does not exist", service_number)
            }
        }
        _ => panic!("Unknown service string {}", service)
    }
}
