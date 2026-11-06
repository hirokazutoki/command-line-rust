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
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["f", "d", "l"]), // 新しく知った書き方
        )
        .get_matches();

    let mut paths = Vec::new();
    if let Some(path) = matches.value_of("path") {
        paths.push(path.to_string());
    }

    let names = matches
        .values_of_lossy("name")
        // ユーザーがnamesにSome(vals)を与えた場合、Option::mapを使う
        .map(|vals| {
            // 値を反復処理
            vals.into_iter()
                .map(|name| {
                    // 与えられたnameで新しいRegexの作成を試みる。結果は、Result型。
                    Regex::new(&name)
                        // エラーが発生すれば、無効な正規表現であることを伝えるエラーメッセージを作成
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                //　結果をベクタにする
                .collect::<Result<Vec<_>, _>>()
        })
        // Option<Result<T, E>>をResult<Option<T>, E>に変換
        .transpose()?
        // Someの値か、デフォルトの値を返す
        .unwrap_or_default();

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
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}