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
#[derive(Clone, Debug)]
pub enum WifiConfigStatus {
    Up, 
    Down
}

// send step 
pub enum SendSteps {
    Enroll(EnrollmentSteps),
    StdComm(StdCommSteps),
}

//enrollment sub steps
pub enum EnrollmentSteps {
    VerifyKeys,
    Enrollment([u8; 32], [u8; 33]),
}

//Standard Communication sub steps 
pub enum StdCommSteps {
    StandardCommunication([u8; 65])
}


//wifi command for Wifi task to communicate with Global state communicator
pub enum WifiCommand {
    Failure,
    Success
}

#[derive(Debug)]
pub enum WifiData {
    SendEnrlInitial(SendPacketInitialEnrl),
    ReceiveEnrl(),
    Connect, 
}
