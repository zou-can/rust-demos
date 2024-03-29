use crate::error::KvError;

include!(concat!(env!("OUT_DIR"), "/command.rs"));

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(values: Vec<Value>) -> Self {
        Self {
            values,
            ..Default::default()
        }
    }
}

impl From<Entry> for CommandResponse {
    fn from(entry: Entry) -> Self {
        Self {
            entries: vec![entry],
            ..Default::default()
        }
    }
}

impl From<Vec<Entry>> for CommandResponse {
    fn from(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let code = match err {
            KvError::NotFound(_) => 404,
            KvError::InvalidCommand => 400,
            _ => 500,
        };

        Self {
            code,
            message: err.to_string(),
            ..Default::default()
        }
    }
}

impl<T> From<Result<T, KvError>> for CommandResponse
    where T: Into<CommandResponse> {
    fn from(result: Result<T, KvError>) -> Self {
        match result {
            Ok(val) => val.into(),
            Err(err) => CommandResponse::from(err)
        }
    }
}

impl From<Result<(), KvError>> for CommandResponse {
    fn from(result: Result<(), KvError>) -> Self {
        match result {
            Ok(_) => CommandResponse::default(),
            Err(err) => CommandResponse::from(err),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            val: Some(value::Val::String(s))
        }
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Self {
            val: Some(value::Val::String(String::from(s)))
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            val: Some(value::Val::String(String::from(s)))
        }
    }
}

impl<T> From<Option<T>> for Value where T: Into<Value> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::none(),
        }
    }
}

impl Value {
    pub fn none() -> Self {
        Self {
            val: None,
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            val: Some(value::Val::Integer(i))
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}