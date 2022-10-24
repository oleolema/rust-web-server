use std::collections::HashMap;
use multimap::MultiMap;
use regex::Regex;
use url::{Url};

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    UNDEFINED,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::UNDEFINED
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    V1,
    UNDEFINED,
}

impl From<&str> for HttpVersion {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => HttpVersion::V1,
            _ => HttpVersion::UNDEFINED
        }
    }
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::V1 => String::from("HTTP/1.1"),
            _ => String::from("UNDEFINED")
        }
    }
}

fn parse_header(value: &str) -> Result<(String, String), &'static str> {
    if let Some(index) = value.find(':') {
        Ok((value[..index].trim_end().into(), value[index + 1..].trim_start().into(), ))
    } else {
        Err("The header parse error")
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    url: Url,

}

impl HttpRequest {
    pub fn new(s: &str) -> Self {
        let (first_line, headers, body) = parse_http_request(s);
        let mut first_line = first_line.split_ascii_whitespace();
        let method: HttpMethod = first_line.next().unwrap().into();
        let path: String = first_line.next().unwrap().into();
        let version: HttpVersion = first_line.next().unwrap().into();
        let headers = headers.iter().map(|it| parse_header(it).unwrap()).collect();
        let body = body.map(|it| it.to_string());
        let url = Url::parse(&format!("{}{}", "http://undefined", &path)).unwrap();
        HttpRequest { method, path: url.path().to_string(), version, headers, body, url }
    }
    pub fn path_match(&self, regex: &Regex) -> bool {
        regex.is_match(&self.path)
    }

    pub fn query(&self) -> Option<&str> {
        self.url.query()
    }

    pub fn query_pair(&self) -> MultiMap<String, String> {
        self.url.query_pairs().into_owned().collect()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_ref().map(|it| &it[..])
    }
}

fn parse_http_request(value: &str) -> (String, Vec<String>, Option<&str>) {
    let mut line = String::new();
    let mut first_line = "".to_string();
    let mut header_list: Vec<String> = Vec::new();
    let mut line_count = 0;
    let mut body_start_index = 0;
    for (i, ch) in value.chars().enumerate() {
        body_start_index = i;
        if ch == '\n' {
            if line.ends_with("\r") {
                line.pop();
            }
            line_count += 1;
            if line_count == 1 {
                first_line = line.trim_end().to_string();
            } else if !line.is_empty() {
                header_list.push(line.clone());
            } else {
                break;
            }
            line.clear();
        } else {
            line.push(ch);
        }
    }
    body_start_index = body_start_index + value[body_start_index..].find('\n').map(|it| it + 1).unwrap_or(0);
    // let body_end_index = body_start_index + &value[body_start_index..].rfind('\n').map(|it| it - 1).unwrap_or(0);
    let body_end_index = value.len();
    let body = if body_start_index < body_end_index { Some(&value[body_start_index..body_end_index]) } else { None };
    return (first_line, header_list, body);
}

#[cfg(test)]
mod test {
    use std::net::TcpListener;
    use super::*;

    #[test]
    fn test1() {
        let s = "GET /a/b/c HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101 Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1

";
        let http_request: HttpRequest = HttpRequest::new(s);
        println!("{:?}", http_request);
        assert!(http_request.path_match(&Regex::new(r"^/.*").unwrap()));
        assert!(http_request.path_match(&Regex::new(r"^/a/.*").unwrap()));
        assert!(http_request.path_match(&Regex::new(r"^/a/b/.*").unwrap()));
        assert!(http_request.path_match(&Regex::new(r"^/a/b/c").unwrap()));
        assert_eq!(false, http_request.path_match(&Regex::new(r"^/b/").unwrap()));
    }

    #[test]
    fn test_no_header() {
        let s = "GET /a/b/c HTTP/1.1

";
        let http_request: HttpRequest = HttpRequest::new(s);
        println!("{:?}", http_request);
    }


    #[test]
    fn test2() {
        let s = "GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101 Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1

hello body
123
321";
        let result = parse_http_request(s);

        let (first, headers, body) = dbg!(result);

        assert_eq!(first, "GET / HTTP/1.1");

        println!("{:?}", headers);
        assert!(headers.contains(&"Host: 127.0.0.1:7878".to_string()));

        assert_eq!(body.unwrap(), "hello body
123
321");
    }

    #[test]
    fn test_url_param() {
        let s = "GET /get?show_env=1&id=abc&id=efg HTTP/1.1

    ";
        let http_request: HttpRequest = HttpRequest::new(s);
        println!("{:?}", http_request);
        println!("{:?}", http_request.query());
        assert_eq!(http_request.query(), Some("show_env=1&id=abc&id=efg"));
        println!("{:?}", http_request.query_pair());
        assert_eq!(http_request.query_pair().get_vec("id"), Some(&vec![String::from("abc"), String::from("efg")]));
    }
}