pub(crate) mod memory;

use kv_core::domain::KV;
use kv_core::error::KvError;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Vec<String>, KvError>;

    fn mget(&self, keys: &[String]) -> Result<Vec<String>, KvError>;

    fn set(&self, key: String, value: String) -> Result<Vec<String>, KvError>;

    fn mset(&self, kvs: Vec<KV>) -> Result<Vec<String>, KvError>;

    fn del(&self, keys: &[String]) -> Result<Vec<String>, KvError>;
}

#[cfg(test)]
mod tests {
    use crate::storage::{memory, Storage};

    fn common_operation_test(store: impl Storage) {
        // empty

        // get
        let res = store.get("k1");
        assert_eq!(Ok(vec![]), res);

        // mget
        let res = store.mget(&[String::from("k1"), String::from("k2")]);
        assert_eq!(Ok(vec![]), res);

        // 插入单个值

        // set
        let res = store.set(String::from("k1"), String::from("v1"));
        assert!(res.is_ok());

        // get
        let res = store.get("k1");
        assert!(res.is_ok());
        assert_eq!(vec![String::from("v1")], res.unwrap());

        // mget
        let res = store.mget(&[String::from("k1"), String::from("k2")]);
        assert!(res.is_ok());
        assert_eq!(vec![String::from("v1")], res.unwrap());

        // 插入多个值

        // set
        let res = store.set(String::from("k2"), String::from("v2"));
        assert!(res.is_ok());

        // get
        let res = store.get("k1");
        assert!(res.is_ok());
        assert_eq!(vec![String::from("v1")], res.unwrap());
        let res = store.get("k2");
        assert!(res.is_ok());
        assert_eq!(vec![String::from("v2")], res.unwrap());

        // mget
        let res = store.mget(&[String::from("k1"), String::from("k2")]);
        assert!(res.is_ok());
        assert_eq!(vec![String::from("v1"), String::from("v2")], res.unwrap());

        // 删除
        let res = store.del(&[String::from("k1"), String::from("k2")]);
        assert!(res.is_ok());

        let res = store.get("k1");
        assert_eq!(Ok(vec![]), res);

        // mget
        let res = store.mget(&[String::from("k1"), String::from("k2")]);
        assert_eq!(Ok(vec![]), res);
    }

    #[test]
    fn test_memory_storage() {
        let store = memory::Memory::new();
        common_operation_test(store)
    }
}
