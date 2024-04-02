use std::sync::Arc;

use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use crate::store::lsm_store::LsmStore;

pub struct KvsActor {
    store: Arc<LsmStore>,
    receiver: mpsc::Receiver<KvsActorMessage>
}

pub enum KvsActorMessage {
    Get {
        key: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>
    },
    Set {
        key: Bytes,
        value: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>
    },
    Delete {
        key: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>
    }
}

impl KvsActor {
    pub fn new(receiver: mpsc::Receiver<KvsActorMessage>) -> Self {
        Self {
            store: Arc::new(LsmStore::new()),
            receiver,
        }
    }
}

#[derive(Clone)]
pub struct KvsActorHandle {
    sender: mpsc::Sender<KvsActorMessage>
}

impl KvsActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);

        let actor = KvsActor::new(receiver);
        tokio::spawn(run_actor(actor));

        Self { sender }
    }

    pub async fn get_by_key(&self, key: Bytes) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
        let msg = KvsActorMessage::Get {
            key,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor has been killed")
    }

    pub async fn set_value(&self, key: Bytes, value: Bytes) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
        let msg = KvsActorMessage::Set {
            key,
            value,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        let result = recv.await.expect("Actor has been killed");
        result
    }

    pub async fn remove_value(&self, key: Bytes) -> Option<Bytes> {
        let (send, recv) = oneshot::channel::<Option<Bytes>>();
        let msg = KvsActorMessage::Delete {
            key,
            respond_to: send
        };
        let _ = self.sender.send(msg).await;
        let result = recv.await.expect("Actor has been killed");
        result
    }
}

async fn run_actor(actor: KvsActor) {
    let store = actor.store;
    let mut receiver = actor.receiver;

    while let Some(msg) = receiver.recv().await {
        let store = store.clone();
        tokio::spawn(async move {
            store.handle_message(msg);
        });
    }
}
