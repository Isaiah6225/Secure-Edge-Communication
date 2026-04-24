use crate::{
    networking::conn,
    common::enums::GlobalStatesEnrollment
};

pub fn manage_global_state() {
    let mut state = GlobalStatesEnrollment;

    loop {
        match state {
            GlobalStatesEnrollment::AwaitRequest => {
                println!("[GlobalStatesEnrollment::AwaitRequest]");

            }
        }
    }
}
