use std::net::TcpListener;
use std::thread;
use tp1::request_handler;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    thread::scope(|s| {
        listener.incoming()
            .filter_map(|stream| stream.ok())
            .for_each(|stream| {
                s.spawn(move || {
                    request_handler::handle_connection(stream);
                });
            });
    });
}