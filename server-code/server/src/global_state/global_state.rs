use crate::{
    networking::conn,
    common::{
        structs::{
            TimeConfig,
        },
        enums::{
            GlobalStatesEnrollment,
            EnrollmentWindowStatus,
        },
    },
    enrollment_checks::enrollment_time,
};
use tokio::time::{Duration, sleep};


pub fn manage_global_state() {
    let mut state = GlobalStatesEnrollment::AwaitRequest;
    let (mut check_window, sleep_time) = enrollment_time::check_window();


    loop {
        match check_window {
            EnrollmentWindowStatus::OpenEnrollment => {
                println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window open await request.");
                state = GlobalStatesEnrollment::AwaitRequest;
                
                match state {
                    GlobalStatesEnrollment::AwaitRequest => {   
                        println!("[GlobalStatesEnrollment::AwaitRequest] awaiting request.");
                        if let Err(e) = conn::tcp_listen(){
                            println!("{}", e);
                        };

                    }

                    GlobalStatesEnrollment::RespondInital | GlobalStatesEnrollment::FinalVerification => todo!()
                }
            }

            EnrollmentWindowStatus::ClosedEnrollment => {
                println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window closed sleeping process until opened ");
                sleep(Duration::from_secs(sleep_time));
                (check_window, _) = enrollment_time::check_window();
                
            }
        }
    }
}
