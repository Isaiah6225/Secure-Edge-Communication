//Generate ECC key pair.
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use esp_hal::{
    rng::{Trng, TrngSource}
};
use rand_core_old::{RngCore as RngCoreOld, CryptoRng as CryptoRngOld}; 
use rand_core_new::RngCore as RngCoreNew;
use log::info;

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

#[embassy_executor::task]
pub async fn priv_key(_trng_source: TrngSource<'static>) {
    //set Trng and pass to wrapper 
    let trng = Trng::try_new();
    let mut wrapper = TrngWrapper(match trng {
        Ok(trng) => trng, 
        Err(e) => {
            info!("{:?}", e);
            return
        },
    });
    
    //set signing key, and verifying key
    let signing_key = SigningKey::random(&mut wrapper); 
    let serialized_skey = SigningKey::to_bytes(&signing_key);
    
    //serialize signing key and verifying key
    let verifying_key = VerifyingKey::from(&signing_key);
    let serialized_vkey = VerifyingKey::to_encoded_point(&verifying_key, true);

    info!("Serialized Signing Key: {:?} and Serilaized Verifying Key: {:?}", serialized_skey, serialized_vkey);
}
