use meic_mq::{context::{publisher::PublisherContext, subscriber::SubscriberContext}, put, get, subscribe};
use std::{thread, time};

pub fn run_publisher_biology(pub_id: Option<&String>) {
    let publisher_id = match pub_id {
        Some(val) => val.clone(),
        None => "BiologyPub".to_owned()
    };
    let publisher_biology: PublisherContext = PublisherContext::new(publisher_id);
    let mut message_counter = 0;
    let delay = time::Duration::from_millis(500);
    loop {
        let message = publisher_biology.create_put_request("biology".to_owned(), format!("Biology Message: {}", message_counter).into_bytes());
        if let Err(err) = put(&message) {
            println!("{}", err)
        } else {
            println!("Message published");
            message_counter += 1;
        }
        thread::sleep(delay);
    }
}

pub fn run_publisher_cars(pub_id: Option<&String>) {
    let publisher_id = match pub_id {
        Some(val) => val.clone(),
        None => "CarsPub".to_owned()
    };
    let publisher_cars: PublisherContext = PublisherContext::new(publisher_id);
    let mut message_counter = 0;
    let delay = time::Duration::from_millis(500);
    loop {
        let message = publisher_cars.create_put_request("cars".to_owned(), format!("Cars Message: {}", message_counter).into_bytes());
        if let Err(err) = put(&message) {
            println!("{}", err)
        } else {
            println!("Message published");
            message_counter += 1;
        }
        thread::sleep(delay);
    }
}

pub fn run_subscriber_biology(sub_id: Option<&String>) {
    let subscriber_id = match sub_id {
        Some(val) => val.clone(),
        None => "biology_sub".to_owned()
    };
    let mut sub = SubscriberContext::new(subscriber_id, "biology".to_owned());
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

pub fn run_subscriber_cars(sub_id: Option<&String>) {
    let subscriber_id = match sub_id {
        Some(val) => val.clone(),
        None => "cars_sub".to_owned()
    };
    let mut sub = SubscriberContext::new(subscriber_id, "cars".to_owned());
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