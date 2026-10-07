use rusqlite::Connection;
use crate::common::{
    errors::ServerError,
    enums::DBSave,
};
use std::convert::AsRef;

pub fn update_device_status(db_conn: &Connection, device_id: [u8; 6], device_pub: [u8; 33], save_op: DBSave) -> Result<(), ServerError> {
    db_conn.execute(
        "UPDATE device SET enrollment_status = (?3) WHERE mac_address = (?1) AND pub_key = (?2)", 
        (device_id , device_pub, save_op.as_ref())
    )?;
    println!("[database::update_device_status] updated device successfully"); 
    Ok(())
}
