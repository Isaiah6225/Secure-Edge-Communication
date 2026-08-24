use tokio::{
    net::TcpStream,
    sync::{
        oneshot::Sender,
    }
};
use strum::AsRefStr;
use crate::common::{
    structs::{CheckDevicePayload, SaveDevicePayload, DeviceEnrl, DeviceStdComm},
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
    DeviceStdCommParsed(DeviceStdComm)
}
