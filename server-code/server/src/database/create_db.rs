use rusqlite::{Connection};
use crate::common::errors::ServerError;

pub fn create_db() -> Result<Connection, ServerError> {
    let conn = Connection::open("device_registry.db").expect("[database::create_db] couldn't create db");

    conn.execute(
        "CREATE TABLE device (id INTEGER PRIMARY KEY, mac_address BLOB, pub_key BLOB, nonce INTEGER, enrollment_status TEXT NOT NULL)",
        (),
    )?;
    println!("[database::create_db] created devices table in device_registry database");
    return Ok(conn)
}
