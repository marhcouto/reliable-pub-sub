use meic_mq::context::publisher::PublisherContext;
use meic_mq::messages::put::Request;
use meic_mq::put;


pub fn publisher1() {

    let mut publisher: PublisherContext = PublisherContext::new(String::from("PubG"));
    let payload: Vec<u8> = vec![1, 2, 3, 4, 5];
    let request: Request = Request::new(publisher.pub_id.clone(), String::from("Guns"), payload);

    match put(&mut publisher, &request) {
        Ok(_) => println!("Published successfully"),
        Err(val) => panic!("Get request failed with error {}", val)
    }
}