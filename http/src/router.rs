use std::error::Error;
use regex::{Regex};
use crate::channel::HttpChannel;
use crate::request::{HttpMethod, HttpRequest};

pub trait RequestMapping {
    fn predicate(&self, http_request: &HttpRequest) -> bool;

    fn handle(&self, http_channel: &mut HttpChannel) -> Result<(), Box<dyn Error>>;
}

pub enum RegexMapping<F>
    where F: Fn(&mut HttpChannel) -> Result<(), Box<dyn Error>>
{
    // matches GET requests
    GET(Regex, F),
    // matches POST requests
    POST(Regex, F),
    // matches ALL requests
    REQUEST(Regex, F),
}

impl<F> RequestMapping for RegexMapping<F>
    where F: Fn(&mut HttpChannel) -> Result<(), Box<dyn Error>>
{
    fn predicate(&self, http_request: &HttpRequest) -> bool {
        match self {
            RegexMapping::GET(regex, _)
            if http_request.method == HttpMethod::GET && http_request.path_match(regex) => true,
            RegexMapping::POST(regex, _)
            if http_request.method == HttpMethod::POST && http_request.path_match(regex) => true,
            RegexMapping::REQUEST(regex, _)
            if http_request.path_match(regex) => true,
            _ => false,
        }
    }

    fn handle(&self, http_channel: &mut HttpChannel) -> Result<(), Box<dyn Error>> {
        match self {
            RegexMapping::GET(_, f) => f(http_channel),
            RegexMapping::POST(_, f) => f(http_channel),
            RegexMapping::REQUEST(_, f) => f(http_channel),
        }
    }
}

pub struct HttpRouter<'b> {
    mappings: Vec<Box<dyn RequestMapping + Send + Sync + 'b>>,
}

impl<'b> HttpRouter<'b> {
    pub fn new() -> Self {
        HttpRouter { mappings: Vec::new() }
    }

    pub fn route(&mut self, request_mapping: Box<dyn RequestMapping + Send + Sync + 'b>) -> &mut Self {
        self.mappings.push(request_mapping);
        self
    }

    pub fn handle(&self, http_channel: &mut HttpChannel) -> Result<(), Box<dyn Error>> {
        self.mappings.iter().find(|it| it.predicate(http_channel.request))
            .map(|it| it.handle(http_channel))
            .unwrap_or_else(|| {
                http_channel.response.not_found();
                Ok(())
            })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::request::*;
    use crate::response::HttpResponse;
    use crate::utils::get_stream;

    #[test]
    fn test1() {
        let mut router = HttpRouter::new();
        router.route(Box::new(RegexMapping::GET(Regex::new(r"^/hello").unwrap(),
                                                |channel| {
                                                    channel.response.body_str(String::from("hello"));
                                                    Ok(())
                                                })));

        router.route(Box::new(RegexMapping::GET(Regex::new(r"^/world").unwrap(),
                                                |channel| {
                                                    channel.response.body_str(String::from("world"));
                                                    Ok(())
                                                })));
        let http_request: HttpRequest = HttpRequest::new("GET /hello HTTP/1.1\n\n");
        let mut http_response = HttpResponse::new();
        let mut stream = get_stream(8090);
        let mut channel = HttpChannel::new(&http_request, &mut http_response, &mut stream);

        router.handle(&mut channel).unwrap();
        let response = channel.response;
        assert_eq!("hello", response.body_str_ref().unwrap())
    }

    #[test]
    fn test2() {
        let a = || 1;
        a();
        // let body = String::from("hello");
        // let a:Fn(HttpRequest)->HttpResponse = |r| HttpResponse::ok().body(&body);
        // a(HttpRequest::new(""));
    }
}