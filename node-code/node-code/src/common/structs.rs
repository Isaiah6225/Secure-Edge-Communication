use embassy_net::Stack;
use esp_nvs::{
    Nvs,
    Key,
    platform::Platform,
};
use core::{
    fmt::Display,
    fmt
};
use esp_hal::rng::Trng;
use esp_radio::wifi::{
    ClientConfig,
    WifiController,
};
use crate::common::error::NodeError;
use rand_core_old::{RngCore as RngCoreOld, CryptoRng as CryptoRngOld}; 
use rand_core_new::RngCore as RngCoreNew;
use log::info;

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

    pub fn get_provision_flag(&mut self) ->  Result<u8, NodeError> {
        let namespace = const {Key::from_str("pro_data")};
        let key = const {Key::from_str("is_pro")}; 

        let provision: u8 = self.handle.get(&namespace, &key)?;
        Ok(provision)
    }

    pub fn set_provision_flag(&mut self) -> Result<(), NodeError> {
        let namespace = const {Key::from_str("pro_data")};
        let key = const {Key::from_str("is_pro")};
        let value: u8 = 0;

        self.handle.set(&namespace, &key, value)?;
        Ok(())
    }
}

pub struct WifiManager{
    pub wifi_con: WifiController<'static>,
    //pub stack: &Stack <'static>,
}

impl WifiManager {
    pub fn new(wifi_con: WifiController<'static> /*stack: &Stack <'static>*/) -> Self{
        Self { wifi_con: wifi_con, /*stack: stack*/}
    }
    
    /*
    pub fn set_station_config() {
        
    } 

    pub fn set_net_stack() {

    }

    pub fn send_data() {

    }
    */
}


//Enrollment Packets Struct 
pub struct SendPacketInitalEnrl {
    pub serialized_vkey: [u8; 33],
    pub dev_mac_add: [u8; 6], 
    pub device_nonce: u32, 
}

impl Display for SendPacketInitalEnrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serialized_vkey: {:?}, dev_mac_add: {:?}, device_nonce: {}", self.serialized_vkey, self.dev_mac_add, self.device_nonce)
    }
}

