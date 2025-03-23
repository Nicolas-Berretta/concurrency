use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Instant
};
use crate::math;

enum RequestError {
    ParseError { message: String },
    UnknownMethod { message: String },
    PathError { message: String }
}

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }

    thread::scope(|s| {
        listener.incoming()
            .filter_map(|stream| stream.ok())
            .for_each(|stream| {
                s.spawn(move || {
                    handle_connection(stream);
                });
            });
    });

}

fn handle_connection(mut stream: TcpStream) {
    let start_time = Instant::now();
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.is_empty() {
        let response = format_response("400 Bad Request", "Empty request received");
        stream.write_all(response.as_bytes()).expect("Failed to write response");
        return;
    }

    let terms = match handle_request(&http_request[0]) {
        Ok(terms) => terms,
        Err(error) => {
            let response = match error {
                RequestError::ParseError { message } => format_response("400 Bad Request", &message),
                RequestError::UnknownMethod { message } => format_response("405 Method Not Allowed", &message),
                RequestError::PathError { message } => format_response("404 Not Found", &message),
            };
            stream.write_all(response.as_bytes()).expect("Failed to write response");
            return;
        }
    };
    let pi = math::liebniz_series(terms);
    let duration = start_time.elapsed().as_secs_f64();
    let body = format!(
        "The value of pi approximated using {} terms is: {:.15} (Time: {:.3} seconds)",
        terms, pi, duration
    );
    let response = format_response("200 OK", &body);
    stream.write_all(response.as_bytes()).expect("Failed to write response");
}

fn handle_request(request_line: &str) -> Result<usize, RequestError> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 3 { // parts should be like: "GET /pi/1 HTTP/1.1"
        return Err(RequestError::ParseError {message: "Unknown method or path".to_string() });// if so, I am missing something
    }
    if parts[0] != "GET" {
        return Err(RequestError::UnknownMethod {message: "Method must be GET".to_string()});
    }
    if let Some(terms_str) = parts[1].strip_prefix("/pi/") {
        return terms_str.parse::<usize>().map_err(|error| RequestError::ParseError {message: error.to_string()});
    }
    Err(RequestError::PathError {message: "Invalid path".to_string()})
}

fn format_response(status: &str, body: &str) -> String {
    format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", status, body.len(), body)
}
