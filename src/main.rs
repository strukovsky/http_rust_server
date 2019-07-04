use std::net::{TcpListener, TcpStream};
use std::thread;


mod client;
mod server;
mod response;
mod request;
mod utilities;
mod sql;
mod data;


fn handle_client(stream: TcpStream, response: &response::Response)
{
   /*client::send_request(&stream, request::Request::new().to_string().as_str());
    server::handle_request(&stream);*/
    server::handle_request(&stream);
    server::send_response(&stream, response)

}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming()
        {
            match stream {
                Ok(stream) => {
                    thread::spawn(||
                        {
                            let response = response::Response::new();
                            handle_client(stream, &response);
                        }
                    );
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }


}
