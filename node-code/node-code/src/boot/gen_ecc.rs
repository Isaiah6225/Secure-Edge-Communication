//Generate ECC key pair.
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use esp_hal::{
    rng::{Trng, TrngSource}
};
use log::info;
use crate::TrngWrapper;



pub fn gen_key_pair(_trng_source: TrngSource<'static>) {
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

    serialized_vkey 
}
