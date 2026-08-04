use crate::{
    common::{
        enums::{
            GlobalStatesEnrollment,
            TimeStatus,
            EnrollmentCheck,
            DBOps
        },
        structs::Device,
    },
    enrollment_checks::{
        enrollment_time,
        check_device_id
    },
};
use rusqlite::Connection;
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
                        println!("[GlobalStatesEnrollment::RespondInital] checking packet then responding.");
                        println!("[GlobalStatesEnrollment::RespondInital] {:?}", stream);

                        let en_check = check_device_id::check_id(&data_parsed.device_id);
                        if en_check == EnrollmentCheck::Success {
                            let (tx, rx) = oneshot::channel();
                            db_tx.send(DBOps::CheckDevice(tx, data_parsed)).await;
                        } else {
                            println!("[GlobalStatesEnrollment::RespondInitial] enrollment check failed moving to closed enrollment");
                            state = GlobalStatesEnrollment::ClosedEnrollment(stream);
                        }
                        /*
                        let check_arr = [en_check, db_check];
                        let check_res = check_arr.into_iter().find(|x| *x == EnrollmentCheck::Error);
                        match check_res {
                            Some(EnrollmentCheck::Success) | None => {
                                println!("[GlobalStatesEnrollment::RespondInitial] device ID check successful, moving to checking if device is existing");
                                state = GlobalStatesEnrollment::FinalVerification(stream);
                            }

                            Some(EnrollmentCheck::Error) => {
                                println!("[GlobalStatesEnrollment::RespondInitial] enrollment check failed moving to closed enrollment");
                                state = GlobalStatesEnrollment::ClosedEnrollment(stream);
                            }
                        }
                        match check_device_id::check_id(&data_parsed.device_id) {
                            EnrollmentCheck::Success => {
                                println!("[GlobalStatesEnrollment::RespondInitial] device ID check successful, moving to checking if device is existing");
                                check_device_db::check_device_db(data_parsed.device_id, data_parsed.device_pub, data_parsed.nonce);
                                state = GlobalStatesEnrollment::FinalVerification(stream);
                            }
                            EnrollmentCheck::Error => {
                                println!("[GlobalStatesEnrollment::RespondInitial] enrollment check failed moving to closed enrollment");
                                state = GlobalStatesEnrollment::ClosedEnrollment(stream);
                            }
                        }*/
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
