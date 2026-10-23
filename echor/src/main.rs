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

    println!("{:?}", matches);
}
