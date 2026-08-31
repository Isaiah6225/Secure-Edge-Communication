use crate::{
    database::{check_device_db, save_device_db},
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
            None => break,
        };
    }
}
