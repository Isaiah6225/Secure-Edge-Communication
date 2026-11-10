use aes_gcm::{
    aead::{Key as AEADKey, KeyInit},
    Aes128Gcm, Nonce, Tag, AeadInOut,
};
use sha2::Sha256;
use hkdf::GenericHkdf;
use inout::InOutBuf;
use embassy_net::{
    Stack,
    Runner
};
use embassy_time::{Duration, Timer};
use embassy_net::tcp::TcpSocket;
use esp_nvs::{
    Nvs,
    Key,
    platform::Platform,
};
use esp_hal::{rng::{TrngSource, Trng}};
use heapless::String as HeaplessString;
use alloc::{string::String, vec::Vec,};
use esp_radio::wifi::Interface;
use crate::{
    common::error::NodeError,
    boot::read_id,
};
use log::info;
use p256::{
    elliptic_curve::Generate, 
    ecdsa::{VerifyingKey, SigningKey, Signature, signature::{Verifier, Signer}},
    Sec1Point, PublicKey,
    ecdh::EphemeralSecret
};

extern crate alloc;

//Storage Service API
pub struct StorageManager<T: Platform> {
    pub handle: Nvs<T>, 
} 


impl<T: Platform> StorageManager<T> {
    pub fn new(handle: Nvs<T>) -> Self {
        Self { handle: handle } 
    }

    
    //get provision flag from nvs
    pub fn get_provision_flag(&mut self) ->  Result<u8, NodeError> {
        let namespace = const {Key::from_str("pro_data")};
        let key = const {Key::from_str("is_pro")}; 

        let provision: u8 = self.handle.get(&namespace, &key)?;
        Ok(provision)
    }

    //set provision to nvs
    pub fn set_provision_flag(&mut self, value: u8) -> Result<(), NodeError> {
        let namespace = const {Key::from_str("pro_data")};
        let key = const {Key::from_str("is_pro")};

        self.handle.set(&namespace, &key, value)?;
        Ok(())
    }

    //get ecdsa pub key from nvs
    pub fn get_ecdsa_pub(&mut self) -> Result<[u8; 33], NodeError> {
        let namespace = const {Key::from_str("ecdsa_keys")};

        let pub_key = const {Key::from_str("pub")};

        let pub_key_value: Vec<u8> = self.handle.get(&namespace, &pub_key)?;
        let final_pub_key_value: [u8; 33] = pub_key_value.try_into().map_err(|v: Vec<u8>| NodeError::InvalidKeyLength(v.len()))?;

        Ok(final_pub_key_value)
    }

    //get ecdsa key pair from nvs
    pub fn get_ecdsa(&mut self) -> Result<([u8; 32], [u8; 33]), NodeError> {
        let namespace = const {Key::from_str("ecdsa_keys")};
        
        let priv_key = const {Key::from_str("priv")};
        let pub_key = const {Key::from_str("pub")};

        let priv_key_value: Vec<u8> = self.handle.get(&namespace, &priv_key)?;
        let pub_key_value: Vec<u8> = self.handle.get(&namespace, &pub_key)?; 

        let final_priv_key_value: [u8; 32] = priv_key_value.try_into().map_err(|v: Vec<u8>| NodeError::InvalidKeyLength(v.len()))?;
        let final_pub_key_value: [u8; 33] = pub_key_value.try_into().map_err(|v: Vec<u8>| NodeError::InvalidKeyLength(v.len()))?;

        Ok((final_priv_key_value, final_pub_key_value))
    }

    //set ecdsa key pair from nvs 
    pub fn set_ecdsa(&mut self, gen_priv_key: &[u8; 32], gen_pub_key: &[u8; 33]) -> Result<(), NodeError> {
        let namespace = const {Key::from_str("ecdsa_keys")};
        
        let priv_key = const {Key::from_str("priv")};
        let pub_key = const {Key::from_str("pub")};
        
        //not set yet figuring out 
        let priv_key_value: &[u8] = gen_priv_key;
        let pub_key_value: &[u8] = gen_pub_key;

        self.handle.set(&namespace, &priv_key, priv_key_value)?;
        self.handle.set(&namespace, &pub_key, pub_key_value)?; 
        Ok(())
    }
    
    //get saved server verifying key (saved with script, safe to get here)
    pub fn get_server_verifying_key(&mut self) -> Result<String, NodeError> {
        let namespace = const {Key::from_str("server_pub_key")};
        let key = const {Key::from_str("server_data")};
        let server_pub_get: String = self.handle.get(&namespace, &key)?;
        Ok(server_pub_get)
    }
}

//Wifi Manager API 
pub struct WifiManager{
    pub stack: Stack<'static>,
}

impl WifiManager {
    pub fn new(
        stack: Stack<'static>, 
    ) -> Self {
        Self { stack: stack }
    }

    pub async fn retry_send_3(send_buffer: &HeaplessString<1024>, tcp_socket: &mut TcpSocket<'_>) {
        for _i in 0..2{
            let init_request = tcp_socket.write(send_buffer.as_bytes()).await;
            if let Ok(usize) = init_request {
                return;
            }
        }
    }
    
    //clear buffer 
    pub fn clear_buffer<'a>(buffer: &'a mut [u8]) {
        for x in buffer {
            *x = 0;
        }
    }
    
    /*
    pub fn gen_secure_key_setup<'a>(a_tag: &'a mut [u8], nonce: [u8; 12], encrypted_buf: &'a mut [u8]) -> SendPacketSecureKS<'a> {
        let header_byte: u8 = 113;
        let command = format_secure_session::format_secure_key_setup(header_byte, nonce, a_tag, encrypted_buf);
        info!("[WifiManager::gen_secure_session] generated secure session initial packet and returning it");
        command
    }

    pub fn gen_secure_session_init(ecdh_pub_key: &[u8; 65]) -> SendPacketSecSesInit {
        let device_id = read_id::read_mac();
        let nonce = gen_nonce::gen_nonce();
        let header_byte: u8 = 1;

        let command = format_secure_session::format_secure_session_init(header_byte, nonce, &ecdh_pub_key, device_id);
        info!("[WifiManager::gen_secure_session] generated secure session initial packet and returning it");
        command
    }

    pub fn gen_enrollment_final(server_challenge: u32, device_signature_bytes: [u8; 64]) -> SendPacketFinalVerification {
        let header_byte: u8 = 101;
        let nonce = gen_nonce::gen_nonce();

        let command = format_enrollment::format_enrollment_final_verification(header_byte, device_signature_bytes, server_challenge, nonce);
        info!("[WifiManager::gen_enrollment] generated enrollment final verification packet and returning it");
        command
    }

    pub fn gen_enrollment_initial_confirmation(is_valid: u8) -> SendConfirmationEnrl {
        let header_byte: u8 = 100;
        let command = format_enrollment::format_enrollment_initial_confirmation(is_valid, header_byte);
        info!("[WifiManager::gen_enrollment] generated enrollment confirmation packet and returning it");
        command
    }

    pub fn gen_enrollment_initial(hma: &[u8; 6], sv_key_bytes: &[u8; 33], nonce: &u32) -> SendPacketInitialEnrl {
        let header_byte: u8 = 0;

        let command = format_enrollment::format_enrollment_initial(header_byte, hma, sv_key_bytes, nonce);
        info!("[WifiManager::gen_enrollment] generated enrollment packet and returning it.");
        command 
    }
    */

    pub async fn check_stack(&self) {
        info!("[WifiManager::check_stack] checking to see if the stack is up.");
        loop {
            //check if a connection's been made on the link layer 
            if self.stack.is_link_up() {
                info!("[WifiManager::check_stack] stack link is up.");
                break;
            }
            info!("[WifiManager::check_stack] stack link is not up. retrying.");
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}


//Crypto API 
pub struct CryptoClient {
    server_pub_key: VerifyingKey,
}
impl CryptoClient {
    pub fn new(server_pub_key: VerifyingKey) -> Self {
        Self { server_pub_key: server_pub_key }
    }
    
    //compare orignal nonce with server sent nonce
    pub fn compare_nonce(og_nonce: &u32, sent_nonce: &u32) -> u8{
        if og_nonce == sent_nonce {
            return 0
        } else {
            return 1
        }
    }
    
    //check signautre based on signature base 
    pub fn check_server_signature(&self, signature_bytes: &[u8; 64], signature_base: &Vec<u8>) -> Result<(), NodeError>{
        let server_signature = Signature::from_slice(signature_bytes)?;
        self.server_pub_key.verify(signature_base, &server_signature)?;
        Ok(())
    }
    
    //sign received server challenge
    pub fn sign_server_challenge(server_challenge: &u32, device_signing_key: &[u8; 32]) -> Result<[u8; 64], NodeError>{
        let mut signature_output = [0u8; 64];
        let server_challenge_bytes = server_challenge.to_be_bytes();

        let device_signing_key = SigningKey::from_slice(device_signing_key)?;
        let signature: Signature = device_signing_key.sign(&server_challenge_bytes);

        let signature_bytes = &signature.to_vec();
        signature_output.copy_from_slice(signature_bytes);
        Ok(signature_output)
    }

    //check if final verification is complete
    pub fn is_verify(is_verify: &u8) -> Result<(), NodeError>{
        if *is_verify == 0 {
            Ok(())
        } else {
            return Err(NodeError::FinalVerificationCleanUpErr)
        }
    }

    pub fn gen_session_key<'a>(&self, ephemeral_secret: EphemeralSecret, server_ecdh: [u8; 65], data: &'a mut [u8]) -> Result<(Tag, &'a mut [u8], [u8; 12]), NodeError> {
        //convert server ecdsa verifying key to bytes 
        let mut vkey_ecdsa_bytes= [0u8; 65];
        let server_ecdsa_key = self.server_pub_key.to_sec1_bytes();
        vkey_ecdsa_bytes.copy_from_slice(&server_ecdsa_key);

        //get shared secret from server public key
        let server_ecdh_public = PublicKey::from_sec1_bytes(&server_ecdh)?;
        let shared_secret = ephemeral_secret.diffie_hellman(&server_ecdh_public);

        //generate OKM with device ID and shared secret
        let device_id = read_id::read_mac();
        let mut okm = [0u8; 16];

        let hk: GenericHkdf<_> = shared_secret.extract::<Sha256>(Some(&device_id));
        hk.expand(&device_id, &mut okm)?;

        //generate aes gcm key and cipher from okm
        //unwrap is safe here because trng source is set in main
        let mut trng = Trng::try_new().expect("error: cannot access trng");
        let nonce = Nonce::generate_from_rng(&mut trng); 
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(nonce.as_slice());
        
        let cipher = Aes128Gcm::new_from_slice(&okm)?;
        let mut a_tag = cipher.encrypt_inout_detached(&nonce, &vkey_ecdsa_bytes, data.into())?;

        Ok((a_tag, data, nonce_bytes))
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await;
}

