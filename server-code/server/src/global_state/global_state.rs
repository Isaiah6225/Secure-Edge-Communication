use crate::{
    common::{
        enums::{
            GlobalStatesEnrollment,
            TimeStatus,
            EnrollmentCheck,
            DBOps,
            ResultDBOps
        },
        structs::Device,
    },
    enrollment_checks::{
        enrollment_time,
        check_device_id
    },
};
use tokio::{
    net::TcpStream,
    sync::{
        oneshot,
        mpsc::Sender,
    }
};
use std::mem;

pub async fn manage_enrollment(stream: TcpStream, data_parsed: Device, db_tx: Sender<DBOps>) {
    let (check_window, _) = enrollment_time::check_window();
    let mut state = GlobalStatesEnrollment::RespondInitial(stream);
    loop {
        let swap_stream = mem::replace(&mut state, GlobalStatesEnrollment::Transitioning);
        println!("value in swap_stream: {:?}", swap_stream);
        match swap_stream {
            GlobalStatesEnrollment::RespondInitial(stream) => {
                match check_window {
                    TimeStatus::Open => {
                        println!("[GlobalStatesEnrollment::RespondInitial] checking packet then responding.");
                        println!("[GlobalStatesEnrollment::RespondInitial] {:?}", stream);

                        let en_check = check_device_id::check_id(&data_parsed.device_id);
                        if en_check == EnrollmentCheck::Success {
                            let (tx, rx) = oneshot::channel();
                            if let Err(_) = db_tx.send(DBOps::CheckDevice(tx, data_parsed)).await{
                                println!("[GlobalStatesEnrollment::RespondInitial] receiver dropped when sending to manage_db");
                            };

                            match rx.await{
                                //Error if device doesn't exist which it shouldn't because this is
                                //the enrollment phase. Success if device exist which should close
                                //the connection as the device exist. 
                                Ok(ResultDBOps::Error) => state = GlobalStatesEnrollment::FinalVerification(stream),
                                Ok(ResultDBOps::Success) | Err(_) => state = GlobalStatesEnrollment::ClosedEnrollment(stream),
                            };
                        } else {
                            println!("[GlobalStatesEnrollment::RespondInitial] enrollment check failed moving to closed enrollment");
                            state = GlobalStatesEnrollment::ClosedEnrollment(stream);
                        }
                    }
                    TimeStatus::Closed => {
                        println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window closed dropping connection");    
                        state = GlobalStatesEnrollment::ClosedEnrollment(stream);
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
