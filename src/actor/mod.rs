use bytes::Bytes;
use tokio::sync::oneshot;
pub enum KvsActorMessage {
    Get {
        key: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>,
    },
    Set {
        key: Bytes,
        value: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>,
    },
    Delete {
        key: Bytes,
        respond_to: oneshot::Sender<Option<Bytes>>,
    },
}

pub mod kvs_actor;
pub mod kvs_actor_handle;
