use crate::{
    database::check_device_db,
    common::{
        enums::{DBOps, ResultDBOps},
        structs::Device
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
                        ResultDBOps::Success
                    }, 
                    Err(_) => {
                        println!("[database::manage_db] check_device_db device not found");
                        ResultDBOps::Error
                    },
                };

                if let Err(_) = sender.send(check_res) {
                    println!("[database::manage_db] send to manage_enrollment failed. receiver dropped");
                };
            },
            None => continue,
        };
    }
}
