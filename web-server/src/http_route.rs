use std::error::Error;
use std::thread;
use std::time::Duration;
use regex::Regex;
use http::channel::HttpChannel;
use http::router::{HttpRouter, RegexMapping};
use http::static_mapping::StaticMapping;

pub fn route(router: &mut HttpRouter) -> Result<(), Box<dyn Error>> {
    router.route(Box::new(StaticMapping::new()));
    router.route(Box::new(
        RegexMapping::GET(Regex::new(r"^/abc$")?, my_handler)));
    router.route(Box::new(
        RegexMapping::POST(Regex::new(r"^/post$")?, post_handler)));
    router.route(Box::new(
        RegexMapping::GET(Regex::new(r"^/post$")?, post_handler)));
    Ok(())
}

fn my_handler(channel: &mut HttpChannel) -> Result<(), Box<dyn Error>> {
    thread::sleep(Duration::from_millis(5000));
    channel.response.body_str(String::from("abc"));
    Ok(())
}

fn post_handler(channel: &mut HttpChannel) -> Result<(), Box<dyn Error>> {
    println!("method: {:?}", channel.request.method);
    println!("path: {:?}", channel.request.path);
    println!("headers: {:?}", channel.request.headers);
    println!("body: {:?}", channel.request.body());
    channel.response
        .header("Content-Type".to_string(), "application/json".to_string())
        .body_str(String::from("{
  \"name\": \"my-name\",
  \"params\": [1, 2, 3]
}"));
    Ok(())
}