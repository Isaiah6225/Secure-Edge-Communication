use std::{io, env, fmt, fmt::Display};
use crate::common::enums::DBOps;

//error enum
#[derive(Debug)]
pub enum ServerError {
    VarErr(env::VarError),
    IoErr(io::Error),
    SerdeStrErr(serde_json::Error),
    SqlErr(rusqlite::Error),
    OneshotRecvErr(tokio::sync::oneshot::error::RecvError),
    MpscSendErr(tokio::sync::mpsc::error::SendError<DBOps>),
    EnrollmentClosedErr,
    CheckDeviceIDErr,
}

impl Display for ServerError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        if let ServerError::EnrollmentClosedErr = self {
            write!(f, "enrollment window closed")
        }
        */
        match self {
            ServerError::EnrollmentClosedErr => write!(f, "enrollment window closed"),
            ServerError::CheckDeviceIDErr => write!(f, "check device id failed"),
            _=>Ok(())
        }
    }
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

impl From<tokio::sync::mpsc::error::SendError<DBOps>> for ServerError {
    fn from(error: tokio::sync::mpsc::error::SendError<DBOps>) -> Self {
        ServerError::MpscSendErr(error)
    }
}
