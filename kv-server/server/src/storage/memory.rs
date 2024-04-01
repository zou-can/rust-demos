use std::collections::HashMap;
use std::sync::RwLock;

use kv_core::command::{Entry, Value};
use kv_core::command::value::Val;
use kv_core::error::KvError;

use crate::storage::Storage;

#[derive(Debug, Default)]
struct Memory {
    // todo: use better cache lib in future
    map: RwLock<HashMap<String, String>>,
}

impl Memory {
    pub fn new() -> Self {
        Default::default()
    }
}


impl Storage for Memory {
    fn get(&self, key: &str) -> Result<Value, KvError> {
        let guard = self.map.read().unwrap();
        Ok(Value::from(guard.get(key)))
    }

    fn mget(&self, keys: &[String]) -> Result<Vec<Value>, KvError> {
        let guard = self.map.read().unwrap();
        let values = keys.iter()
            .map(|k| {
                Value::from(guard.get(k))
            })
            .collect();

        Ok(values)
    }

    fn set(&self, key: String, value: String) -> Result<(), KvError> {
        self.map.write().unwrap().insert(key, value);
        Ok(())
    }

    fn mset(&self, entries: Vec<Entry>) -> Result<(), KvError> {
        let mut guard = self.map.write().unwrap();
        for entry in entries {
            if let Val::String(v) = entry.value.unwrap().val.unwrap() {
                guard.insert(entry.key, v);
            }
        }

        Ok(())
    }

    fn del(&self, keys: &[String]) -> Result<(), KvError> {
        let mut guard = self.map.write().unwrap();
        for key in keys {
            guard.remove(key);
        }
        Ok(())
    }
}