use std::io::{ Write};
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use http::channel::HttpChannel;
use http::request::HttpRequest;
use http::response::HttpResponse;
use http::router::HttpRouter;
use http::utils::MyRead;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let mut router = HttpRouter::new();
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream, &mut router).unwrap();
        }
    };
}

fn handle_connection(mut stream: TcpStream, router: &mut HttpRouter) -> Result<(), Box<dyn Error>> {
    let input = stream.read_all_string()?;
    let http_request = HttpRequest::new(&input);
    dbg!(&input);

    let mut http_response = HttpResponse::new();
    let mut http_channel = HttpChannel::new(&http_request, &mut http_response, &mut stream);

    router.handle(&mut http_channel).unwrap_or_else(|_| {
        http_response.error();
    });

    stream.write(dbg!(http_response.to_string()).as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}
