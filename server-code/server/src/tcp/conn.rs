use std::{error::Error, net::TcpListener};

//Listen for tcp connection on loop back addr. 
pub fn tcp_listen() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established"); 
            }

            Err(e) => {
                println!("Connection failed with: {}", e);
            }
        }
    }
    Ok(())
}
