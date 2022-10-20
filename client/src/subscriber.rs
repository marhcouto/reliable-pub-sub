use meic_mq::context::subscriber::SubscriberContext;
use meic_mq::messages::subscribe::Request as SubRequest;
use meic_mq::messages::unsubscribe::Request as UnsubRequest;
use meic_mq::messages::get::Request as GetRequest;
use meic_mq::{ get, subscribe, unsubscribe };

pub fn subscriber1() {

    let mut sub_ctx: SubscriberContext = SubscriberContext::new(String::from("SubD"), String::from("GUNS"));
    let sub_request: SubRequest = sub_ctx.create_subscribe_request();

    match subscribe(&mut sub_ctx, &sub_request) {
        Ok(_) => println!("Subscribed successfully"),
        Err(val) => panic!("Subscribe failed with error: {}", val)
    }

    let get_request: GetRequest = sub_ctx.create_get_request();
    match get(&mut sub_ctx, &get_request) {
        Ok(payload) => println!("Payload: {:?}", payload),
        Err(val) => eprintln!("Get request failed with error: {}", val)
    }

    let unsub_request: UnsubRequest = sub_ctx.create_unsubscribe_request();
    match unsubscribe(&mut sub_ctx, &unsub_request) {
        Ok(_) => println!("Unsubscribed successfully"),
        Err(val) => eprintln!("Get request failed with error: {}", val)
    }
}

pub fn subscriber2() {

    let mut sub_ctx: SubscriberContext = SubscriberContext::new(String::from("SubG"), String::from("GUNS"));
    let sub_request: SubRequest = sub_ctx.create_subscribe_request();

    match subscribe(&mut sub_ctx, &sub_request) {
        Ok(_) => println!("Subscribed successfully"),
        Err(val) => panic!("Subscribe failed with error: {}", val)
    }

    let get_request: GetRequest = sub_ctx.create_get_request();
    match get(&mut sub_ctx, &get_request) {
        Ok(payload) => println!("Payload: {:?}", payload),
        Err(val) => eprintln!("Get request failed with error: {}", val)
    }
}

pub fn subscriber3() {

    let mut sub_ctx: SubscriberContext = SubscriberContext::new(String::from("SubG"), String::from("GUNS"));

    let get_request: GetRequest = sub_ctx.create_get_request();
    match get(&mut sub_ctx, &get_request) {
        Ok(payload) => println!("Payload: {:?}", payload),
        Err(val) => eprintln!("Get request failed with error: {}", val)
    }
}