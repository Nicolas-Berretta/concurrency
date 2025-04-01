mod server;
mod math;
mod thread_pool;
mod request_handler;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <mode>", args[0]);
        eprintln!("Modes: multi, pool");
        return;
    }

    let mode = &args[1];

    match mode.as_str() {
        "multi" => server::start_server(),
        "pool" => thread_pool::start_server(),
        _ => eprintln!("Invalid mode! Use 'multi' or 'pool'"),
    }
}
