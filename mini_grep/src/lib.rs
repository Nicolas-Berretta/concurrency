use std::fs;
use std::error::Error;
use std::thread;
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub method: String,
    pub query: String,
    pub file_paths: Box<[String]>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let method = args[1].clone();
        let query = args[2].clone();
        let file_paths = args[3..].to_vec().into_boxed_slice();

        Ok(Config { method, query, file_paths })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    match config.method.as_str() {
        "seq" => sequential_search(config),
        "conc" => concurrency_search(config),
//        "c-chunk" => c_chuck_search(config),
        _ => Err("Invalid method".into())

    }
}

fn sequential_search(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = config.file_paths
        .iter()
        .try_for_each(|file_path| {
            let contents = fs::read_to_string(file_path)?;
            handle_file(config.query.as_str(),&contents);

            Ok::<(), Box<dyn Error>>(())
        });
    Ok(())
}

fn concurrency_search(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = Arc::new(config.query);

    let handles: Vec<_> = config.file_paths
        .as_ref()
        .iter()
        .map(|file_path| {
            let query = Arc::clone(&query);
            let file_path = file_path.clone();

            thread::spawn(move || {
                let contents = fs::read_to_string(file_path)?;
                handle_file(&query, &contents);
                Ok::<(), Box<dyn Error + Send + Sync>>(())
            })
        })
        .collect();

    for handle in handles {
        if let Err(err) = handle.join().unwrap() {
            return Err(err);
        }
    }

    Ok(())
}

// fn c_chuck_search(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {

// }

fn handle_file(query: &str, contents: &str) {
    contents
        .lines()
        .for_each(|line| {
            if line.contains(query) {println!("{}",line)}
        });
}

// cargo run -- <seq/conc/c-chunk> <word> <[book.txt, book.txt, book.txt, book.txt]>


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
