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
use alloc::vec::Vec;
use esp_hal::rng::Trng;
use esp_radio::wifi::{
    WifiDevice
};
use crate::{
    enrollment::format_enrollment_initial,
    common::{
        error::NodeError,
        enums::{EnrollmentSteps, WifiData, WifiCommand},
    },
    boot::{
        read_id,
    },
    nonce::gen_nonce,
};
use rand_core_old::{RngCore as RngCoreOld, CryptoRng as CryptoRngOld}; 
use rand_core_new::RngCore as RngCoreNew;
use log::info;

extern crate alloc;

// p256 and esphal both use rand core on different versions (esp_hal v0.9.5 and p256 v0.6.4)
// creating wrapper to match version implmentations
pub struct TrngWrapper(pub Trng);
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

    //get ecc key pair from nvs
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

    //set ecc key pair from nvs 
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

    pub fn gen_enrollment_initial(&self, sv_key_bytes: [u8; 33]) -> SendPacketInitialEnrl {
        let mac = read_id::read_mac();
        let nonce = gen_nonce::gen_nonce();

        let command = format_enrollment_initial::format_enrollment_initial(mac, sv_key_bytes, nonce);
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
pub struct GSCManager{
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
            EnrollmentSteps::Initial(pub_key) => {
                info!("[GSCManager::send_enrollment] sending INITIAL ENROLLMENT request to wifi_task.");
                self.gsc_sender_handle.send(EnrollmentSteps::Initial(*pub_key)).await;
            }, 

            EnrollmentSteps::FinalVerification => {
                info!("[GSCManager::send_enrollment] sending VERIFICATION ENROLLMENT request to wifi_task.");
                self.gsc_sender_handle.send(EnrollmentSteps::FinalVerification).await;
            }

            EnrollmentSteps::VerifyKeys => {} 
        }
    }

    pub async fn receive_enrollment(&self) -> EnrollmentSteps {
        info!("[GSCManager::receive_enrollment]");
        let wt_response = self.wtc_receiver_handle.receive().await;
        match wt_response {
            WifiCommand::Initial => {
                info!("[GSCManager::receive_enrollment] wifi_task sent initial returning EnrollmentSteps::Initial");
                return EnrollmentSteps::VerifyKeys;
            }

            WifiCommand::FinalVerification => {
                info!("[GSCManager::receive_enrollment] wifi_task sent final verification returning EnrollmentSteps::FinalVerification");
                return EnrollmentSteps::FinalVerification;
            }
        }
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await;
}


//Enrollment Packets Struct 
#[derive(Debug)]
pub struct SendPacketInitialEnrl {
    pub serialized_vkey: [u8; 33],
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32, 
}

impl Display for SendPacketInitialEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serialized_vkey: {:?}, dev_mac_add: {:?}, device_nonce: {}", self.serialized_vkey, self.dev_mac_add, self.device_nonce)
    }
}

