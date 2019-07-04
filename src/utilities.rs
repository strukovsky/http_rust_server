use std::collections::HashMap;

pub struct Url
{
    path: String,
    arguments: HashMap<String, String>,
}

impl Url {
    pub fn path(&self) -> &String{&self.path}
    pub fn arguments(&self) -> &HashMap<String, String>
    {
        &self.arguments
    }

    pub fn new() -> Self
    {
        Url{path:"/".to_string(), arguments:HashMap::new()}
    }
    pub fn parse(url_string: &str) -> Self
    {
        let mut url = Url::new();
        if url_string.contains("?")
        {
            let items: Vec<&str> = url_string.split("?").collect();
            url.path = items[0].to_string();
            let arguments: Vec<&str> = items[1].split("&").collect();
            for argument in arguments
                {
                    let split_argument: Vec<&str> = argument.split("=").collect();
                    url.arguments.insert(split_argument[0].to_string(), split_argument[1].to_string());
                }
        }
        else {
            url.path = url_string.to_string();
        }
        return url;
    }
}

