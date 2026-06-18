//Generate ECC key pair.
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
};
use esp_hal::{
    rng::Trng
};

pub fn gen_key_pair() -> ([u8; 32], [u8; 33]){
    //unwrap is safe here because trng source is set in main
    let mut trng = Trng::try_new().unwrap();
    let mut vkey_output = [0u8; 33];
    let mut skey_output = [0u8; 32];

    //set and serialize signing key
    let signing_key = SigningKey::random(&mut trng); 
    let serialized_skey = SigningKey::to_bytes(&signing_key);
    skey_output.copy_from_slice(&serialized_skey);


    //set and serialize verifying key
    let verifying_key = VerifyingKey::from(&signing_key);
    let serialized_vkey = VerifyingKey::to_encoded_point(&verifying_key, true);

    //convert s_vkey to byte array as_bytes returns &[u8]
    let serialized_vkey_bytes = serialized_vkey.as_bytes(); 
    vkey_output.copy_from_slice(serialized_vkey_bytes);

    //set signing and verifying key to nvs
    (skey_output, vkey_output)
}
