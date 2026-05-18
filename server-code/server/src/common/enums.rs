use tokio::net::TcpStream;
use crate::common::structs::EnrollmentReceiveInital;

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
    Enroll(TcpStream),
    Drop,
}

