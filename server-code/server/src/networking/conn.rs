use tokio::{
    net::{
        TcpListener,
        TcpStream,
    },
    io
};

use crate::common::{
    enums::MainFlow,
    errors::ServerError,
    structs::EnrollmentReceiveInital,
};
use std::{
    str,
    time::Duration,
};

//Listen for tcp connection on loop back addr. 
pub async fn tcp_listen(listener: &TcpListener) -> Result<tokio::net::TcpStream, ServerError>{
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
pub fn handle_connection(tcp_stream: TcpStream) -> MainFlow {
    let mut buf = [0u8; 4096];
    

    match tcp_stream.try_read(&mut buf) {
        Ok(0) => {
            println!("[networking::conn::handle_connection] 0 bytes returned");
            return MainFlow::Drop;
        }
        Ok(n) => {
            println!("[networking::conn::handle_connection] read {} bytes", n);
            println!("[networking::conn::handle_connection] parsing buffer");

            let string = match str::from_utf8(&buf[..n]) {
                Ok(v) => v,
                Err(e) => {
                    return MainFlow::Drop;
                },
            };
            println!("[networking::conn::handle_connection] string res: {:?}", string.split("\n"));
            
            return MainFlow::Enroll(tcp_stream);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            println!("[networking::conn::handle_connection] error would block");
            return MainFlow::Drop;
        }
        Err(e) => {
            println!("[networking::conn::handle_connection] error: {:?}", e);
            return MainFlow::Drop;
        }
    }
}
