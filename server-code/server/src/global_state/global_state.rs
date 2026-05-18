use crate::{
    common::{
        enums::{
            GlobalStatesEnrollment,
            TimeStatus
        },
    },
    enrollment_checks::enrollment_time,
};
use tokio::{
    net::TcpStream,
};
use std::mem;

pub async fn manage_enrollment(stream: TcpStream) {
    let (check_window, _) = enrollment_time::check_window();
    let mut state = GlobalStatesEnrollment::RespondInitial(stream);

    loop {
        let swap_stream = mem::replace(&mut state, GlobalStatesEnrollment::Transitioning);

        match swap_stream {
            GlobalStatesEnrollment::RespondInitial(stream) => {
                match check_window {
                    TimeStatus::Open => {
                        println!("[GlobalStatesEnrollment::RespondInital] checking packet then responding.");
                        println!("{:?}", stream);
                        state = GlobalStatesEnrollment::FinalVerification(stream);
                    }

                    TimeStatus::Closed => {
                        state = GlobalStatesEnrollment::ClosedEnrollment(stream);
                        println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window closed dropping connection");    
                    }
                }
            }


            GlobalStatesEnrollment::FinalVerification(stream) => {
                println!("[GlobalStatesEnrollment::FinalVerification] receiving final verification packet then responding.");
            },

            GlobalStatesEnrollment::ClosedEnrollment(_) | GlobalStatesEnrollment::Transitioning => todo!()
        }
    }
}
