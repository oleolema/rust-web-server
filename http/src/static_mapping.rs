use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;
use crate::channel::HttpChannel;
use crate::request::{HttpMethod, HttpRequest};
use crate::router::RequestMapping;
use crate::utils::MyRead;


struct StaticMapping {
    static_path: Vec<Regex>,
}

impl StaticMapping {
    fn new(static_path: Vec<Regex>) -> Self {
        Self {
            static_path
        }
    }
}

impl RequestMapping for StaticMapping {
    fn predicate(&self, http_request: &HttpRequest) -> bool {
        http_request.method == HttpMethod::GET
            && self.static_path.iter().find(|it| http_request.path_match(it)).is_some()
    }

    fn handle(&mut self, channel: &mut HttpChannel) -> Result<(), Box<dyn Error>> {
        let path = format!(".{}", &channel.request.path);
        let mut file = File::open(&path)
            .or(Err(format!("the file opening error {}", path)))?;
        const BUFFER_LEN: usize = 1024;
        let mut buffer = [0; BUFFER_LEN];

        loop {
            let n = file.read(&mut buffer)?;
            channel.stream.write(&buffer[0..n])?;

            if n < BUFFER_LEN {
                channel.stream.flush()?;
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::net::{TcpListener, TcpStream};
    use crate::channel::HttpChannel;
    use crate::request::HttpRequest;
    use crate::response::HttpResponse;
    use crate::router::RequestMapping;
    use crate::static_mapping::StaticMapping;
    use crate::utils::{MyRead};

    #[test]
    fn test_static_mapping() {
        let http_request: HttpRequest = HttpRequest::new("GET /hello.html HTTP/1.1\n\n");
        let mut http_response = HttpResponse::new();
        let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
        let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
        let mut incoming = listener.incoming();
        let mut channel = HttpChannel::new(&http_request, &mut http_response, &mut stream);
        let mut static_mapping = StaticMapping::new(vec![]);
        static_mapping.predicate(&http_request);
        static_mapping.handle(&mut channel).expect("TODO: panic message");
        let s = incoming.next();
        let mut s = s.unwrap().unwrap();
        let s = s.read_all_string();
        println!("{}", s.unwrap());
    }

    #[test]
    fn test1() {
        let a = fs::read_to_string("./hello.html").unwrap();
        println!("{}", a);
    }
}

