use std::io::{self, Read};

fn main() {
    // 入力の読み込み
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim(); // 改行などを除去

    // 荷物（#）のある位置を1-indexで収集
    let mut positions = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            positions.push(i + 1);
        }
    }

    // 2つずつペアにして出力
    for pair in positions.chunks(2) {
        println!("{},{}", pair[0], pair[1]);
    }
}
