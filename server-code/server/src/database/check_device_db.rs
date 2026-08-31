use rusqlite::{Connection, named_params};
use crate::common::{
    structs::DeviceEnrl,
    errors::ServerError
};

pub fn check_device_db(
    db_conn: &Connection, 
    dev_id: [u8; 6], 
    dev_pub: [u8; 33], 
) -> Result<bool, ServerError> {
    let mut select_stmt = db_conn.prepare("SELECT mac_address, pub_key, nonce 
                                          FROM device WHERE mac_address = :mac_address AND pub_key = :pub_key")?;
    let device_exist = select_stmt.exists(named_params!{
       ":mac_address": dev_id,
       ":pub_key": dev_pub,
    })?;

    Ok(device_exist)
}
