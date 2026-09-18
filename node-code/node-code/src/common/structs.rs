use embassy_net::{
    Stack,
    Runner
};
use embassy_sync::{
    channel::{Sender, Receiver},
    blocking_mutex::raw::CriticalSectionRawMutex
};
use embassy_time::{Duration, Timer};
use esp_nvs::{
    Nvs,
    Key,

    platform::Platform,
};
use esp_hal::{
    rng::TrngSource,
};
use core::{
    fmt::Display,
    fmt,
};
use alloc::{
    string::String, 
    vec::Vec,
};
use esp_radio::wifi::Interface;
use crate::{
    enrollment::format_enrollment_initial,
    common::{
        error::NodeError,
        enums::{EnrollmentSteps, WifiCommand},
    },
    boot::{
        read_id,
    },
    nonce::gen_nonce,
};
use log::info;
use serde::Deserialize;
use serde_big_array::BigArray;
use p256::ecdsa::VerifyingKey;

extern crate alloc;

//Storage Service API
pub struct StorageManager<T: Platform>{
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
    pub fn set_provision_flag(&mut self) -> Result<(), NodeError> {
        let namespace = const {Key::from_str("pro_data")};
        let key = const {Key::from_str("is_pro")};
        let value: u8 = 0;

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
    pub fn get_ecc(&mut self) -> Result<([u8; 32], [u8; 33]), NodeError> {
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
    pub fn set_ecc(&mut self, gen_priv_key: &[u8; 32], gen_pub_key: &[u8; 33]) -> Result<(), NodeError> {
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
    trng_source: TrngSource<'static>, 
}

impl WifiManager {
    pub fn new(
        stack: Stack<'static>, 
        trng_source: TrngSource<'static> 
    ) -> Self {
        Self { stack: stack, trng_source: trng_source }
    }

    pub fn gen_enrollment_initial_confirmation(&self, is_valid: u8) -> SendConfirmationEnrl {
        let command = format_enrollment_initial::format_enrollment_initial_confirmation(is_valid);
        info!("[WifiManager::gen_enrollment] generated enrollment confirmation packet and returning it");
        command
    }

    pub fn gen_enrollment_initial(&self, sv_key_bytes: [u8; 33]) -> SendPacketInitialEnrl {
        let mac = read_id::read_mac();
        let nonce = gen_nonce::gen_nonce();
        let header_byte: u8 = 0;

        let command = format_enrollment_initial::format_enrollment_initial(header_byte, mac, sv_key_bytes, nonce);
        info!("[WifiManager::gen_enrollment] generated enrollment packet and returning it.");
        command 
    }

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

//Global State Communicator Manager API
pub struct GSCManager {
    gsc_sender_handle: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 8>,
    wtc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, WifiCommand, 8>,
}

impl GSCManager {
    pub fn new(
        gsc_sender_handle: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 8>,
        wtc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, WifiCommand, 8>, 
    ) -> Self {
        Self { gsc_sender_handle: gsc_sender_handle, wtc_receiver_handle: wtc_receiver_handle }
    }

    pub async fn send_enrollment(&self, enrollment_steps: &EnrollmentSteps) {
        info!("[GSCManager::send_enrollment]");
        match enrollment_steps {
            EnrollmentSteps::Enrollment(pub_key) => {
                info!("[GSCManager::send_enrollment] sending ENROLLMENT request to wifi_task.");
                self.gsc_sender_handle.send(EnrollmentSteps::Enrollment(*pub_key)).await;
            }, 
            EnrollmentSteps::VerifyKeys => {} 
        }
    }

    pub async fn receive_enrollment(&self) -> WifiCommand {
        info!("[GSCManager::receive_enrollment]");
        let wt_response = self.wtc_receiver_handle.receive().await;
        match wt_response {
            WifiCommand::Failure => {
                info!("[GSCManager::receive_enrollment] wifi_task sent failure returning EnrollmentSteps::Enrollment");
                return WifiCommand::Failure;
            }

            WifiCommand::Success => {
                info!("[GSCManager::receive_enrollment] wifi_task sent succcess");
                return WifiCommand::Success;
            }
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
    
    //compare server public key to received public key
    pub fn compare_pub_key(&self, received_pub_key: [u8; 65]) -> u8 {
        let mut server_vkey_output = [0u8; 65];
        let server_vkey_bytes = self.server_pub_key.to_sec1_bytes();
        server_vkey_output.copy_from_slice(&server_vkey_bytes);
        if server_vkey_output == received_pub_key {
            return 0
        } else {
            return 1 
        }
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await;
}

// Data structs 

#[derive(Debug, Deserialize)]
pub struct ReceivePacketFinVeri {
    #[serde(rename = "signature_bytes", with = "BigArray")]
    pub signature_bytes: [u8; 64],
    #[serde(rename = "signature_base", with = "BigArray")]
    pub signature_base: [u8; 1500],
    #[serde(rename = "server_challenge")]
    pub server_challenge: u32,
}

impl ReceivePacketFinVeri{
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}

pub struct SendConfirmationEnrl {
    pub is_valid: u8,
}

impl Display for SendConfirmationEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "is_valid: {:?}", self.is_valid)
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceivePacketInitialEnrl {
    #[serde(rename = "server_pub_key", with = "BigArray")]
    pub server_pub_key: [u8; 65],
}

impl ReceivePacketInitialEnrl {
    pub fn new<T: AsRef<str>>(string: T) -> Result<Self, NodeError>{
        let res = serde_json::from_str(string.as_ref())?;
        Ok(res)
    }
}


//Enrollment Packets Struct 
#[derive(Debug)]
pub struct SendPacketInitialEnrl {
    pub header_byte: u8,
    pub serialized_vkey: [u8; 33],
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32, 
}

impl Display for SendPacketInitialEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serialized_vkey: {:?}, dev_mac_add: {:?}, device_nonce: {}", self.serialized_vkey, self.dev_mac_add, self.device_nonce)
    }
}
