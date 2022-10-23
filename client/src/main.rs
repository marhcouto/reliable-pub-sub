use std::env;
use std::process;

mod slow_subscriber_scenario;
mod late_sub_scen;
mod concurrent;

pub const USAGE_MESSAGE: &str = "USAGE:\ncargo run <client_case> \nclient_case: name of the case to run. Can be one of [slow_sub, late_sub,
conc_sub_cars, conc_sub_bio, conc_pub_cars, conc_pub_bio].";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Incorrect usage of the command line interface: missing arguments.");
        println!("{}", USAGE_MESSAGE);
        process::exit(1);
    }

    let case: &str = &args[1];

    match case {
        "slow_sub" => slow_subscriber_scenario::run(),
        "late_sub" => late_sub_scen::run(),
        "conc_sub_cars" => concurrent::run_subscriber_cars(),
        "conc_sub_bio" => concurrent::run_subscriber_biology(),
        "conc_pub_cars" => concurrent::run_publisher_cars(),
        "conc_pub_bio" => concurrent::run_publisher_biology(),
        _ => panic!("Function scenario {} does not exist", case)
    }


}
