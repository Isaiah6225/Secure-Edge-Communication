use tokio::{
    net::{TcpListener, TcpStream},
};
use crate::{
    networking::conn,
    common::errors::ServerError,
};


pub async fn manage_request(listener: &TcpListener) -> Result<TcpStream, ServerError> {
    let soc = conn::tcp_listen(&listener).await;
    match soc {
        Ok(stream) => {     
            println!("[manage_request] received socket moving to handle and parse packet");
            return Ok(stream); 
        },
        Err(e) => {
            println!("[manage_request] error from stream {:?}", e);
            return Err(e)
        },
    }
}
