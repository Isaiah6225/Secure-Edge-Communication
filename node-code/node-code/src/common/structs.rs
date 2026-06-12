use embassy_net::{
    Stack,
    Runner
};
use embassy_sync::{
    channel::{Sender, Receiver},
    blocking_mutex::raw::CriticalSectionRawMutex
};
use embassy_time::{Duration, Timer};
use embassy_net::{tcp::TcpSocket};
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
use esp_hal::rng::Trng;
use esp_radio::wifi::{
    WifiDevice
};
use crate::{
    enrollment::format_data,
    common::{
        error::NodeError,
        enums::{EnrollmentSteps, WifiData, EnrollmentError},
    },
};
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

//wifi functions and handle 
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

    pub fn gen_enrollment(&self, enrollment_steps: &EnrollmentSteps) -> WifiData {
        let command = format_data::format_enrollment(enrollment_steps);
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

pub struct GSCManager{
    gsc_sender: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
    gsc_receiver: Receiver<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
}

impl GSCManager {
    pub fn new(
        gsc_sender: Sender<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>,
        gsc_receiver: Receiver<'static, CriticalSectionRawMutex, EnrollmentSteps, 16>, 
    ) -> Self {
        Self { gsc_sender: gsc_sender, gsc_receiver: gsc_receiver }
    }

    pub async fn send_enrollment(&self, enrollment_steps: &EnrollmentSteps) {
        info!("[GSCManager::send_enrollment]");
        match enrollment_steps {
            EnrollmentSteps::Initial => {
                info!("[GSCManager::send_enrollment] sending INITIAL ENROLLMENT request to wifi_task.");
                self.gsc_sender.send(EnrollmentSteps::Initial).await;
            }, 

            EnrollmentSteps::FinalVerification => {
                info!("[GSCManager::send_enrollment] sending VERIFICATION ENROLLMENT request to wifi_task.");
                self.gsc_sender.send(EnrollmentSteps::FinalVerification).await;
            }
        }
    }

    pub async fn receive_enrollment(&self) -> EnrollmentSteps{
        info!("[GSCManager::receive_enrollment]");
        let wt_response = self.gsc_receiver.receive().await;
        wt_response
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

