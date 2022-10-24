mod http_route;

use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::sync::{Arc};
use std::thread;
use http::channel::HttpChannel;
use http::request::HttpRequest;
use http::response::HttpResponse;
use http::router::{HttpRouter};
use http::utils::MyRead;

fn main() {
    match TcpListener::bind("0.0.0.0:8085") {
        Ok(listener) => {
            let mut router = HttpRouter::new();
            http_route::route(&mut router).expect("route mapping error");
            let router = Arc::new(router);
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let router = Arc::clone(&router);
                        thread::spawn(move || {
                            if let Err(e) = handle_connection(stream, &router) {
                                eprintln!("handle error: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("can't get incoming stream: {}", e);
                    }
                }
            };
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("the port binding error");
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: &HttpRouter) -> Result<(), Box<dyn Error>> {
    let input = stream.read_all_string()?;
    let input = dbg!(input);
    if input.is_empty() {
        return Ok(());
    }
    let http_request = HttpRequest::new(&input);
    let mut http_response = HttpResponse::new();
    let mut http_channel = HttpChannel::new(&http_request, &mut http_response, &mut stream);
    if let Err(e) = router.handle(&mut http_channel) {
        eprintln!("handle error: {}", e);
        http_channel.response.error().body_str(e.to_string());
    };
    if !http_channel.is_sent {
        let output = http_channel.response.to_string();
        http_channel.stream.write(dbg!(output).as_bytes())?;
        http_channel.stream.flush()?;
    }
    Ok(())
}



