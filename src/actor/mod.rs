use std::sync::{Arc};

use tokio::sync::{mpsc, oneshot};

use crate::store::Store;

pub struct DbActor {
    store: Arc<Store>,
    receiver: mpsc::Receiver<DbActorMessage>
}

pub enum DbActorMessage {
    Get {
        key: String,
        time_delay: u64,
        respond_to: oneshot::Sender<Option<String>>
    },
    Set {
        key: String,
        value: String,
        time_delay: u64,
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

    pub async fn get_by_key(&self, key: String, time_delay: u64) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let msg = DbActorMessage::Get {
            key,
            time_delay,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor has been killed")
    }

    pub async fn set_value(&self, key: String, value: String, time_delay: u64) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let msg = DbActorMessage::Set {
            key,
            value,
            time_delay,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
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
            store.handle_message(msg).await;
        });
    }
}
