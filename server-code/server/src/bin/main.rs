use server::{
    global_state::global_state,
    networking::conn,
    enrollment_checks::enrollment_time,
    common::enums::{
        EnrollmentWindowStatus,
    }
};
use tokio::{
    task,
    time::{Duration, sleep},
};


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut join_handles = vec![];
    let (mut check_window, sleep_time) = enrollment_time::check_window();

    
    join_handles.push(task::spawn(async move {
        loop {
            match check_window {
                EnrollmentWindowStatus::OpenEnrollment => {
                    println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window open await request.");
                    let sleep = sleep(Duration::from_secs(sleep_time));

                    tokio::pin!(sleep);
                    tokio::select!{
                        soc = conn::tcp_listen() => {
                            println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] received request moving to respond inital");
                            match soc {
                                Ok(stream) => {
                                    println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Passing the stream to manage_global_state fn.");
                                    task::spawn(global_state::manage_global_state(stream));
                                },
                                Err(_e) => println!("Error from stream"),
                            }
                        }
                        _ = &mut sleep => {
                            println!("[GlobalStatesEnrollment::AwaitRequest] enrollment time closed moving to closed enrollment."); 
                            check_window = EnrollmentWindowStatus::ClosedEnrollment;
                        }
                    }

                },

                EnrollmentWindowStatus::ClosedEnrollment => {
                    println!("[GlobalStatesEnrollment::EnrollmentWindowStatus] Enrollment window closed sleeping process until opened ");

                    sleep(Duration::from_secs(sleep_time)).await;
                    (check_window, _) = enrollment_time::check_window();
                },
            }
            
        }
    }));

    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
}
