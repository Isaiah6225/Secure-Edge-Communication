use rusqlite::{Connection, named_params};
use crate::common::{
    structs::DeviceEnrl,
    errors::ServerError
};

pub fn check_device_db(
    db_conn: &Connection, 
    dev_id: [u8; 6], 
    dev_pub: [u8; 33], 
) -> Result<(), ServerError> {
    let mut select_stmt = db_conn.prepare("SELECT mac_address, pub_key, nonce  FROM device WHERE mac_address = :mac_address AND pub_key = :pub_key")?;
    let device_iter = select_stmt.query_map(named_params!{
       ":mac_address": dev_id,
       ":pub_key": dev_pub,
    }, |row| {
        Ok(DeviceEnrl {
            device_id: row.get(0)?,
            device_pub: row.get(1)?,
            nonce: row.get(2)?,
        })
    })?;
    for dev in device_iter {
        if let Ok(found_dev) = dev {
            println!("[database::check_device_db] Dev: {:?}", found_dev);
        }
    }
    Ok(())
}
