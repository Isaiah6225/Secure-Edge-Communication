use std::{io, env};

//error enum
#[derive(Debug)]
pub enum ServerError {
    VarErr(env::VarError),
    IoErr(io::Error),
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

