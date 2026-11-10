use crate::common::data_structs::*;

//states 
pub enum GlobalStates {
    IsProvisioned,
    StandardComm, 
    Enrollment, 
}

//provision enum 
pub enum ProvisionStatus {
    Provisioned,
    NotProvisioned,
    NotSet, 
}

//wifi config enum
#[derive(Clone, Debug)]
pub enum WifiConfigStatus {
    Up, 
    Down
}

//sendsteps enum used for gsc to communication with wtc  
#[derive(Debug)]
pub enum SendSteps {
    InitialEnrollment(SendPacketInitialEnrl), 
}


//wifi response for Wifi task to communicate with Global state communicator
#[derive(Debug, PartialEq)]
pub enum WifiResponse {
    InitialEnrollment(ReceivePacketInitialEnrl),
    Failure 
}

