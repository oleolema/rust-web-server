use std::io;
use std::io::Write;
use std::net::TcpStream;
use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub struct HttpChannel<'a> {
    pub request: &'a HttpRequest,
    pub response: &'a mut HttpResponse,
    pub stream: &'a mut TcpStream,
    pub is_sent: bool,
}

impl<'a> HttpChannel<'a> {
    pub fn new(request: &'a HttpRequest, response: &'a mut HttpResponse, stream: &'a mut TcpStream) -> Self {
        Self { request, response, stream, is_sent: false }
    }

    pub fn send(&mut self, b: &[u8]) -> io::Result<()> {
        self.stream.write(self.response.to_string().as_bytes())?;
        self.stream.write(b)?;
        self.stream.flush()?;
        self.is_sent = true;
        Ok(())
    }
}