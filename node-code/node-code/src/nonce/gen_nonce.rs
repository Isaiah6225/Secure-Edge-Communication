//generate device nonce using Trng from device
use esp_hal::{
    rng::Trng
};

pub fn gen_nonce() -> u32{
    //unwrap is safe here because trng source is set in main
    let trng = Trng::try_new().unwrap();
    let nonce = trng.random();
    nonce
}
