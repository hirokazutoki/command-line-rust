# Command-line Rust (Rust練習帳)

## echo
```aiignore
cd echor

$ cargo run
Args { inner: ["target/debug/echor"] }

$ cargo run Hello world
Args { inner: ["target/debug/echor", "Hello", "world"] }

$ cargo run Hello world -n
Args { inner: ["target/debug/echor", "Hello", "world", "-n"] }

$ cargo run -- -n Hello world
Args { inner: ["target/debug/echor", "-n", "Hello", "world"] }
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