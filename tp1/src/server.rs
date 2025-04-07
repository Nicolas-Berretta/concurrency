use std::{ net::{TcpListener}, thread};
use crate::request_handler::handle_connection;


pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
