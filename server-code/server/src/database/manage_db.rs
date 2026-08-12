use crate::{
    database::{check_device_db, save_device_db},
    common::{
        enums::{DBOps, ResultDBOps},
    },
};
use rusqlite::Connection; 
use tokio::{
    sync::mpsc::Receiver,
};

pub async fn manage_db(db_conn: Connection, mut rx_mpsc: Receiver<DBOps>) {
    println!("[database::manage_db] starting manage_db");
    loop {
        match rx_mpsc.recv().await{
            Some(DBOps::CheckDevice(sender, device)) => {
               //receive device struct from global_state and check device in db 
                let check_res = match check_device_db::check_device_db(&db_conn, device.device_id, device.device_pub, device.nonce){
                    Ok(()) => {
                        println!("[database::manage_db] check_device_db found device");
                        ()
                    }, 
                    Err(e) => {
                        println!("[database::manage_db] check_device_db device not found");
                    },
                };

                if let Err(_) = sender.send(Ok(check_res)) {
                    println!("[database::manage_db] send to manage_enrollment failed. receiver dropped");
                };
            },

            Some(DBOps::SaveDevice(sender, device, dbsave)) => {
                let check_save = match save_device_db::save_device(&db_conn, device.device_id, device.device_pub, device.nonce, dbsave) {
                    Ok(()) => {
                        println!("[database::manage_db] save operation successful");
                        ResultDBOps::Success
                    },
                    Err(e) => {
                        println!("[database::manage_db] save operation failed with: {:?}", e);
                        ResultDBOps::Error(device)
                    },
                };
                
                if let Err(_) = sender.send(check_save) {
                    println!("[database::manage_db] send to manage_enrollment failed receiver dropped");
                };
            },
            None => break,
        };
    }
}
