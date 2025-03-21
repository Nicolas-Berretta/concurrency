use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Instant,
};
use crate::math;

enum RequestError {
    ParseError { message: String },
    UnknownMethod { message: String },
    PathError { message: String }
}

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let terms = match handle_request(&http_request[0]) {
        Ok(terms) => terms,
        Err(error) => {
            let response = match error {
                RequestError::ParseError { message } => format!("HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}", message.len(), message),
                RequestError::UnknownMethod { message } => format!("HTTP/1.1 405 Method Not Allowed\r\nContent-Length: {}\r\n\r\n{}", message.len(), message),
                RequestError::PathError { message } => format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}", message.len(), message),
            };
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    };

    let start_time = Instant::now();
    let pi = math::liebniz_series(terms);
    let duration = start_time.elapsed().as_secs_f64();
    let body = format!(
        "The value of pi approximated using {} terms is: {:.15} (Time: {:.3} seconds)",
        terms, pi,duration
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_request(request_line: &str) -> Result<usize, RequestError> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 3 { // parts should be like: "GET /pi/1 HTTP/1.1"
        return Err(RequestError::ParseError {message: "Unknown method or path".to_string() });// if so, I am missing something
    }

    if parts[0] != "GET" {
        return Err(RequestError::UnknownMethod {message: "Method must be GET".to_string()});
    }
    let path = parts[1]; // /pi/1
    if path.starts_with("/pi/") {//
        let terms_str = path.strip_prefix("/pi/") ;
        return match terms_str.expect("REASON").parse::<usize>() {
            Ok(terms) => Ok(terms),
            Err(error) => {Err(RequestError::ParseError {message: error.to_string()})},
        }
    }
    Err(RequestError::PathError {message: "Invalid path".to_string()})
}