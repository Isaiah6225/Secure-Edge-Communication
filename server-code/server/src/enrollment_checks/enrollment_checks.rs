/*use crate::{
    database::check_device_db,
    enrollment_checks::check_device_id,
    common::{
        enums::EnrollmentCheck,
        structs::Device, 
    }
};

pub fn enrollment_checks(data_parsed: &Device) -> EnrollmentCheck{
    let check_device_id_res = check_device_id::check_id(data_parsed.device_id);
    let check_device_db_res = check_device_db::check_device_db(data_parsed.device_id, data_parsed.device_pub.clone(), data_parsed.nonce);
}
*/
