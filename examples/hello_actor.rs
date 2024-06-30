use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use bytes::Bytes;
use kvs::actor::kvs_actor_handle::KvsActorHandle;

#[tokio::main]
async fn main() {
    let actor_handle = Arc::new(KvsActorHandle::new());

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let _ = handle.set_value(
                Bytes::from(n.clone().to_string()),
                Bytes::from(n.clone().to_string())
            ).await;
        });
    }

    sleep(Duration::new(1, 1));

    actor_handle.set_value(Bytes::from(1.to_string()), Bytes::from(11.to_string())).await;

    actor_handle.remove_value(Bytes::from(3.to_string())).await;

    sleep(Duration::new(1, 1));

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let result = handle.get_by_key(
                Bytes::from(n.clone().to_string())
            ).await;
            println!("Get result: {:?}", result);
        });
    }

    sleep(Duration::new(1, 1));
}
