# Command-line Rust (Rust練習帳)

## cat
```bash
$ cd catr

$ cargo run

# クレートを直接呼びたい場合
$ cargo run -- --lib
```

## echo
```aiignore
cd echor

$ cargo run -- --help

$ cargo run -- --version

$ cargo run 1>out 2>err

$ cargo run -- Hello world

$ cargo run -- -n Hello world

##　あらかじめ echo コマンドの実行結果を保存しておいて、expected とする
$ ./mk-outs.sh

$ cargo test
```


## Hello, world!

```bash
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

## シングルスレッドでテストを実行する
$ cargo test -- --test-threads=1
```