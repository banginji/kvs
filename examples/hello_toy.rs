use std::thread::sleep;

use tokio::time::Duration;

use kvs::toy::{Toy, ToyMessage};

#[tokio::main]
async fn main() {
    let toy = Toy::new();

    tokio::join!(
        send_message(toy.clone(), "Hello_1".to_string(), 5),
        send_message(toy.clone(), "Hello_2".to_string(), 1),
        send_message(toy.clone(), "Hello_3".to_string(), 2)
    );

    sleep(Duration::new(6, 1));
}

async fn send_message(toy: Toy, message: String, time_delay: u64) {
    let _ = toy.sender.send(ToyMessage::Greetings { payload: message.to_string(), time_delay }).await;
}
