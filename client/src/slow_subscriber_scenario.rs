use meic_mq::{context::{publisher::PublisherContext, subscriber::SubscriberContext}, get, subscribe, put};

pub fn run() {
    let mut publisher_tech: PublisherContext = match PublisherContext::from_file(&String::from("Publisher")) {
        Ok(val) => {
            println!("Found pub with data: {:?}", val);
            val
        },
        Err(_) => {
            println!("Creating new pub");
            PublisherContext::new(String::from("tech"))
        }
    };

    let mut fast_sub: SubscriberContext = match SubscriberContext::from_file(&String::from("FastSub")) {
        Ok(val) => {
            println!("Found fast sub with data: {:?}", val);
            val
        },
        Err(_) => {
            println!("Creating new fast sub");
            let mut fast_sub = SubscriberContext::new(String::from("FastSub"), String::from("tech"));
            let fast_sub_sub_req = fast_sub.create_subscribe_request();
            subscribe(&mut fast_sub, &fast_sub_sub_req).unwrap();
            fast_sub
        }
    };

    let mut slow_sub: SubscriberContext = match SubscriberContext::from_file(&String::from("SlowSub")) {
        Ok(val) => {
            println!("Found slow sub with data: {:?}", val);
            val
        },
        Err(_) => {
            println!("Creating new slow sub");
            let mut slow_sub = SubscriberContext::new(String::from("SlowSub"), String::from("tech"));
            let slow_sub_sub_req = slow_sub.create_subscribe_request();
            subscribe(&mut slow_sub, &slow_sub_sub_req).unwrap();
            slow_sub
        }
    };

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
}