use rusqlite::{Connection, named_params};
use crate::common::{
    structs::Device,
    errors::ServerError
};

pub fn check_device_db(
    db_conn: &Connection, 
    dev_id: [u8; 6], 
    dev_pub: [u8; 33], 
    nonce:  u32
) -> Result<(), ServerError> {

    //let conn = Connection::open_in_memory()?;
    let dev = Device {
        device_id: dev_id, 
        device_pub: dev_pub,
        nonce: nonce,
    };

    let mut select_stmt = db_conn.prepare("SELECT mac_address, pub_key, nonce  FROM device WHERE mac_address = :mac_address AND pub_key = pub_key")?;

    let device_iter = select_stmt.query_map(named_params!{
       ":mac_address": dev.device_id,
       ":pub_key": dev.device_pub,
    }, |row| {
        Ok(Device {
            device_id: row.get(0)?,
            device_pub: row.get(1)?,
            nonce: row.get(2)?,
        })
    })?;
    Ok(())
}
