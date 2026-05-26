//Generate ECC key pair.
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use esp_hal::{
    rng::{Trng, TrngSource, TrngError}
};
use log::info;
use crate::{
    common::structs::TrngWrapper, 
    common::error::NodeError
};



pub fn gen_key_pair(_trng_source: &TrngSource<'static>) -> Result<[u8; 33], NodeError>{
    //set Trng and pass to wrapper 
    let trng = Trng::try_new();
    let mut vkey_output = [0u8; 33];
    let mut wrapper = TrngWrapper(match trng {
        Ok(trng) => trng, 
        Err(e) => {
            info!("{:?}", e);
            return Err(NodeError::Rng(TrngError::TrngSourceNotEnabled))
        },
    });
    
    //set signing key, and verifying key
    let signing_key = SigningKey::random(&mut wrapper); 
    let _serialized_skey = SigningKey::to_bytes(&signing_key);

    //serialize signing key and verifying key
    let verifying_key = VerifyingKey::from(&signing_key);
    let serialized_vkey = VerifyingKey::to_encoded_point(&verifying_key, true);
    //as_bytes returns &[u8]
    let serialized_vkey_bytes = serialized_vkey.as_bytes(); 
    vkey_output.copy_from_slice(serialized_vkey_bytes);

    //info!("Serialized Signing Key: {:?} and Serilaized Verifying Key: {:?}", serialized_skey, serialized_vkey_byte_arr);
    Ok(vkey_output)
}
