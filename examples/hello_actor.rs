use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use bytes::Bytes;

use kvs::actor::DbActorHandle;

#[tokio::main]
async fn main() {
    let actor_handle = Arc::new(DbActorHandle::new());

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let _ = handle.set_value(
                Bytes::from(n.clone().to_string()),
                Bytes::from(n.clone().to_string()),
                5/n.clone()
            ).await;
        });
    }

    sleep(Duration::new(6, 1));

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let result = handle.get_by_key(
                Bytes::from(n.clone().to_string()),
                5/n.clone()
            ).await;
            println!("Get result: {:?}", result);
        });
    }

    sleep(Duration::new(6, 1));

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let result = handle.remove_value(
                Bytes::from(n.clone().to_string()),
                5/n.clone()
            ).await;
            println!("Removal result: {:?}", result);
        });
    }

    sleep(Duration::new(6, 1));

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let result = handle.get_by_key(
                Bytes::from(n.clone().to_string()),
                5/n.clone()
            ).await;
            println!("Get result after removal: {:?}", result);
        });
    }

    sleep(Duration::new(6, 1));
}
