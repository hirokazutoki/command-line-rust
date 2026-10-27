use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .arg(
            Arg::with_name("files")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}