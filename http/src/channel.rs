use std::net::TcpStream;
use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub struct HttpChannel<'a> {
    pub request: &'a HttpRequest,
    pub response: &'a mut HttpResponse,
    pub stream: &'a mut TcpStream,
}

impl<'a> HttpChannel<'a> {
    pub fn new(request: &'a HttpRequest, response: &'a mut HttpResponse, stream: &'a mut TcpStream) -> Self {
        Self { request, response, stream }
    }
}