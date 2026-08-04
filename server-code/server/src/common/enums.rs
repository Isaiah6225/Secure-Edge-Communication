use tokio::{
    net::TcpStream,
    sync::{
        oneshot::Sender,
    }
};
use crate::common::structs::Device;

#[derive(Debug)]
pub enum GlobalStatesEnrollment{
    ClosedEnrollment(TcpStream),
    RespondInitial(TcpStream), 
    FinalVerification(TcpStream),
    Transitioning, 
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
}

pub enum ResultDBOps {
    Success, 
    Error
}
