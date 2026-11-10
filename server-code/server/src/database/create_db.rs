use rusqlite::{Connection};
use crate::common::errors::ServerError;

pub fn create_db() -> Result<Connection, ServerError> {
    let conn = Connection::open("device_registry.db").expect("[database::create_db] couldn't open db");
    println!("[database::create_db] opened devices table in device_registry database");
    return Ok(conn)
}
