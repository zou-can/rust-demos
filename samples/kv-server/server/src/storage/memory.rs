use std::collections::HashMap;
use std::sync::RwLock;
use kv_core::domain::KV;
use kv_core::error::KvError;

use crate::storage::Storage;

#[derive(Debug, Default)]
pub(crate) struct Memory {
    // todo: use better cache lib in future
    map: RwLock<HashMap<String, String>>,
}

impl Memory {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Storage for Memory {
    fn get(&self, key: &str) -> Result<Vec<String>, KvError> {
        let mut res = vec![];

        let guard = self.map.read().unwrap();
        if let Some(v) = guard.get(key) {
            res.push(String::from(v));
        }

        Ok(res)
    }

    fn mget(&self, keys: &[String]) -> Result<Vec<String>, KvError> {
        let guard = self.map.read().unwrap();

        let res = keys.iter()
            .filter_map(|key| guard.get(key))
            .map(|s| String::from(s))
            .collect();

        Ok(res)
    }

    fn set(&self, key: String, value: String) -> Result<Vec<String>, KvError> {
        self.map.write().unwrap().insert(key, value);
        Ok(vec![])
    }

    fn mset(&self, kvs: Vec<KV>) -> Result<Vec<String>, KvError> {
        let mut guard = self.map.write().unwrap();

        for KV { key, value } in kvs {
            guard.insert(key, value);
        }

        Ok(vec![])
    }

    fn del(&self, keys: &[String]) -> Result<Vec<String>, KvError> {
        let mut guard = self.map.write().unwrap();
        for key in keys {
            guard.remove(key);
        }
        Ok(vec![])
    }
}
