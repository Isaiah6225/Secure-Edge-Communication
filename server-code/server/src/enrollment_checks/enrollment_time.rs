use chrono::{DateTime, Local};
use crate::common::errors::ServerError;

pub fn check_window() -> Result<(), ServerError>{
    //get current local system time
    let dt: DateTime<Local> = Local::now();
    let time_formatted = format!("{}", dt.format("%M"));
    let current_minute = time_formatted.parse::<u64>().unwrap();
    println!("{:?}", current_minute);

    if let 0_u64..=29_u64 = current_minute {
        return Err(ServerError::EnrollmentClosedErr)
    } else {
        Ok(())
    }
}
