use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader}
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Show counts")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
            .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    // 前の行を保持する可変な変数
    let mut previous = String::new();
    //　出現回数を保持する可変な変数
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        // 現在の行と前の行を比較する。可能な限り末尾の空白を除去をしている。
        if line.trim_end() != previous.trim_end() {
            // 出現回数が0より大きい場合、出現回数と前の行を出力する。
            if count > 0 {
                print!("{:>4} {}", count, previous)
            }
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    if count > 0 {
        print!("{:>4} {}", count, previous)
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}