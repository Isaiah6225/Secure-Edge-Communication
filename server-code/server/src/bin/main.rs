use server::{
    networking::conn,
    enrollment_checks::enrollment_time,
};

use std::thread;



fn main() {
    println!("Running conn::tcp_listen function...");
    enrollment_time::check_window();
    conn::tcp_listen();
}
