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
};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc,
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
                            let (tx, mut rx) = mpsc::channel(10);
                            println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] checking device id in manage_db");

                            if let Err(_) = db_tx.send(DBOps::CheckDevice(tx.clone(), data_parsed)).await{
                                println!("[GlobalStatesEnrollment::RespondInitial] receiver dropped when sending to manage_db");
                                break;
                            };

                            match rx.recv().await {
                                //Error if device doesn't exist which it shouldn't because this is
                                //the enrollment phase then enroll. Success if device exist close
                                //the connection as the device exist. 
                                Some(ResultDBOps::Error(device)) => {
                                    if let Err(_) = db_tx.send(DBOps::SaveDevice(tx.clone(), device, DBSave::Pending)).await{
                                        println!("[GlobalStatesEnrollment::RespondInitial] receiver dropped when sending to manage_db");
                                        break;
                                    };
                                    state = Some(GlobalStatesEnrollment::FinalVerification(stream));
                                },
                                Some(ResultDBOps::Success) => break,
                                None => break,
                            };

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
