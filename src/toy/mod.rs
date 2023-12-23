use tokio::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Toy {
    pub sender: mpsc::Sender<ToyMessage>
}

pub enum ToyMessage {
    Greetings {
        payload: String,
        time_delay: u64
    }
}

impl Toy {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        tokio::spawn(handle_message(receiver));
        Self {
            sender
        }
    }
}

pub async fn handle_message(mut receiver: mpsc::Receiver<ToyMessage>) {
    while let Some(msg) = receiver.recv().await {
        println!("In while recv");
        tokio::spawn( async move {
            match msg {
                ToyMessage::Greetings { payload, time_delay } => {
                    println!("In Match for delay {:?} and payload {:?}", time_delay, payload);
                    tokio::time::sleep(Duration::from_secs(time_delay)).await;
                    println!("Processed payload {:?}", payload);
                }
            }
        });
    }
}
