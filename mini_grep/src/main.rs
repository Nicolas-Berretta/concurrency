use std::env;
use std::process;
use std::time::Instant;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    let start_time = Instant::now();
    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    let duration = start_time.elapsed().as_secs_f64();
    println!("time elapsed: {} ms", duration);
}
