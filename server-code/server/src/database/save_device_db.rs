use rusqlite::{Connection};
use crate::common::{
    enums::DBSave,
    errors::ServerError
};
use std::convert::AsRef;

pub fn save_device(
    db_conn: &Connection,     
    dev_id: [u8; 6],
    dev_pub: [u8; 33], 
    nonce: u32,
    save_op: DBSave
) -> Result<(), ServerError> {
    db_conn.execute(
        "INSERT INTO device (mac_address, pub_key, nonce, enrollment_status) VALUES (?1, ?2, ?3, ?4)",
        (dev_id, dev_pub, nonce, save_op.as_ref()),
    )?;
    Ok(())
}
