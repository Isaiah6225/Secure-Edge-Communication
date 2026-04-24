use std::{
    env,
    error::Error, 
    net::{TcpListener, TcpStream},
    io::{BufReader, BufRead}
};
use dotenv::dotenv;

//Listen for tcp connection on loop back addr. 
pub fn tcp_listen() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let ip = env::var("IP")?;
    let listener = TcpListener::bind(ip)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established"); 
                handle_connection(stream);
            }

            Err(e) => {
                println!("Connection failed with: {}", e);
            }
        }
    }
    Ok(())
}

//handle connection received connection 
fn handle_connection(mut tcp_stream: TcpStream){
    let buf_reader = BufReader::new(&tcp_stream);
    let req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {req:#?}");
}
