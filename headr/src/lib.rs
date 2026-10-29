use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Lines number")
                .takes_value(true)
                .conflicts_with("bytes"), // 効いていない？
        )
        .arg(
            Arg::with_name("bytes")
                .short("b")
                .long("bytes")
                .help("bytes")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of("lines").unwrap_or("10").parse()?, // Assistant AI
        bytes: matches.value_of("bytes").map(|s| s.parse()).transpose()?, // Assistant AI
    })
}