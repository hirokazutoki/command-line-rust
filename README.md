# Command-line Rust (Rust練習帳)

## head
```bash
$ cd headr

$ cargo test

$ cargo run -- -l 20 tests/inputs/twelve.txt

$ cargo run -- -b 4 tests/inputs/twelve.txt

$ cargo run -- -l 20 -b 4 tests/inputs/twelve.txt

$ cargo test test_parse_positive_int
$ cargo run -- -n 3 tests/inputs/one.txt
$ cargo run -- -c 4 tests/inputs/*.txt

# -n や -c の値が正の整数として解析できない場合、エラーで停止する
$ cargo run -- -n blargh tests/inputs/one.txt
$ cargo run -- -c 0 tests/inputs/one.txt

# -n と -c の併用は不可
$ cargo run -- -n 1 -c 1 tests/inputs/one.txt

$ cargo run -- blargh tests/inputs/one.txt
Failed to open blargh: No such file or directory (os error 2)
Opened tests/inputs/one.txt
```

## cat
```bash
$ cd catr

$ cargo run

$ cargo run -- --help

$ touch cant-touch-this && chmod 000 cant-touch-this
$ cargo run -- blargh cant-touch-this tests/inputs/fox.txt
$ cargo test skips_bad_file

# -b と -n の併用は不可
$ cargo run -- -b -n tests/inputs/fox.txt

# 全て同じ出力結果
$ cargo run -- tests/inputs/fox.txt
$ cat tests/inputs/fox.txt | cargo run
$ cargo run -- - < tests/inputs/fox.txt 

$ cargo run -- -n tests/inputs/fox.txt
$ cargo run -- -n tests/inputs/the-bustle.txt

$ cargo run -- -b tests/inputs/the-bustle.txt

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