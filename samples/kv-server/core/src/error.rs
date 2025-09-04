use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for {0}")]
    NotFound(String),

    #[error("Cannot parse command.")]
    InvalidCommand,

    #[error("Cannot process command {0} with key: {1}. Error: {2}")]
    StorageError(&'static str, String, String),

    // #[error("Failed to encode protobuf message")]
    // EncodeError(#[from] prost::EncodeError),
    //
    // #[error("Failed to decode protobuf message")]
    // DecodeError(#[from] prost::DecodeError),

    #[error("Internal error: {0}")]
    Internal(String),
}
