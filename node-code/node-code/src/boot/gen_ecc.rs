//Generate ECC key pair.
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use esp_hal::{
    rng::Trng
};

pub fn gen_key_pair() -> [u8; 33] {
    //unwrap is safe here because trng source is set in main
    let mut trng = Trng::try_new().unwrap();
    let mut vkey_output = [0u8; 33];
    /*
    let mut wrapper = TrngWrapper(match trng {
        Ok(trng) => trng, 
        Err(e) => {
            info!("{:?}", e);
            //throwing the error should be good here but need to think about it further. 
            //the trng source is set so it should never throw an error here.
            return Err(NodeError::Rng(TrngError::TrngSourceNotEnabled))
        },
    });
    */
    
    //set signing key, and verifying key
    let signing_key = SigningKey::random(&mut trng); 
    let _serialized_skey = SigningKey::to_bytes(&signing_key);

    //serialize signing key and verifying key
    let verifying_key = VerifyingKey::from(&signing_key);
    let serialized_vkey = VerifyingKey::to_encoded_point(&verifying_key, true);
    //as_bytes returns &[u8]
    let serialized_vkey_bytes = serialized_vkey.as_bytes(); 
    vkey_output.copy_from_slice(serialized_vkey_bytes);

    //info!("Serialized Signing Key: {:?} and Serilaized Verifying Key: {:?}", serialized_skey, serialized_vkey_byte_arr);
    vkey_output
}
