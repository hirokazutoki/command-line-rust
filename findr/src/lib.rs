use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    //
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .about("Rust findr")
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

    // TODO: p.176ff
    Ok(Config {
        //
    })
}

pub fn run(config: Config) -> MyResult<()> {
    print!("{:?}", config);
    Ok(())
}