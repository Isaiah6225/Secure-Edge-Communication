use rusqlite::{Connection, named_params};
use crate::common::{
    errors::ServerError
};

pub fn check_device_db(
    db_conn: &Connection, 
    dev_id: [u8; 6], 
) -> Result<bool, ServerError> {
    let mut select_stmt = db_conn.prepare("SELECT mac_address, pub_key 
                                          FROM device WHERE mac_address = :mac_address")?;
    let device_exist = select_stmt.exists(named_params!{
       ":mac_address": dev_id,
    })?;
    
    println!("[database::check_device_db] {:?}", device_exist);
    Ok(device_exist)
}
