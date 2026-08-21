use server::{
    database::{create_db, manage_db},
    global_state::{global_state, manage_request},
    networking::conn,
    common::{
        errors::ServerError,
        enums::MainFlow,
        structs::DBClient,
    }
};
use tokio::{
    task,
    net::TcpListener,
    sync::mpsc
};
use dotenv::dotenv;
use std::env;
use p256::ecdsa::SigningKey;



#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), ServerError> {
    //check db
    let db_conn = create_db::create_db()?;
    let mut join_handles = vec![];
    
    //set tcp listener and extract IP from environment variables
    dotenv().ok();
    let ip = env::var("IP")?;
    let listener = TcpListener::bind(ip).await?;
    let (tx, rx) = mpsc::channel(50);

    
    join_handles.push(task::spawn(async move {
        println!("[main] spawing task to manage db connections");
        task::spawn(manage_db::manage_db(db_conn, rx));
        
        loop {
            let tx_c = tx.clone();
            match manage_request::manage_request(&listener).await {
                Ok(stream) => {
                    println!("[main] spawning task to handle connection");
                    task::spawn(async move {
                        match conn::handle_connection(stream).await {
                            MainFlow::Drop => {
                                println!("Dropped connection");
                            },

                            MainFlow::Enroll(stream, data_parsed) => {
                                //init db_client
                                let db_client = DBClient::new(tx_c);
                                task::spawn(global_state::manage_enrollment(stream, data_parsed, db_client));
                            }
                        }
                    });
                }
                Err(_) => {
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
