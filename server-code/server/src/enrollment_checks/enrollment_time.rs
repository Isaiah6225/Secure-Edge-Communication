use crate::{
    common::enums::EnrollmentWindowStatus,
};
use chrono::{DateTime, Local};

pub fn check_window() -> EnrollmentWindowStatus{
    //get current local system time
    let dt: DateTime<Local> = Local::now();
    let time_formatted = format!("{}", dt.format("%M"));
    let current_minute = time_formatted.parse::<u8>().unwrap();

    match current_minute {
        30_u8..=59_u8=> {
            println!("Enrollment window is open where the minute is: {:?}", time_formatted);
            return EnrollmentWindowStatus::OpenEnrollment; 
        },

        0_u8..=29_u8 => {  
            println!("Enrollment window is closed where the minute is: {:?}", time_formatted);
            return EnrollmentWindowStatus::ClosedEnrollment;
        },

        _ => todo!(),
    } 

}
