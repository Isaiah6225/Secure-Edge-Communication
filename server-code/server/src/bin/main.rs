use server::{
    common::structs::TimeConfig,
    global_state::global_state,
};
use tokio::{
    task,
    sync::watch,
    time::{Duration, sleep},
};

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let mut join_handles = vec![];
    
    for _ in 1..=3 {
        join_handles.push(task::spawn(async move {
            global_state::manage_global_state().await; 
        }));
    }

    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
}
