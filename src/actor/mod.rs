use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

use crate::store::Store;

pub struct DbActor {
    store: Arc<Store>,
    receiver: mpsc::Receiver<DbActorMessage>
}

pub enum DbActorMessage {
    Get {
        key: String,
        respond_to: oneshot::Sender<Option<String>>
    },
    Set {
        key: String,
        value: String,
        time: u64,
        respond_to: oneshot::Sender<Option<String>>
    }
}

impl DbActor {
    pub fn new(receiver: mpsc::Receiver<DbActorMessage>) -> Self {
        Self {
            store: Arc::new(Store::new()),
            receiver,
        }
    }

    pub async fn handle_message(&mut self, message: DbActorMessage) {
        match message {
            DbActorMessage::Get { key, respond_to } => {
                let _ = respond_to.send(self.store.get_by_key(&key).await);
            }
            DbActorMessage::Set { key, value, time, respond_to } => {
                self.store.insert(key, value, time).await;
                let _ = respond_to.send(Some("OK".to_string()));
            }
        }
    }
}

#[derive(Clone)]
pub struct DbActorHandle {
    sender: mpsc::Sender<DbActorMessage>
}

impl DbActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = DbActor::new(receiver);
        tokio::spawn(run_actor(actor));

        Self { sender }
    }

    pub async fn get_by_key(&self, key: String) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let msg = DbActorMessage::Get {
            key,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        println!("Get operation");
        recv.await.expect("Actor has been killed")
    }

    pub async fn set_value(&self, key: String, value: String, time: u64) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let msg = DbActorMessage::Set {
            key,
            value,
            time,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        println!("Set operation");
        let result = recv.await.expect("Actor has been killed");
        result
    }
}

async fn run_actor(actor: DbActor) {
    let store = actor.store;
    let mut receiver = actor.receiver;
    while let Some(msg) = receiver.recv().await {
        let store = store.clone();
        tokio::spawn(async move {
            println!("handling message");
            handle_message(store.as_ref(), msg).await;
        });
    }
}

pub async fn handle_message(store: &Store, message: DbActorMessage) {
    match message {
        DbActorMessage::Get { key, respond_to } => {
            let _ = respond_to.send(store.get_by_key(&key).await);
        }
        DbActorMessage::Set { key, value, time, respond_to } => {
            store.insert(key, value, time).await;
            let _ = respond_to.send(Some("OK".to_string()));
        }
    }
}
