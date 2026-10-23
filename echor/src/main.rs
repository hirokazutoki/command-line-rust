use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Hirokazu Toki")
        .about("Rust echo")
        .get_matches();
}
