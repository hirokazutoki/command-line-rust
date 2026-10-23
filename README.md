# Command-line Rust (Rust練習帳)

## Hello, world!

```bash
# ファイルをコンパイル
$ cd hello

# コンパイル&実行
$ cargo run
cargo run
   Compiling hello v0.1.0 (/Users/hoge/command-line-rust/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/hello`
Hello, world!

## ステータスを表示させたくない場合
$ cargo run --quiet
Hello, world!

## 実行ファイルを直接実行する
$ ./target/debug/hello 
Hello, world!

# test
$ cargo test
```