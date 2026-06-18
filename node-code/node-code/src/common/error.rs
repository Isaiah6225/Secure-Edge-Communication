use esp_hal::rng::{
    TrngError
};

//error enum
#[derive(Debug)]
pub enum NodeError {
    Rng(TrngError),
    NvsError(esp_nvs::error::Error),
    InvalidKeyLength(usize),
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

/*
impl From<InvalidKeyLength> for NodeError {
    fn from(error: InvalidKeyLength) -> Self {
        NodeError::(InvalidKeyLength)
    }
}
*/
