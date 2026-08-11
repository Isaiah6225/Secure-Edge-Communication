use tokio::{
    net::TcpStream,
    sync::{
        mpsc::Sender,
    }
};
use strum::AsRefStr;
use crate::common::structs::Device;

#[derive(Debug)]
pub enum GlobalStatesEnrollment{
    RespondInitial(TcpStream), 
    FinalVerification(TcpStream),
}

#[derive(Debug)]
pub enum TimeStatus {
    Open,
    Closed
}


#[derive(Debug)]
pub enum MainFlow {
    Enroll(TcpStream, Device),
    Drop,
}

#[derive(Debug, PartialEq)]
pub enum EnrollmentCheck {
    Success, 
    Error 
}

pub enum DBOps {
    CheckDevice(Sender<ResultDBOps>, Device),
    SaveDevice(Sender<ResultDBOps>, Device, DBSave),
}

#[derive(AsRefStr, Debug)]
pub enum DBSave {
    Pending,
    Verified,
    Rejected
}

pub enum ResultDBOps {
    Success, 
    Error(Device)
}
