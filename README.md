# Command-line Rust (Rust練習帳)

## uniq
```bash
$ cd uniqr

$ cargo run -- tests/inputs/one.txt
a

$ cargo run -- - < tests/inputs/one.txt
a

$ cargo run -- -c tests/inputs/one.txt 
   1 a

$ cargo run -- -c tests/inputs/two.txt
   2 a

$ cargo run -- -c tests/inputs/three.txt
   2 a
   2 b
   1 a
   3 c
   1 a
   4 d

$ sort tests/inputs/three.txt | cargo run -- -c
   4 a
   2 b
   3 c
   4 d

$ sort tests/inputs/three.txt | cargo run -- -c - out
$ cat out
   4 a
   2 b
   3 c
   4 d

# クローン対象の理解を深めるためのコマンドテスト
$ uniq tests/inputs/empty.txt

$ uniq tests/inputs/one.txt    
a

# -cフラグでカウントを表示
$ uniq -c  tests/inputs/one.txt
   1 a

$ cat tests/inputs/two.txt 
a
a

$ uniq -c tests/inputs/two.txt
   2 a

$ cat tests/inputs/three.txt 
a
a
b
b
a
c
c
c
a
d
d
d
d

# 新しい文字列に出くわすたびに１から数え直す
$ uniq -c tests/inputs/three.txt
   2 a
   2 b
   1 a
   3 c
   1 a
   4 d

#　sortで並び替えた後だと、実際の一意な値の数がわかる
$ sort tests/inputs/three.txt | uniq -c
   4 a
   2 b
   3 c
   4 d

$ cat tests/inputs/skip.txt 
a

a
b

# 空行があっても、カウンターをリセットすることになる
$ uniq -c tests/inputs/skip.txt
   1 a
   1 
   1 a
   1 b

# 出力をファイルに書き出すこともできる
$ uniq -c tests/inputs/two.txt out
$ cat out
   2 a

# 標準入力をファイルに書き込むには、入力ファイル名にダッシュを使う
$ cat tests/inputs/two.txt | uniq -c - out
$ cat out                                 
   2 a
```

## wc
```bash

$ cargo run -- --help

$ cargo test

$ cargo run -- -cm tests/inputs/fox.txt
error: The argument '--bytes' cannot be used with '--chars'

USAGE:
    wcr --bytes --chars

$ cargo run -- -l tests/inputs/*.txt
       4 tests/inputs/atlamal.txt
       0 tests/inputs/empty.txt
       1 tests/inputs/fox.txt
       5 total

$ cargo run -- tests/inputs/*.txt
       4      29     177 tests/inputs/atlamal.txt
       0       0       0 tests/inputs/empty.txt
       1       9      48 tests/inputs/fox.txt
       5      38     225 total


# クローン対象の理解を深めるためのコマンドテスト
$ wc tests/inputs/empty.txt 
       0       0       0 tests/inputs/empty.txt
       
$ cat -te tests/inputs/fox.txt 
The  quick brown fox^Ijumps over   the lazy dog.$

$ cat -te tests/inputs/fox.txt 
The  quick brown fox^Ijumps over   the lazy dog.$

$ wc tests/inputs/atlamal.txt
       4      29     177 tests/inputs/atlamal.txt

# -lフラグで行数のみを表示
$ wc -l tests/inputs/atlamal.txt
       4 tests/inputs/atlamal.txt

# -cでバイト数のみ、-wで単語のみを表示
$ wc -w -c tests/inputs/atlamal.txt
      29     177 tests/inputs/atlamal.txt

# フラグの順番を変えても出力は変わらない
$ wc -wc tests/inputs/atlamal.txt
$ wc -cw tests/inputs/atlamal.txt

# -mフラグで文字数を表示
$ wc -m tests/inputs/atlamal.txt
     159 tests/inputs/atlamal.txt

# 位置引数が与えられない場合、wcは標準入力から読み込み、ファイル名を表示しません
$ cat tests/inputs/atlamal.txt| wc -lc
       4     177

$ wc tests/inputs/*.txt       
       4      29     177 tests/inputs/atlamal.txt
       0       0       0 tests/inputs/empty.txt
       1       9      48 tests/inputs/fox.txt
       5      38     225 total

$ wc tests/inputs/fox.txt blargh tests/inputs/atlamal.txt 
       1       9      48 tests/inputs/fox.txt
wc: blargh: open: No such file or directory
       4      29     177 tests/inputs/atlamal.txt
       5      38     225 total

#　標準エラーをerrファイルにリダイレクト
$ wc tests/inputs/fox.txt blargh tests/inputs/atlamal.txt 2>err
       1       9      48 tests/inputs/fox.txt
       4      29     177 tests/inputs/atlamal.txt
       5      38     225 total
$ cat err
wc: blargh: open: No such file or directory
```

## head
```bash
$ cd headr

$ cargo test

$ cargo run -- -n 9 tests/inputs/twelve.txt

$ cargo run -- -c 2 tests/inputs/twelve.txt

$ cargo run -- -n 9 -c 2 tests/inputs/twelve.txt

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

# Öが2バイト長の文字列
$ head -c 2 tests/inputs/one.txt 
Ö
$ head -c 1 tests/inputs/one.txt
�

$ cargo run -- -n 1 tests/inputs/twelve.txt
one

$ cargo run -- -n 11 tests/inputs/twelve.txt
one
two
three
four
five
six
seven
eight
nine
ten
eleven
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