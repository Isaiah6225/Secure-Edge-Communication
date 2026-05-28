//generate device nonce using Trng from device
use esp_hal::{
    rng::{Trng, TrngSource, TrngError}
};
use log::info; 
use crate::common::{
    structs::TrngWrapper,
    error::NodeError
}; 


pub fn gen_nonce(trng_source: &TrngSource<'static>) -> u32{
    let trng = Trng::try_new().unwrap();
    let nonce = trng.random();
    nonce
}
