use meic_mq::{context::{publisher::PublisherContext, subscriber::SubscriberContext}, get, subscribe, put};

pub fn run() {
    let publisher_cars: PublisherContext = PublisherContext::new(String::from("CarsPub"));
    let mut first_sub: SubscriberContext = SubscriberContext::new(String::from("FirstSub"), "cars".to_owned());
    let mut second_sub: SubscriberContext = SubscriberContext::new(String::from("SecondSub"), "cars".to_owned());

    let sub_req = first_sub.create_subscribe_request();
    subscribe(&mut first_sub, &sub_req).unwrap();

    let payload: String = format!("Hello this is payload number {}", 0);
    let put_req = publisher_cars.create_put_request("cars".to_owned(), payload.into_bytes());
    put(&put_req).unwrap();

    let sub_req = second_sub.create_subscribe_request();
    subscribe(&mut second_sub, &sub_req).unwrap();

    let payload: String = format!("Hello this is payload number {}", 1);
    let put_req = publisher_cars.create_put_request("cars".to_owned(), payload.into_bytes());
    put(&put_req).unwrap();

    let get_req = first_sub.create_get_request();
    println!("First Sub: {}", std::str::from_utf8(get(&mut first_sub, &get_req).unwrap().as_slice()).unwrap());

    let get_req = first_sub.create_get_request();
    println!("First Sub: {}", std::str::from_utf8(get(&mut first_sub, &get_req).unwrap().as_slice()).unwrap());

    let get_req = second_sub.create_get_request();
    println!("Second Sub: {}", std::str::from_utf8(get(&mut second_sub, &get_req).unwrap().as_slice()).unwrap());

    let get_req = second_sub.create_get_request();
    if let Err(err) = get(&mut second_sub, &get_req) {
        println!("Second Sub: {}", err);
    } else {
        println!("Expected no messages to second subscriber");
    }
}