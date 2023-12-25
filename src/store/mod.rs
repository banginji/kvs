use std::collections::HashMap;
use std::sync::{RwLock};
use std::thread;

use tokio::time::{Duration, sleep};

use crate::actor::DbActorMessage;

pub struct Store {
    mem: RwLock<HashMap<String, String>>
}

impl Store {
    pub fn new() -> Self {
        return Self { mem: RwLock::new(HashMap::new()) }
    }

    pub async fn handle_message(&self, message: DbActorMessage) {
        match message {
            DbActorMessage::Get { key, time_delay, respond_to } => {
                let _ = respond_to.send(self.get_by_key(&key, time_delay).await);
            }
            DbActorMessage::Set { key, value, time_delay, respond_to } => {
                self.insert(key, value, time_delay).await;
                let _ = respond_to.send(Some("OK".to_string()));
            }
        }
    }

    pub async fn insert(&self, key: String, value: String, time: u64) {
        sleep(Duration::from_secs(time)).await;
        println!("Set operation for {:?} with time_delay {:?} in thread {:?}", key, time, thread::current().id());
        let mut guard = self.mem.write().unwrap();
        guard.insert(key, value);
    }

    pub fn remove(&self, key: String) {
        let mut guard = self.mem.write().unwrap();
        guard.remove(&key);
    }

    pub async fn get_by_key(&self, key: &String, time_delay: u64) -> Option<String> {
        sleep(Duration::from_secs(time_delay)).await;
        let guard = self.mem.read().unwrap();
        let res = guard.get(key).cloned();
        println!("Get result for key {:?} with delay {:?} in thread {:?}: {:?}", key, time_delay, thread::current().id(), res);
        res
    }

    pub fn print_all_elements(&self) {
        let guard = self.mem.read().unwrap();
        for (key, value) in guard.iter() {
            println!("Get result in thread {:?} -> key: {:?}, value:{:?}", thread::current().id(), key, value);
        }
    }
}
