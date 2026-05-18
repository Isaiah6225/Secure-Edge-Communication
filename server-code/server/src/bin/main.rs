use server::{
    global_state::{global_state, manage_request},
    networking::conn,
    common::{
        errors::ServerError,
        enums::{
            MainFlow
        }
    }
};
use tokio::{
    task,
    net::TcpListener
};
use dotenv::dotenv;
use std::env;


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), ServerError>{
    let mut join_handles = vec![];
    
    //set tcp listener and extract IP from environment variables
    dotenv().ok();
    let ip = env::var("IP")?;
    let listener = TcpListener::bind(ip).await?;
    
    join_handles.push(task::spawn(async move {
        loop {
            match manage_request::manage_request(&listener).await {
                Ok(stream) => {
                    task::spawn(async move {
                        match conn::handle_connection(stream) {
                            MainFlow::Drop => {
                                println!("Dropped connection");
                            },

                            MainFlow::Enroll(stream) => {
                                task::spawn(global_state::manage_enrollment(stream));
                            }
                        }
                    });
                }

                Err(e) => {
                    continue;
                }
            }
        }
    }));

    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
    Ok(())
}
