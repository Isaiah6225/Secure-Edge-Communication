use crate::{
    common::enums::EnrollmentWindowStatus,
};
use chrono::{DateTime, Local};

pub fn check_window() -> (EnrollmentWindowStatus, u64){
    //get current local system time
    let dt: DateTime<Local> = Local::now();
    let time_formatted = format!("{}", dt.format("%M"));
    let current_minute = time_formatted.parse::<u64>().unwrap();
    println!("{:?}", current_minute);

    match current_minute {
        30_u64..=59_u64=> {
            println!("Enrollment window is open where the minute is: {:?}", time_formatted);
            return (EnrollmentWindowStatus::OpenEnrollment, 0); 
        },

        0_u64..=29_u64 => {  
            println!("Enrollment window is closed where the minute is: {:?}", time_formatted);
            let seconds_to_sleep = get_seconds(current_minute);
            return (EnrollmentWindowStatus::ClosedEnrollment, seconds_to_sleep);
        },

        _ => todo!(),
    } 

}

fn get_seconds(c_m: u64) -> u64{
    let minutes_sleep = 30 - c_m; 
    let seconds_sleep: u64 = (minutes_sleep * 60).into(); 
    return seconds_sleep
}
