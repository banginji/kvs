use bytes::Bytes;

use crate::{actor::KvsActorMessage, mem_table::MemTable};

pub struct LsmStore {
    mem_table: MemTable
}

impl LsmStore {
    pub fn new() -> Self {
        Self { mem_table: MemTable::new() }
    }

    pub fn handle_message(&self, message: KvsActorMessage) {
        match message {
            KvsActorMessage::Get { key, respond_to } => {
                let _ = respond_to.send(self.get(key));
            }
            KvsActorMessage::Set { key, value, respond_to } => {
                self.set(key, value);
                let _ = respond_to.send(Some(Bytes::from("OK")));
            }
            KvsActorMessage::Delete { key, respond_to } => {
                self.delete(key);
                let _ = respond_to.send(Some(Bytes::from("OK")));
            }
        }
    }

    fn get(&self, key: Bytes) -> Option<Bytes> {
        self.mem_table.get_by_key(key)
    }

    fn set(&self, key: Bytes, value: Bytes) {
        self.mem_table.insert(key, value);
    }

    fn delete(&self, key: Bytes) {
        self.mem_table.insert(key, Bytes::new());
    }
}
