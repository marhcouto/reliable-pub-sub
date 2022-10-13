use meic_mq::messages;
use bson;

fn main() {
    let payload = vec![12, 13, 14, 15];
    let message = messages::put::Request::new("teste".to_string(), "teste".to_string(), "teste".to_string(), payload);
    dbg!(bson::to_bson(&message).unwrap());

}
