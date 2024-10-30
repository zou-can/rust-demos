use kv_core::domain::Request::{Del, Get, MGet, MSet, Set};
use kv_core::domain::{Request, Response, KV};

use crate::storage::Storage;

/// process request
pub fn handle(request: Request, storage: &impl Storage) -> Response {
    let res = match request {
        Get { key } => storage.get(&key),
        MGet { keys } => storage.mget(&keys),
        Set { kv: KV { key, value, } } => storage.set(key, value),
        MSet { kvs } => storage.mset(kvs),
        Del { keys } => storage.del(&keys),
    };

    Response::from(res)
}


