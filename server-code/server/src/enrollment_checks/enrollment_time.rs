use crate::{
    common::enums::TimeStatus
};
use chrono::{DateTime, Local};

pub fn check_window() -> (TimeStatus, u64){
    //get current local system time
    let dt: DateTime<Local> = Local::now();
    let time_formatted = format!("{}", dt.format("%M"));
    let current_minute = time_formatted.parse::<u64>().unwrap();
    println!("{:?}", current_minute);
    let seconds_to_sleep = get_seconds(current_minute);


    match current_minute {
        30_u64..=59_u64=> {
            println!("[enrollment_time::check_window] Enrollment window is open where the minute is: {:?}", time_formatted);
            return (TimeStatus::Open, seconds_to_sleep); 
        },

        0_u64..=29_u64 => {  
            println!("[enrollment_time::check_window] Enrollment window is closed where the minute is: {:?}", time_formatted);
            return (TimeStatus::Closed, seconds_to_sleep);
        },

        _ => todo!(),
    } 

}



fn get_seconds(c_m: u64) -> u64{
    if c_m >= 30 {
        let open_minutes_sleep = c_m - 30;
        let open_seconds_sleep: u64 = (open_minutes_sleep * 60).into();
        return open_seconds_sleep
    } else {
        let closed_minutes_sleep = 30 - c_m; 
        let closed_seconds_sleep: u64 = (closed_minutes_sleep * 60).into(); 
        return closed_seconds_sleep
    };
    
}

