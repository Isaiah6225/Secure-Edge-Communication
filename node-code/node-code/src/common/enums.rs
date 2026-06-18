use crate::common::structs::SendPacketInitialEnrl;

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

//check whether ecc key pair is set.
pub enum EccStatus {
    Set,
    NotSet,
}

pub enum EnrollmentError {
    Success, 
    Error
}

//wifi config enum
#[derive(Clone)]
pub enum WifiConfigStatus {
    Up, 
    Down
}

//enrollment sub steps
//TODO every enum below this can be refactor to be one enum for simplicity
pub enum EnrollmentSteps {
    VerifyKeys,
    Initial([u8; 33]),
    FinalVerification
}

//channel communication
/*
#[derive(Debug)]
pub enum WifiCommand {
    SendEnrlInitial,
    SendFinalVerification,
}
*/

//wifi command for Wifi task to communicate with Global state communicator
pub enum WifiCommand {
    Initial, 
    FinalVerification, 
}

#[derive(Debug)]
pub enum WifiData {
    SendEnrlInitial(SendPacketInitialEnrl),
    ReceiveEnrl(),
    Connect, 
}
