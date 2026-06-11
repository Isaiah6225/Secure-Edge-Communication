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
    Initial,
    FinalVerification
}

//channel communication
#[derive(Debug)]
pub enum WifiCommand {
    SendEnrlInitial,
    SendFinalVerification,
}

#[derive(Debug)]
pub enum WifiData {
    SendEnrlInitial(SendPacketInitialEnrl),
    ReceiveEnrl(),
    Connect, 
}
