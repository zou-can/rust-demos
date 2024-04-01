mod memory;

use kv_core::command::{Entry, Value};
use kv_core::error::KvError;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Value, KvError>;

    fn mget(&self, keys: &[String]) -> Result<Vec<Value>, KvError>;

    fn set(&self, key: String, value: String) -> Result<(), KvError>;

    fn mset(&self, entries: Vec<Entry>) -> Result<(), KvError>;

    fn del(&self, keys: &[String]) -> Result<(), KvError>;
}


#[cfg(test)]
mod tests {
    use kv_core::command::Value;

    use crate::storage::Storage;

    fn common_operation_test(store: impl Storage) {

        // empty

        // get
        let res = store.get("k1");
        assert_eq!(Ok(Value::none()), res);

        // mget
        let res = store.mget(&[String::from("v1"), String::from("v2")]);
        assert_eq!(Ok(vec![Value::none(), Value::none()]), res);

        // 插入单个值

        // set
        let res = store.set(String::from("k1"), String::from("v1"));
        assert!(res.is_ok());

        // get
        let res = store.get("k1");
        assert_eq!(Ok(Value::from("v1")), res);

        // mget
        let res = store.mget(&[String::from("v1"), String::from("v2")]);
        assert_eq!(
            Ok(
                vec![Value::from("v1"), Value::none()]
            ), res
        );

        // 插入多个值

        // set
        let res = store.set(String::from("k2"), String::from("v2"));
        assert!(res.is_ok());

        // get
        let res = store.get("k1");
        assert_eq!(Ok(Value::from("v1")), res);
        let res = store.get("k2");
        assert_eq!(Ok(Value::from("v2")), res);

        // mget
        let res = store.mget(&[String::from("v1"), String::from("v2")]);
        assert_eq!(
            Ok(
                vec![Value::from("v1"), Value::from("v2")]
            ), res
        );

        // 删除
        let res = store.del(&[String::from("v1"), String::from("v2")]);
        assert!(res.is_ok());


        let res = store.get("k1");
        assert_eq!(Ok(Value::none()), res);

        // mget
        let res = store.mget(&[String::from("v1"), String::from("v2")]);
        assert_eq!(Ok(vec![Value::none(), Value::none()]), res);
    }

    #[test]
    fn test() {
        assert_eq!("v1", String::from("v1"));
    }
}