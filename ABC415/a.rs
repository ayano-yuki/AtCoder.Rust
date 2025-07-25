use std::io::{self, BufRead};

fn main() {
    // 標準入力をロック
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1行目：Nを読み込む
    let n: usize = lines
        .next().unwrap().unwrap()
        .trim().parse().unwrap();

    // 2行目：A1 A2 ... AN
    let a: Vec<i32> = lines
        .next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // 3行目：X
    let x: i32 = lines
        .next().unwrap().unwrap()
        .trim().parse().unwrap();

    // 確認出力
    // println!("N = {}", n);
    // println!("A = {:?}", a);
    // println!("X = {}", x);

    // XがAの中にあるか判定
    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
