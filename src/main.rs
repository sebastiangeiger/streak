extern crate hyper;
extern crate html5ever;

use std::env;
use std::io::Write;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::status::StatusCode as StatusCode;
use hyper::client::response::Response;
use hyper::HttpError as HttpError;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn main() {
    match get_streak() {
        Ok(result) => println!("{}",result.as_slice()),
        Err(error) => println_stderr!("{}",error.as_slice())
    }
}

fn get_streak() -> Result<String, String> {
    let github_username : String = try!(get_github_username());
    let github_url = format!("https://github.com/{}", github_username);
    let body = try!(get_body(&github_url));
    let streak = try!(parse_out_streak(&body));
    Ok(streak)
}

fn get_github_username() -> Result<String, String> {
    match env::var("GITHUB_USERNAME") {
        Ok(github_username) => Ok(github_username),
        Err(_) => Err("Please set ENV['GITHUB_USERNAME']!".to_string())
    }
}

fn get_body(url : &String) -> Result<String, String> {
    let mut client = Client::new();

    let response : Result<Response, HttpError> = client.get(url.as_slice())
        .header(Connection(vec![ConnectionOption::Close]))
        .send();

    match response {
        Ok(mut res) => {
            let mut body = String::new();
            match res.status {
                StatusCode::Ok => {
                    res.read_to_string(&mut body).unwrap();
                    Ok(body)
                },
                _ => {
                    let mut message = "HTTP request failed, server returned ".to_string();
                    message.push_str(res.status.canonical_reason().unwrap());
                    Err(message)
                }
            }
        },
        Err(_) => Err("HTTP request failed, could not reach server".to_string())
    }
}

fn parse_out_streak(html : &String) -> Result<String,String> {
    //Use Html5ever here
    Ok("TODO")
}
