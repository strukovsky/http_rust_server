
enum ContentType {
    TextHtml,
    TextPlain,
    Other(String),
}

enum Charset
{
    UTF8,
    Other(String),
}

pub struct Response {
    code: i16,
    content_type: ContentType,
    charset: Charset,
    body: String,

}

impl Response {
    pub fn new() -> Self
    {
        Response { code: 200, content_type: ContentType::TextHtml, charset: Charset::UTF8, body: "<html>Hello, world!</html>".to_string() }
    }

    pub fn set_body(&mut self, body: &str)
    {
        self.body = body.to_string();
    }

    fn format_status(&self) -> String
    {
        let  status;
        match &self.code {
            100 => status = "Continue",
            101 => status = "Switching Protocols",
            102 => status = "Processing",
            200 => status = "200 OK",
            201 => status = "201 Created",
            403 => status = "403 Forbidden",
            _ => status = ""
        }
        return status.to_string();
    }

     fn format_charset(&self) -> String
    {
        let  charset;
        match &self.charset {
            Charset::UTF8 => charset = "UTF8",
            Charset::Other(s) => charset = s.as_str()
        }
        return charset.to_string();
    }

    fn format_content_type(&self) -> String
    {
        let  content_type;
        match &self.content_type {
            ContentType::TextHtml => content_type = "text/html",
            ContentType::TextPlain => content_type = "text/plain",
            ContentType::Other(s) => content_type = s.as_str()
        }
        return content_type.to_string();
    }

    pub fn make_response(&self) -> String {


        let content_type = self.format_content_type();
        let charset = self.format_charset();
        let status = self.format_status();
        return format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-type: {};charset={}\r\n\r\n{}\r\n", status,  self.body.len(),content_type, charset, self.body);
    }
}