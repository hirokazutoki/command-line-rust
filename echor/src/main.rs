use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .help("Input text")
                .required(true)
                // 少なくとも１つ以上は指定する必要がある
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                // -n という短い名前だけを持つフラグ
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    // 引数 text から Option<Vec<String>> を取得する
    let text = matches.values_of_lossy("text").unwrap();

    // 引数　omit_newline から bool を取得する
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    println!("{}{}", text.join(" "), ending);
}
