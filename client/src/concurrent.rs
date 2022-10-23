use meic_mq::{context::{publisher::PublisherContext, subscriber::SubscriberContext}, put, get, subscribe};
use std::{thread, time};

pub fn run_publisher_biology() {
    let mut publisher_biology: PublisherContext = PublisherContext::new(String::from("BiologyPub"));
    let mut message_counter = 0;
    let delay = time::Duration::from_millis(500);
    loop {
        let message = publisher_biology.create_put_request("biology".to_owned(), format!("Biology Message: {}", message_counter).into_bytes());
        if let Err(err) = put(&mut publisher_biology, &message) {
            println!("{}", err)
        } else {
            message_counter += 1;
        }
        thread::sleep(delay);
    }
}

pub fn run_publisher_cars() {
    let mut publisher_cars: PublisherContext = PublisherContext::new(String::from("CarsPub"));
    let mut message_counter = 0;
    let delay = time::Duration::from_millis(500);
    loop {
        let message = publisher_cars.create_put_request("cars".to_owned(), format!("Cars Message: {}", message_counter).into_bytes());
        if let Err(err) = put(&mut publisher_cars, &message) {
            println!("{}", err)
        } else {
            message_counter += 1;
        }
        thread::sleep(delay);
    }
}

pub fn run_subscriber_biology() {
    let mut sub = SubscriberContext::new("biology_sub".to_owned(), "biology".to_owned());
    let delay = time::Duration::from_millis(500);
    let sub_req = sub.create_subscribe_request();
    subscribe(&mut sub, &sub_req).unwrap();
    loop {
        let request = sub.create_get_request();
        match get(&mut sub, &request) {
            Ok(val) => println!("{}", std::str::from_utf8(&val).unwrap()),
            Err(err) => println!("{}", err)
        }
        thread::sleep(delay);
    }
}

pub fn run_subscriber_cars() {
    let mut sub = SubscriberContext::new("cars_sub".to_owned(), "cars".to_owned());
    let sub_req = sub.create_subscribe_request();
    subscribe(&mut sub, &sub_req).unwrap();
    let delay = time::Duration::from_millis(500);
    loop {
        let request = sub.create_get_request();
        match get(&mut sub, &request) {
            Ok(val) => println!("{}", std::str::from_utf8(&val).unwrap()),
            Err(err) => println!("{}", err)
        }
        thread::sleep(delay);
    }
}