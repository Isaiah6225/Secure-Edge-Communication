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
    task::spawn_blocking(move || {
        global_state::manage_global_state(); 
    });
}
