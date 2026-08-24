use tokio::{
    net::{
        TcpListener,
        TcpStream,
    },
    io::Interest,
};
use crate::{
    common::{
        enums::{MainFlow, ParsedStruct},
        errors::ServerError,
    },
    parse::parse_packet,
};
use std::{
    str,
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
pub async fn handle_connection(tcp_stream: TcpStream) -> MainFlow {
    println!("[networking::conn::handle_connection] starting handle connection match statement");
    let mut buf = [0u8; 4096];
     
    //tcp_stream.readable().await;
    match tcp_stream.ready(Interest::READABLE).await {
        Ok(_) => {
            match tcp_stream.try_read(&mut buf) {
                Ok(0) => {
                    println!("[networking::conn::handle_connection] 0 bytes returned");
                    return MainFlow::Drop;
                },

                Ok(n) => {
                    println!("[networking::conn::handle_connection] read {} bytes", n);
                    println!("[networking::conn::handle_connection] parsing buffer");

                    let string = match str::from_utf8(&buf[..n]) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("[networking::conn::handle_connection] error: {:?}", e);
                            return MainFlow::Drop;
                        },
                    };
                    //let v: Vec<&str> = string.split("\n").collect();
                    println!("[networking::conn::handle_connection] string res: {:?}", string);
                    let data_parsed = match parse_packet::parse(string) {
                        Ok(data) => data,
                        Err(_) => return MainFlow::Drop,
                    };
                    return MainFlow::Enroll(tcp_stream, data_parsed);
                },

                Err(e) => {
                    println!("[networking::conn::handle_connection] error: {:?}", e);
                    return MainFlow::Drop;
                },
            } 
        },

        Err(e) => {
            println!("[networking::conn::handle_connection] got an error from tcp_stream ready with: {:?}", e);
            return MainFlow::Drop;
        },
    }
}
