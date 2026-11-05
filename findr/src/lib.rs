use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .about("Rust find")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("Search paths")
                .default_value("."),
        )
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .takes_value(true)
                .possible_values(&["f", "d", "l"]), // 新しく知った書き方
        )
        .get_matches();

    let mut paths = Vec::new();
    if let Some(path) = matches.value_of("path") {
        paths.push(path.to_string());
    }

    let mut entry_types = Vec::new();
    if matches.value_of("type").unwrap_or("") == "f" {
        entry_types.push(File);
    } else if matches.value_of("type").unwrap_or("") == "d" {
        entry_types.push(Dir);
    } else if matches.value_of("type").unwrap_or("") == "l" {
        entry_types.push(Link);
    }

    Ok(Config {
        paths,
        names: Vec::new(),
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}