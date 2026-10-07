use tokio::{
    net::TcpStream,
    sync::{
        oneshot::Sender,
    }
};
use strum::AsRefStr;
use crate::common::{
    structs::{UpdateDeviceStatusPayload, CheckDevicePayload, SaveDevicePayload, DeviceEnrl, DeviceStdComm, IsValid, FinalVeriCleanUp},
    errors::ServerError,
};

#[derive(Debug)]
pub enum GlobalStatesEnrollment{
    RespondInitial(TcpStream), 
    FinalVerification(TcpStream),
}

#[derive(Debug)]
pub enum MainFlow {
    Enroll(TcpStream, ParsedStruct),
    Drop,
}

pub enum DBOps {
    CheckDevice(Sender<Result<(), ServerError>>, CheckDevicePayload),
    SaveDevice(Sender<Result<(), ServerError>>, SaveDevicePayload),
    UpdateDeviceStatus(Sender<Result<(), ServerError>>, UpdateDeviceStatusPayload)
}

#[derive(AsRefStr, Debug)]
pub enum DBSave {
    Pending,
    Verified,
    Rejected
}

#[derive(Debug)]
pub enum ParsedStruct {
    DeviceEnrlParsed(DeviceEnrl),
    DeviceReceiveInitialEnrlParsed(IsValid),
    DeviceStdCommParsed(DeviceStdComm), 
    DeviceFinalVerificationParsed(FinalVeriCleanUp)
}
