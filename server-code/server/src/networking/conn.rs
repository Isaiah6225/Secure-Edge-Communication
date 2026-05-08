use std::env;
use tokio::net::TcpListener;
use dotenv::dotenv;
use crate::common::errors::ServerError;

//Listen for tcp connection on loop back addr. 
pub async fn tcp_listen() -> Result<tokio::net::TcpStream, ServerError>{
    dotenv().ok();
    let ip = env::var("IP")?;
    let listener = TcpListener::bind(ip).await?;

    match listener.accept().await {
        Ok((socket, addr)) => {
            println!("new client: {:?}", addr);
            return Ok(socket);
        },
        Err(e) => {
            println!("couldn't get client: {:?}", e);
            return Err(ServerError::IoErr(e));
        },
    }
}

//handle connection received connection 
/*
fn handle_connection(mut tcp_stream: TcpStream){
    let buf_reader = BufReader::new(&tcp_stream);
    let req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {req:#?}");
}
*/
