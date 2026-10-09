use crate::{
    database::{check_device_db, save_device_db, update_device_status_db, get_enrollment_status_db},
    common::{
        enums::DBOps,
        errors::ServerError
    },
};
use rusqlite::Connection; 
use tokio::sync::mpsc::Receiver;

pub async fn manage_db(db_conn: Connection, mut rx_mpsc: Receiver<DBOps>) {
    println!("[database::manage_db] starting manage_db");
    loop {
        match rx_mpsc.recv().await{
            Some(DBOps::CheckDevice(sender, device)) => {
               //receive device struct from global_state and check device in db 
                let check_res = match check_device_db::check_device_db(&db_conn, device.device_id, device.device_pub){
                    Ok(device_exists) => {
                        if device_exists == true {
                            println!("[database::manage_db] check_device_db device found");
                            Err(ServerError::DeviceExistErr)
                        } else {
                            println!("[database::manage_db] check_device_db device not found");
                            Ok(())
                        }
                    }, 
                    Err(e) => {
                        println!("[database::manage_db] error from check_device_db: {:?}", e);
                        Err(e)
                    },
                };

                if let Err(_) = sender.send(check_res) {
                    println!("[database::manage_db] send to manage_enrollment failed. receiver dropped");
                };
            },

            Some(DBOps::SaveDevice(sender, device)) => {
                let check_save = match save_device_db::save_device(&db_conn, device.device_id, device.device_pub, device.nonce, device.save_op) {
                    Ok(()) => {
                        println!("[database::manage_db] save operation successful");
                        ()
                    },
                    Err(e) => {
                        println!("[database::manage_db] save operation failed with: {:?}", e);
                    },
                };
                
                if let Err(_) = sender.send(Ok(check_save)) {
                    println!("[database::manage_db] send to manage_enrollment failed receiver dropped");
                };
            },

            Some(DBOps::UpdateDeviceStatus(sender, device)) => {
                let check_update_status = match update_device_status_db::update_device_status(&db_conn, device.device_id, device.device_pub, device.save_op) {
                    Ok(()) => {
                        println!("[database::manage_db] update device status operation successful"); 
                        ()
                    }, 
                    Err(e) => {
                        println!("[database::manage_db] update operation failed with: {:?}", e);
                    }, 
                };
                if let Err(_) = sender.send(Ok(check_update_status)) {
                    println!("[database::manage_db] send to manage_enrollment failed receiver dropped");
                };
            }, 

            Some(DBOps::IsDeviceVerified(sender, device)) => {
                let check_device_enrollment_status = match get_enrollment_status_db::get_enrollment_status(&db_conn, device.device_id) {
                    Ok(device_verified) => {
                        if device_verified == true {
                            println!("[database::manage_db] device is verified");
                            Ok(())
                        } else {
                            println!("[database::manage_db] device isn't verified");
                            Err(ServerError::DeviceNotVerifiedErr)
                        }
                    }, 
                    Err(e) => {
                        println!("[database::manage_db] get operation failed with: {:?}", e);
                        Err(e)
                    }
                };

                if let Err(_) = sender.send(check_device_enrollment_status) {
                    println!("[database::manage_db] send to manage_enrollment failed receiver dropped");
                }
            }

            None => break,
        };
    }
}
