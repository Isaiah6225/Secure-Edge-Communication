use esp_hal::rng::{
    TrngError
};

//error enum
#[derive(Debug)]
pub enum NodeError {
    Rng(TrngError),
    NvsError(esp_nvs::error::Error),
    InvalidKeyLength(usize),
    SerdeErr(serde_json::Error),
    CryptoErr(p256::ecdsa::Error)
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
