use std::{collections::BTreeMap, sync::RwLock};

use bytes::Bytes;

pub struct MemTable {
    map: RwLock<BTreeMap<Bytes, Bytes>>
}

impl MemTable {
    pub fn new() -> Self {
        return Self { map: RwLock::new(BTreeMap::new()) };
    }

    pub fn get_by_key(&self, key: Bytes) -> Option<Bytes> {
        return self.map.read().unwrap().get(key.as_ref()).cloned();
    }

    pub fn insert(&self, key: Bytes, value: Bytes) {
        self.map.write().unwrap().insert(key, value);
    }
}
