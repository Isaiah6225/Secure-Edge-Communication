use p256::{
    ecdh::EphemeralSecret,
    Sec1Point, PublicKey, 
    elliptic_curve::Generate
};

use esp_hal::rng::Trng;

pub fn gen_ecdh_pub() -> [u8; 65] {
    let mut trng = Trng::try_new().expect("error: cannot access trng"); 
    let mut ecdh_pub_output = [0u8; 65];

    let secret = EphemeralSecret::generate_from_rng(&mut trng);
    let pub_key = Sec1Point::from(secret.public_key());

    let pub_key_bytes = pub_key.as_bytes();
    ecdh_pub_output.copy_from_slice(pub_key_bytes);
    ecdh_pub_output
}
