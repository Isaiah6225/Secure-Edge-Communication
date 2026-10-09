use rusqlite::{Connection, named_params};
use crate::common::{
    structs::DBEnrollmentStatus,
    errors::ServerError
};

pub fn get_enrollment_status(db_conn: &Connection, device_id: [u8; 6]) -> Result<bool, ServerError> {
    let mut select_stmt = db_conn.prepare(
        "SELECT enrollment_status FROM device  WHERE mac_address = :mac_address", 
    )?;
    
    let result_stmt = select_stmt.query_one(
        named_params!{
            ":mac_address": device_id
        }, |row| {
            Ok(DBEnrollmentStatus {
                save_op: row.get(0)?,
            })
        }
    )?;
    if result_stmt.save_op == "Verified" {
        Ok(true)
    } else {
        Ok(false)
    }
}
