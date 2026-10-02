use esp_hal::rng::{
    TrngError
};
use core::{
    fmt, 
    fmt::Display
};

//error enum
#[derive(Debug)]
pub enum NodeError {
    Rng(TrngError),
    NvsError(esp_nvs::error::Error),
    SerdeErr(serde_json::Error),
    CryptoErr(p256::ecdsa::Error),
    InvalidKeyLength(usize),
    FinalVerificationCleanUpErr
}

impl Display for NodeError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeError::InvalidKeyLength(usize) => write!(f, "invalid key length"),
            NodeError::FinalVerificationCleanUpErr => write!(f, "server return final verification failed"), 
            _=> Ok(())
        }
    }
}

impl From<TrngError> for NodeError {
    fn from(error: TrngError) -> Self {
        NodeError::Rng(error)
    }
}

impl From<esp_nvs::error::Error> for NodeError {
    fn from(error: esp_nvs::error::Error) -> Self {
        NodeError::NvsError(error)
    }
}

impl From<serde_json::Error> for NodeError {
    fn from(error: serde_json::Error) -> Self {
        NodeError::SerdeErr(error)
    }
}

impl From<p256::ecdsa::Error> for NodeError {
    fn from(error: p256::ecdsa::Error) -> Self {
        NodeError::CryptoErr(error)
    }
}
/*
impl From<InvalidKeyLength> for NodeError {
    fn from(error: InvalidKeyLength) -> Self {
        NodeError::(InvalidKeyLength)
    }
}
*/
