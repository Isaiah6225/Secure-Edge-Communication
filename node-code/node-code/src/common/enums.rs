use crate::common::structs::SendPacketInitalEnrl;
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

//enrollment sub steps
pub enum EnrollmentSteps {
    Initial,
    FinalVerification
}

//channel communication
#[derive(Debug)]
pub enum WifiCommand {
    SendEnrlInitial(SendPacketInitalEnrl),
    Connect, 
    AwaitCommand,
}
