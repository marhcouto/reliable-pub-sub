use meic_mq::{context::{publisher::PublisherContext, subscriber::SubscriberContext}, get, subscribe, put, unsubscribe};

pub fn run() {
    let mut publisher_tech: PublisherContext = PublisherContext::new(String::from("tech"));

    let mut fast_sub: SubscriberContext = SubscriberContext::new(String::from("FastSub"), String::from("tech"));
    let fast_sub_req = fast_sub.create_subscribe_request();
    subscribe(&mut fast_sub, &fast_sub_req).unwrap();

    let mut slow_sub: SubscriberContext = SubscriberContext::new(String::from("SlowSub"), String::from("tech"));
    let slow_sub_req = slow_sub.create_subscribe_request();
    subscribe(&mut slow_sub, &slow_sub_req).unwrap();

    for i in 0..5 {
        let payload: String = format!("Hello this is payload number {}", i);
        let put_req = publisher_tech.create_put_request("tech".to_owned(), payload.into_bytes());
        put(&mut publisher_tech, &put_req).unwrap();
    }

    println!("This is the fast sub reading");
    for _ in 0..5 {
        let get_req = fast_sub.create_get_request();
        println!("Received: {}", std::str::from_utf8(get(&mut fast_sub, &get_req).unwrap().as_slice()).unwrap());
    }

    println!("This is the slow sub reading");
    for _ in 0..5 {
        let get_req = slow_sub.create_get_request();
        println!("Received: {}", std::str::from_utf8(get(&mut slow_sub, &get_req).unwrap().as_slice()).unwrap());
    }

    let slow_unsub_req = slow_sub.create_unsubscribe_request();
    unsubscribe(&mut slow_sub, &slow_unsub_req).unwrap();

    let fast_unsub_req = fast_sub.create_unsubscribe_request();
    unsubscribe(&mut fast_sub, &fast_unsub_req).unwrap();
}