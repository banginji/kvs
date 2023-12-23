use std::thread::sleep;
use tokio::sync::{broadcast, mpsc, oneshot};
use kvs::toy::{Toy, ToyMessage};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let toy = Toy::new();
    tokio::join!(
        send_message(toy.clone(), "Hello_1".to_string(), 5),
        send_message(toy.clone(), "Hello_2".to_string(), 1),
        send_message(toy.clone(), "Hello_3".to_string(), 2)
    );

    sleep(Duration::new(10, 1));

    // let (tx, rx) = mpsc::channel::<u64>(8);
    // tokio::join!(
    //     send_msg(5, tx.clone()),
    //     send_msg(1, tx.clone()),
    //     send_msg(2, tx.clone()),
    //     process_msg(rx)
    // );

    // let (tx, rx) = mpsc::channel(8);
    // tokio::join!(
    // producer(5, tx.clone()),
    // producer(3, tx.clone()),
    // producer(1, tx.clone()),
    // );
    //
    // consumer(rx).await;

    // tokio::join!(
    // sleep_print(5),
    // sleep_print(2),
    // sleep_print(1)
    //     );
}

async fn send_message(toy: Toy, message: String, time_delay: u64) {
    println!("Are you up for it in fn");
    let _ = toy.sender.send(ToyMessage::Greetings { payload: message.to_string(), time_delay }).await;
}

async fn send_msg(time_delay: u64, sender: mpsc::Sender<u64>) {
    let _ = sender.send(time_delay).await;
}

async fn process_msg(mut receiver: mpsc::Receiver<u64>) {
    while let Some(time_delay) = receiver.recv().await {
        tokio::spawn(async move {
            println!("In while recv for payload {:?}", time_delay);
            tokio::time::sleep(Duration::from_secs(time_delay)).await;
            println!("Processed payload {:?}", time_delay);
        });
    }
}

async fn producer(payload: u64, tx: mpsc::Sender<u64>) {
    let _ = tx.send(payload).await;
}

async fn consumer(mut rx: mpsc::Receiver<u64>) {
    while let Some(time_delay) = rx.recv().await {
        println!("Received payload {:?}", time_delay);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(time_delay)).await;
            println!("Completed processing for time delay {:?}", time_delay)
        });
    }
}

async fn sleep_print(key: u64) {
    tokio::time::sleep(Duration::from_secs(key)).await;
    println!("For {:?}", key);
}
