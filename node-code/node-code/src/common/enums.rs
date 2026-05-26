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

//channel communication
pub enum WifiCommand {
    SendEnrollment,
    Connect, 
    AwaitCommand,
}
