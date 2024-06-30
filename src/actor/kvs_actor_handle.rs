use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use super::{kvs_actor::{run_actor, KvsActor}, KvsActorMessage};

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
