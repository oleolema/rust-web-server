use std::collections::HashMap;
use std::str;

use crate::request::HttpVersion;

#[derive(Debug)]
pub struct HttpResponse {
    pub code: i32,
    pub message: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Server"), String::from("Rust Server/1.0"));
        HttpResponse { code: 200, message: String::from("OK"), version: HttpVersion::V1, headers, body: None }
    }

    pub fn success(&mut self) -> &mut Self {
        self.code = 200;
        self.message = String::from("OK");
        self
    }

    pub fn bad_request(&mut self) -> &mut Self {
        self.code = 400;
        self.message = String::from("Bad Request");
        self.body_str(String::from("400 Bad Request"));
        self
    }

    pub fn not_found(&mut self) -> &mut Self {
        self.code = 404;
        self.message = String::from("Not Found");
        self.body_str(String::from("404 Not Found"));
        self
    }

    pub fn error(&mut self) -> &mut Self {
        self.code = 500;
        self.message = String::from("Server Error");
        self.body_str(String::from("500 Server Error"));
        self
    }
}

impl HttpResponse {
    pub fn header(&mut self, name: String, value: String) -> &mut Self {
        self.headers.insert(name, value);
        self
    }

    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers.extend(headers);
        self
    }

    pub fn body_str(&mut self, body: String) -> &mut Self {
        self.header(String::from("Content-Length"), body.len().to_string());
        self.body = Some(body.into());
        self
    }

    pub fn body(&mut self, body: Vec<u8>) -> &mut Self {
        self.header(String::from("Content-Length"), body.len().to_string());
        self.body = Some(body);
        self
    }

    pub fn body_ref(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|it| &it[..])
    }

    pub fn body_str_ref(&self) -> Option<&str> {
        self.body.as_ref().and_then(|it| str::from_utf8(it).ok())
    }
}

impl From<&HttpResponse> for String {
    fn from(response: &HttpResponse) -> Self {
        let first_line = format!("{} {} {}", response.version.to_string(), response.code, response.message);
        let headers = response.headers.iter().map(|it| format!("{}: {}", it.0, it.1))
            .reduce(|acc, it| format!("{}\n{}", acc, it)).unwrap_or_else(|| "".to_string());
        format!("{}\n{}\n\n{}", first_line, headers, response.body_str_ref().unwrap_or_else(|| ""))
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_body() {
        let mut r1 = HttpResponse::new();
        r1.success();
        r1.body_str(String::from("hello world"));
        dbg!(r1.to_string());
        assert_eq!("HTTP/1.1 200 OK", r1.to_string().lines().next().unwrap());
        assert_eq!(Some(&"11".to_string()), r1.headers.get("Content-Length"));
        assert_eq!(Some("hello world"), r1.body_str_ref());
    }

    #[test]
    fn test_no_body() {
        let mut r1 = HttpResponse::new();
        r1.success();
        dbg!(r1.to_string());
        assert_eq!("HTTP/1.1 200 OK", r1.to_string().lines().next().unwrap());
        assert_eq!(None, r1.headers.get("Content-Length"));
        assert_eq!(None, r1.body_ref());
    }
}