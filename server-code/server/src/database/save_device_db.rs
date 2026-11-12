use rusqlite::{Connection};
use crate::common::{
    enums::DBSave,
    errors::ServerError
};
use std::convert::AsRef;

pub fn save_device(
    db_conn: &Connection,     
    dev_id: [u8; 6],
    save_op: DBSave
) -> Result<(), ServerError> {
    db_conn.execute(
        "INSERT INTO device (mac_address, enrollment_status) VALUES (?1, ?2)",
        (dev_id, save_op.as_ref()),
    )?;
    println!("[database::save_device_db] updated device data");
    Ok(())
}
