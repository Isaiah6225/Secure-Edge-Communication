use std::{io, env};

//error enum
#[derive(Debug)]
pub enum ServerError {
    VarErr(env::VarError),
    IoErr(io::Error),
    SerdeStrErr(serde_json::Error),
    InvalidKeyLength(usize),
    SqlErr(rusqlite::Error),
    OneshotRecvErr(tokio::sync::oneshot::error::RecvError),
}

impl From<env::VarError> for ServerError {
    fn from(error: env::VarError) -> Self {
        ServerError::VarErr(error)
    }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError::IoErr(error)
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(error: serde_json::Error) -> Self {
        ServerError::SerdeStrErr(error)
    }
}

impl From<rusqlite::Error> for ServerError {
    fn from(error: rusqlite::Error) -> Self {
        ServerError::SqlErr(error)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for ServerError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        ServerError::OneshotRecvErr(error)
    }
}
