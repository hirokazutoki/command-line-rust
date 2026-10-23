fn main() {
    // Args が Display トレイトを実装していない。
    // 構造体をデバッグ用の形式で表示するために {:?} を使う。
    println!("{:?}", std::env::args());
}
