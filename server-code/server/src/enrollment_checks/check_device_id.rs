use crate::common::{
    enums::EnrollmentCheck,
};

pub fn check_id(mac: &[u8; 6]) -> EnrollmentCheck {
    //TODO create a cleaner implementation. Probably should use the OUI in the mac address instead. 
    //look into a more idomatic approach
    let cmp_1 = [0; 6];
    let cmp_2 = [255; 6];
    println!("[enrollment_checks::check_device_id::check_id] testing mac address: {:?}", mac);

    if cmp_1 == *mac {
        return EnrollmentCheck::Error
    } else if cmp_2 == *mac {
        return EnrollmentCheck::Error
    } else {
        return EnrollmentCheck::Success
    }
}
