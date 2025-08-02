use std::io;

fn main() {
    // 標準入力から読み込む
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.trim().split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // 品物の合計サイズを計算
    let sum: usize = a.iter().sum();

    // 判定して出力
    if sum <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
