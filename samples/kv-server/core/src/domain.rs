use crate::error::KvError;

#[derive(Debug)]
pub enum Request {
    Get { key: String },
    MGet { keys: Vec<String> },
    Set { kv: KV },
    MSet { kvs: Vec<KV> },
    Del { keys: Vec<String> },
}

#[derive(Default, Debug)]
pub struct Response {
    pub code: u32,
    pub message: String,
    pub values: Vec<String>,
}

#[derive(Debug)]
pub struct KV {
    pub key: String,
    pub value: String,
}


impl From<Vec<String>> for Response {
    fn from(values: Vec<String>) -> Self {
        Self {
            values,
            ..Default::default()
        }
    }
}

impl From<KvError> for Response {
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

impl<T> From<Result<T, KvError>> for Response
where
    T: Into<Response>,
{
    fn from(result: Result<T, KvError>) -> Self {
        match result {
            Ok(val) => val.into(),
            Err(err) => Response::from(err),
        }
    }
}

impl From<Result<(), KvError>> for Response {
    fn from(result: Result<(), KvError>) -> Self {
        match result {
            Ok(_) => Response::default(),
            Err(err) => Response::from(err),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
