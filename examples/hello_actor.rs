use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use kvs::actor::DbActorHandle;

#[tokio::main]
async fn main() {
    let actor_handle = Arc::new(DbActorHandle::new());

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let _ = handle.set_value(n.to_string(), n.to_string(), 5/n).await;
        });
    }

    sleep(Duration::new(6, 1));

    for n in 1..11 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let result = handle.get_by_key(n.to_string(), 5/n).await;
            println!("Get result: {:?}", result);
        });
    }

    sleep(Duration::new(6, 1));
}
