use std::net::TcpStream;
use std::io::{Write, Read};

use crate::request::{RequestBase, RequestPOST};
use crate::response::Response;
use crate::utilities;
use crate::utilities::Url;
use crate::sql;
use crate::data::Person;
use postgres::Connection;

pub fn send_response(mut stream: &TcpStream, response:&Response )
{
    let response_string = response.make_response();
    match stream.write(response_string.as_bytes()) {
        Ok(_)=>println!("Server: Sent response \n{}\n\n\n", response_string),
        Err(e)=>println!("{}", e)
    }

}

pub fn handle_request(mut stream: &TcpStream)
{
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_)=>{
            let req = String::from_utf8_lossy(&buf).to_string();
            println!("Server: Received request \n {} \n\n\n", req);
            let request = RequestBase::build_request( req.as_str());
            if req.starts_with("POST"){
                println!("Request is POST");
                let mut _request_post = RequestPOST::build_request(&req);

            }
            else if req.starts_with("GET"){
                println!("Request is GET");
                let url = utilities::Url::parse(request.url()) ;
                let response = do_request(&url);
                send_response(stream, &response);
            }

        }
        Err(e)=>println!("{}", e)
    }
}

pub fn do_request(url: &Url) -> Response
{
    let path = url.path().as_str();
    let arguments = url.arguments();
    let conn = &sql::establish_connection();
    sql::create_table(conn);
    let mut response = Response::new();

    match path {
        "/select" =>{
            let items = sql::select(conn, "");
            let mut result = "".to_owned();
            for item in items{
                result.push_str(&format!("{:?}", item))
            }
            response.set_body(&result)

        }
        "/insert" =>{
            let person = Person{id:0, name: arguments.get("name").unwrap().to_string(), age: arguments.get("age").unwrap().parse::<i32>().unwrap()};
            sql::insert(conn, &person);

        }
        "/delete" =>{

        }
        "/update"=>{

        }
        _ => {

        }

    }

    return response;



}

