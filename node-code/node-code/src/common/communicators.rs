use embassy_sync::{
    channel::{Sender, Receiver},
    blocking_mutex::raw::CriticalSectionRawMutex
};
use crate::common::{
    enums::{SendSteps, WifiResponse},
    data_structs::*,
};
use log::info;

//Global State Communicator Manager API
pub struct WTCManager {
    gsc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, SendSteps, 8>,
    wtc_sender_handle: Sender<'static, CriticalSectionRawMutex, WifiResponse, 8>,
}

impl WTCManager {
    pub fn new(
        gsc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, SendSteps, 8>,
        wtc_sender_handle: Sender<'static, CriticalSectionRawMutex, WifiResponse, 8>, 
    ) -> Self {
        Self { gsc_receiver_handle: gsc_receiver_handle, wtc_sender_handle: wtc_sender_handle }
    }

    pub async fn send_initial_enrollment(&self, wifi_res: WifiResponse) {
        info!("[WTCManager::send_initial_enrollment]");
        match wifi_res {
            WifiResponse::InitialEnrollment(init_packet_recv) => {
                info!("[GSCManager::send_initial_enrollment] sending ENROLLMENT request to wifi_task.");
                self.wtc_sender_handle.send(WifiResponse::InitialEnrollment(init_packet_recv)).await;
            }, 
            WifiResponse::Failure => (),
        }
    }
    
    pub async fn receive_initial_enrollment(&self) -> SendSteps {
        info!("[GSCManager::receive_initial_enrollment]");
        let gsc_response = self.gsc_receiver_handle.receive().await;
        return gsc_response
    }
}

//Global State Communicator Manager API
pub struct GSCManager {
    gsc_sender_handle: Sender<'static, CriticalSectionRawMutex, SendSteps, 8>,
    wtc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, WifiResponse, 8>,
}

impl GSCManager {
    pub fn new(
        gsc_sender_handle: Sender<'static, CriticalSectionRawMutex, SendSteps, 8>,
        wtc_receiver_handle: Receiver<'static, CriticalSectionRawMutex, WifiResponse, 8>, 
    ) -> Self {
        Self { gsc_sender_handle: gsc_sender_handle, wtc_receiver_handle: wtc_receiver_handle }
    }

    pub async fn send_initial_enrollment(&self, send_steps: SendSteps) {
        info!("[GSCManager::send_initial_enrollment]");
        match send_steps{
            SendSteps::InitialEnrollment(init_packet) => {
                info!("[GSCManager::send_initial_enrollment] sending ENROLLMENT request to wifi_task.");
                self.gsc_sender_handle.send(SendSteps::InitialEnrollment(init_packet)).await;
            }
        }
    }
    
    pub async fn receive_initial_enrollment(&self) -> WifiResponse {
        info!("[GSCManager::receive_initial_enrollment]");
        let wt_response = self.wtc_receiver_handle.receive().await;
        return wt_response
        /*
        match wt_response {
            WifiResponse::Failure => {
                info!("[GSCManager::receive_initial_enrollment] wifi_task sent failure");
                return WifiResponse::Failure;
            },
            
            WifiResponse::InitialEnrollment(init_packet_recv) => {
                info!("[GSCManager::receive_initial_enrollment] wifi task send initial enrollment data");
                return WifiResponse::InitialEnrollment(init_packet_recv); 
            },
        }*/
    }

    /*
    pub async fn send_session(&self, send_steps: &SendSteps) {
        info!("[GSCManager::send_secure_session]");
        match send_steps {
            SendSteps::StdComm(StdCommSteps::StandardCommunication) => {
                info!("[GSCManager::send_secure_session] sending STANDARD COMM request to wifi_task");
                self.gsc_sender_handle.send(SendSteps::StdComm(StdCommSteps::StandardCommunication)).await;
            },
            SendSteps::Enroll(EnrollmentSteps::VerifyKeys) | SendSteps::Enroll(EnrollmentSteps::Enrollment(..)) => {} 
        }
    }

    pub async fn receive_session(&self) -> WifiCommand {
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
    */
}

