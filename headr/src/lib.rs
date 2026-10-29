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

// コマンドライン引数は全て文字列として扱われるため、linesとbytesが有効な整数値であるかの確認が必要。
fn parse_positive_int(val: &str) -> MyResult<usize> {
    unimplemented!()
}

#[test]
fn test_parse_positive_int() {
    // ３は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}