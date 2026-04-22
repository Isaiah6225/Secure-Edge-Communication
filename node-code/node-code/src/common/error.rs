use esp_hal::rng::{
    Trng, TrngError
};


//error enum
#[derive(Debug)]
pub enum NodeError {
    Rng(TrngError),
    NvsError(esp_nvs::error::Error), 
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
