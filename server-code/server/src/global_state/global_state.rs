use crate::{
    common::{
        enums::{
            GlobalStatesEnrollment,
            TimeStatus,
            EnrollmentCheck,
            DBOps,
            ResultDBOps, 
            DBSave
        },
        structs::Device,
    },
    enrollment_checks::{
        enrollment_time,
        check_device_id
    },
    global_state::manage_db_request,
};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::Sender,
    }
};

pub async fn manage_enrollment(stream: TcpStream, data_parsed: Device, db_tx: Sender<DBOps>) {
    let (check_window, _) = enrollment_time::check_window();
    let mut state = Some(GlobalStatesEnrollment::RespondInitial(stream));
    loop {
        let swap_stream = state.take();
        match swap_stream {
            Some(GlobalStatesEnrollment::RespondInitial(stream)) => {
                match check_window {
                    TimeStatus::Open => {
                        println!("[GlobalStatesEnrollment::RespondInitial] checking packet then responding.");
                        let en_check = check_device_id::check_id(&data_parsed.device_id);

                        if en_check == EnrollmentCheck::Success {
                            manage_db_request::manage_check_dev(db_tx.clone(), data_parsed).await;

                        } else {
                            println!("[GlobalStatesEnrollment::RespondInitial] enrollment check failed moving to closing enrollment");
                            break;
                        }
                    }
                    TimeStatus::Closed => {
                        println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window closed dropping connection");    
                        break;
                    }
                }
            }
            Some(GlobalStatesEnrollment::FinalVerification(stream)) => {
                println!("[GlobalStatesEnrollment::FinalVerification] receiving final verification packet then responding.");
            },

           None => break,
        }
    }
}
