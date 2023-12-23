use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep, Duration};

pub struct Store {
    mem: Mutex<HashMap<String, String>>
}

impl Store {
    pub fn new() -> Self {
        return Self { mem: Mutex::new(HashMap::new()) }
    }

    pub async fn insert(&self, key: String, value: String, time: u64) {
        sleep(Duration::from_secs(time)).await;
        println!("Set operation for {:?} with time_delay {:?} in thread {:?}", key, time, thread::current().id());
        let mut mem = self.mem.lock().unwrap();
        // let key_clone = key.clone();
        mem.insert(key, value);
        // println!("Finished set operation for {:?}", key.clone());
    }

    pub fn remove(&mut self, key: String) {
        self.mem.lock().unwrap().remove(&key);
    }

    pub async fn get_by_key(&self, key: &String) -> Option<String> {
        sleep(Duration::from_millis(10)).await;
        let res = self.mem.lock().unwrap().get(key).cloned();
        println!("Get result for key {:?} in thread {:?}: {:?}", key, thread::current().id(), res);
        res
    }

    pub fn print_all_elements(&self) {
        for (key, value) in self.mem.lock().unwrap().iter() {
            println!("Get result in thread {:?} -> key: {:?}, value:{:?}", thread::current().id(), key, value);
        }
    }
}
