#![no_std]
use esp_hal::rng::{
    Trng, TrngError
};
use rand_core_old::{RngCore as RngCoreOld, CryptoRng as CryptoRngOld}; 
use rand_core_new::RngCore as RngCoreNew;

// p256 and esphal both use rand core on different versions (esp_hal v0.9.5 and p256 v0.6.4)
// creating wrapper to match version implmentations
struct TrngWrapper(Trng);
impl RngCoreOld for TrngWrapper {
    fn next_u32(&mut self) -> u32{
        RngCoreNew::next_u32(&mut self.0) 
    }

    fn next_u64(&mut self) -> u64 {
        RngCoreNew::next_u64(&mut self.0)
    }

    fn fill_bytes(&mut self, dst: &mut[u8]){
        RngCoreNew::fill_bytes(&mut self.0, dst)
    }

    fn try_fill_bytes(&mut self, dst: &mut[u8]) -> Result<(), rand_core_old::Error>{
        RngCoreNew::fill_bytes(&mut self.0, dst);
        Ok(())
    }
}
impl CryptoRngOld for TrngWrapper {}

//VerifyingKey Encoded Point type simplified
//type VerifyingKeyRet = sec1::point::EncodedPoint<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

//error enum
#[derive(Debug)]
enum NodeError {
    Rng(TrngError), 
}

impl From<TrngError> for NodeError {
    fn from(error: TrngError) -> Self {
        NodeError::Rng(error)
    }
}

//Packet Struct 
pub struct SendPacketInital  {
    pub serialized_vkey: f32,
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32, 
}

//Imports 
pub mod nonce;
pub mod boot;
pub mod enroll_device;

