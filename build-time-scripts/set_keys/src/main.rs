use std::env; 
use base64ct::LineEnding;
use p256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    elliptic_curve::Generate,
    SecretKey,
    pkcs8::{EncodePrivateKey,EncodePublicKey},
};

fn main() {
    //server public/private key generation 
    let mut server_signing_key = SigningKey::generate(); 
    let server_verifying_key = server_signing_key.verifying_key();

    //write to pem 
    server_signing_key.write_pkcs8_pem_file("../priv_key.pem", LineEnding::LF);
    server_verifying_key.write_public_key_pem_file("../pub_key.pem", LineEnding::CR);

    //node public/private key generation
    let mut node_signing_key = SigningKey::generate(); 
    let node_verifying_key = node_signing_key.verifying_key();

    let node_signing_key_bytes: Vec<u8> = node_signing_key.to_bytes().to_vec();
    let node_verifying_key_bytes: Vec<u8> = node_verifying_key.to_sec1_bytes().to_vec();

    println!("node_signing_key: {:?}", node_signing_key_bytes);
    println!("node_verifying_key: {:?}", node_verifying_key_bytes);
}
