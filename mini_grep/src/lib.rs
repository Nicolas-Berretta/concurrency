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
        "c-chunk" => c_chuck_search(config),
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

fn c_chuck_search(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = Arc::new(config.query);

    let handles: Vec<_> = config.file_paths
        .iter()
        .map(|file_path| {
            let query = Arc::clone(&query);
            let file_path = file_path.clone();

            thread::spawn(move || {
                let contents = fs::read_to_string(&file_path)?;
                let lines: Vec<&str> = contents.lines().collect();
                let chunks = lines.chunks(100);

                for chunk in chunks {
                    let query = Arc::clone(&query);
                    let chunk_string = chunk.join("\n");
                    thread::spawn(move || {
                        handle_file(&query, &chunk_string);
                    });
                }
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

fn handle_file(query: &str, contents: &str) {
    contents
        .lines()
        .for_each(|line| {
            if line.contains(query) {println!("{}",line)}
        });
}

// cargo run -- <seq/conc/c-chunk> <word> <[book.txt, book.txt, book.txt, book.txt]>


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_test_file(file_name: &str, content: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_sequential_search() -> Result<(), Box<dyn Error>> {
        let file_name = "test_seq.txt";
        let content = "Hello Rust!\nThis is a test file.\nRust is great!";
        create_test_file(file_name, content)?;

        let args = vec![
            "mini_grep".to_string(),
            "seq".to_string(),
            "Rust".to_string(),
            file_name.to_string(),
        ];

        let config = Config::build(&args)?;
        let result = run(config);

        fs::remove_file(file_name)?;
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_concurrent_search() -> Result<(), Box<dyn Error>> {
        let file_name = "test_conc.txt";
        let content = "Rust is concurrent!\nConcurrency is powerful.\nLet's test it!";
        create_test_file(file_name, content)?;

        let args = vec![
            "mini_grep".to_string(),
            "conc".to_string(),
            "Concurrency".to_string(),
            file_name.to_string(),
        ];

        let config = Config::build(&args)?;
        let result = run(config);

        fs::remove_file(file_name)?;
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_chunked_search() -> Result<(), Box<dyn Error>> {
        let file_name = "test_chunk.txt";
        let content = "Chunk testing.\nBreaking it into smaller pieces.\nChunk processing!";
        create_test_file(file_name, content)?;

        let args = vec![
            "mini_grep".to_string(),
            "c-chunk".to_string(),
            "Chunk".to_string(),
            file_name.to_string(),
        ];

        let config = Config::build(&args)?;
        let result = run(config);

        fs::remove_file(file_name)?;
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_invalid_method() {
        let args = vec![
            "mini_grep".to_string(),
            "invalid_method".to_string(),
            "Rust".to_string(),
            "file.txt".to_string(),
        ];

        let config = Config::build(&args).unwrap();
        let result = run(config);

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_arguments() {
        let args = vec!["mini_grep".to_string()];
        let result = Config::build(&args);

        assert!(result.is_err());
    }
}