use std::net::TcpStream;
use std::io::{Read, Write};


pub fn handle_response(mut stream: &TcpStream)
{
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) =>
            println!("Client: Received response \n{}\n\n\n", String::from_utf8_lossy(&buf)),
        Err(e) => println!("{}", e)
    }
}

pub fn send_request(mut stream: &TcpStream, request: &str)
{
    match stream.write(request.as_bytes()) {
        Ok(_) => {
            println!("Client: Request sent \n{}\n\n\n", request);

           // handle_request(stream);
        }
        Err(e) => println!("{}", e)
    }
}