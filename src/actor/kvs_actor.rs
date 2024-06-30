use std::sync::Arc;

use tokio::sync::mpsc;

use crate::store::lsm_store::LsmStore;

use super::KvsActorMessage;

pub struct KvsActor {
    store: Arc<LsmStore>,
    receiver: mpsc::Receiver<KvsActorMessage>
}

impl KvsActor {
    pub fn new(receiver: mpsc::Receiver<KvsActorMessage>) -> Self {
        Self {
            store: Arc::new(LsmStore::new()),
            receiver,
        }
    }
}

pub async fn run_actor(actor: KvsActor) {
    let store = actor.store;
    let mut receiver = actor.receiver;

    while let Some(msg) = receiver.recv().await {
        let store = store.clone();
        tokio::spawn(async move {
            store.handle_message(msg);
        });
    }
}
