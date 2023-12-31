use std::sync::Arc;

use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use crate::store::Store;

pub struct DbActor {
    store: Arc<Store>,
    receiver: mpsc::Receiver<DbActorMessage>
}

pub enum DbActorMessage {
    Get {
        key: Bytes,
        time_delay: u64,
        respond_to: oneshot::Sender<Option<Bytes>>
    },
    Set {
        key: Bytes,
        value: Bytes,
        time_delay: u64,
        respond_to: oneshot::Sender<Option<Bytes>>
    },
    Remove {
        key: Bytes,
        time_delay: u64,
        respond_to: oneshot::Sender<Option<Bytes>>
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

    pub async fn get_by_key(&self, key: Bytes, time_delay: u64) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
        let msg = DbActorMessage::Get {
            key,
            time_delay,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor has been killed")
    }

    pub async fn set_value(&self, key: Bytes, value: Bytes, time_delay: u64) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
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

    pub async fn remove_value(&self, key: Bytes, time_delay: u64) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
        let msg = DbActorMessage::Remove {
            key,
            time_delay,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor has been killed")
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
