


pub struct RequestBase
{
    method: String,
    url: String,


}

pub struct RequestPOST
{
    base: RequestBase,
    content_type: String,
    content_length: i64,
    contents: String,

}

impl RequestBase
{


    pub fn url(&self) ->&str {self.url.as_str()}
    pub fn new() -> Self
    {
        RequestBase {method:"GET".to_string(),
            url:"/".to_string(),
          }
    }

    /*pub fn to_string(&self) -> String
    {
        format!("{} {} HTTP/1.1\r\nUser-Agent: {}\r\nHost: {}\r\nAccept-Language: {}\r\nAccept-Encoding: {}\r\nConnection: {}",
                "GET",
                self.url,
                self.user_agent,
                self.host,
                self.accept_language,
                self.accept_encoding,
                self.connection)
    }*/



     pub fn build_request(string: &str) ->  Self
    {
        let mut req = RequestBase::new();
        let items:Vec<&str> = string.split("\r\n").collect();
        let first_string: Vec<&str> = items[0].split(" ").collect();
        let method = first_string[0];
        let url = first_string[1];
        req.method = method.to_string();
        req.url = url.to_string();
        return req
        /*
        for item in items{
            let string: Vec<&str> = item.split(": ").collect();
            let title = string[0];
            let contents = string[1].to_string();
            match title as &str {
                "User-Agent" => self.user_agent = contents,
                "Host" => self.host = contents,
                "Accept-Language" => self.accept_language = contents,
                "Accept-Encoding"=> self.accept_encoding = contents,
                "Connection"=>self.connection = contents,
                _ => self.connection = "Keep-alive".to_string()
            }
        }
        */

    }
}

impl RequestPOST {
    pub fn new() -> Self
    {
        RequestPOST{
            base: RequestBase::new() ,
            content_length: 17,
            content_type: "application/x-www-form-urlencoded".to_string(),
            contents: "name=Boris&age=32".to_string()
        }
    }

    pub fn build_request(string: &str)
    {
        let mut req = RequestPOST::new();
        let items:Vec<&str> = string.split("\r\n").collect();
        let first_string: Vec<&str> = items[0].split(" ").collect();
        let method = first_string[0];
        let url = first_string[1];
        req.base.method = method.to_string();
        req.base.url = url.to_string();

        for item in items
            {
                let string: Vec<&str> = item.split(": ").collect();
                let title = string[0];
                let contents = string[1].to_string();
                match title as &str {
                    "Content-Type"=> req.content_type = contents,
                    "Content-Length"=>req.content_length = contents.parse::<i64>().unwrap(),
                   _ => {}
                }
            }

        let contents_split: Vec<&str> = string.split("\r\n\r\n").collect();
        req.contents = contents_split[1].to_string();

    }
}