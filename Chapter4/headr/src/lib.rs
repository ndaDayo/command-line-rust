use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1")
        .author("nda dayo")
        .about("rust head")
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}
