use std::collections::HashMap;
use std::sync::{Mutex};
use std::thread;

use tokio::time::{Duration, sleep};

use crate::actor::DbActorMessage;

pub struct Store {
    mem: Mutex<HashMap<String, String>>
}

impl Store {
    pub fn new() -> Self {
        return Self { mem: Mutex::new(HashMap::new()) }
    }

    pub async fn handle_message(&self, message: DbActorMessage) {
        match message {
            DbActorMessage::Get { key, respond_to } => {
                let _ = respond_to.send(self.get_by_key(&key).await);
            }
            DbActorMessage::Set { key, value, time, respond_to } => {
                self.insert(key, value, time).await;
                let _ = respond_to.send(Some("OK".to_string()));
            }
        }
    }

    pub async fn insert(&self, key: String, value: String, time: u64) {
        sleep(Duration::from_secs(time)).await;
        println!("Set operation for {:?} with time_delay {:?} in thread {:?}", key, time, thread::current().id());
        let mut guard = self.mem.lock().unwrap();
        guard.insert(key, value);
    }

    pub fn remove(&self, key: String) {
        let mut guard = self.mem.lock().unwrap();
        guard.remove(&key);
    }

    pub async fn get_by_key(&self, key: &String) -> Option<String> {
        sleep(Duration::from_millis(10)).await;
        let guard = self.mem.lock().unwrap();
        let res = guard.get(key).cloned();
        println!("Get result for key {:?} in thread {:?}: {:?}", key, thread::current().id(), res);
        res
    }

    pub fn print_all_elements(&self) {
        let guard = self.mem.lock().unwrap();
        for (key, value) in guard.iter() {
            println!("Get result in thread {:?} -> key: {:?}, value:{:?}", thread::current().id(), key, value);
        }
    }
}
