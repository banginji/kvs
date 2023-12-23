use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use kvs::actor::DbActorHandle;

#[tokio::main]
async fn main() {
    let actor_handle = Arc::new(DbActorHandle::new());

    for n in 1..100 {
        let handle = actor_handle.clone();
        tokio::spawn(async move {
            let _ = handle.set_value(n.to_string(), n.to_string(), 50/n).await;
        });
    }

    // tokio::join!(
    //     set(1, 5, actor_handle.clone()),
    //     set(2, 1, actor_handle.clone()),
    //     set(3, 1, actor_handle.clone()),
    //     set(4, 4, actor_handle.clone()),
    //     set(5, 1, actor_handle.clone()),
    //     set(6, 1, actor_handle.clone()),
    //     set(7, 1, actor_handle.clone()),
    //     set(8, 1, actor_handle.clone()),
    //     set(9, 1, actor_handle.clone())
    // );

    sleep(Duration::new(5, 1));

    for n in 1..100 {
        let handle = actor_handle.clone();

        let result = handle.get_by_key(n.to_string()).await;
        // println!("Set result: {:?}", result);
    }

    // tokio::join!(
    //     get(1, actor_handle.clone()),
    //     get(2, actor_handle.clone()),
    //     get(3, actor_handle.clone()),
    //     get(4, actor_handle.clone()),
    //     get(5, actor_handle.clone()),
    //     get(6, actor_handle.clone()),
    //     get(7, actor_handle.clone()),
    //     get(8, actor_handle.clone()),
    //     get(9, actor_handle.clone())
    // );

    // tokio::join!(
    //     sleep_print(3),
    //     sleep_print(2),
    //     sleep_print(1)
    // );
}

async fn set(elem: i8, time: u64, handle: Arc<DbActorHandle>) {
    let _ = handle.set_value(elem.to_string(), elem.to_string(), time).await;
}

async fn get(elem: i8, handle: Arc<DbActorHandle>) {
    let _ = handle.get_by_key(elem.to_string()).await;
}

async fn sleep_print(key: u64) {
    tokio::time::sleep(Duration::from_secs(key)).await;
    println!("For {:?}", key);
}