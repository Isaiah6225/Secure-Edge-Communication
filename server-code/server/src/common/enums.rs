#[derive(Debug)]
pub enum GlobalStatesEnrollment {
    AwaitRequest,
    RespondInital, 
    FinalVerification,
}

#[derive(Debug)]
pub enum EnrollmentWindowStatus {
    OpenEnrollment,
    ClosedEnrollment
}
