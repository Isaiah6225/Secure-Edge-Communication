use crate::{
    common::{
        enums::{
            GlobalStatesEnrollment,
        },
    },
};
use tokio::{
    net::TcpStream,
};


pub async fn manage_global_state(stream: TcpStream) {
    let mut state = GlobalStatesEnrollment::AwaitRequest;


    loop {
        match state {
            GlobalStatesEnrollment::AwaitRequest => {   
                println!("[GlobalStatesEnrollment::AwaitRequest] awaiting request.");
                println!("got stream: {:?}", stream);
                state = GlobalStatesEnrollment::RespondInital;

            }

            GlobalStatesEnrollment::RespondInital => {
                println!("[GlobalStatesEnrollment::RespondInital] checking packet then responding.");
            }

            GlobalStatesEnrollment::FinalVerification => todo!()
        }
    }
}
