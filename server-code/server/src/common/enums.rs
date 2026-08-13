use tokio::{
    net::TcpStream,
    sync::{
        oneshot::Sender,
    }
};
use strum::AsRefStr;
use crate::common::{
    structs::{Device, CheckDevicePayload, SaveDevicePayload},
    errors::ServerError,
};

#[derive(Debug)]
pub enum GlobalStatesEnrollment{
    RespondInitial(TcpStream), 
    FinalVerification(TcpStream),
}

#[derive(Debug)]
pub enum MainFlow {
    Enroll(TcpStream, Device),
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
