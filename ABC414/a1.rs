use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1行目の入力: N L R
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let l: i32 = first_iter.next().unwrap().parse().unwrap();
    let r: i32 = first_iter.next().unwrap().parse().unwrap();

    // 座標のベクタを格納する
    let mut coordinates = Vec::with_capacity(n);

    // N行の (Xi, Yi) を読み取る
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        coordinates.push((x, y));
    }

    // 入力確認用の出力（必要に応じて削除）
    // println!("N = {}, L = {}, R = {}", n, l, r);
    // for (i, (x, y)) in coordinates.iter().enumerate() {
    //     println!("Point {}: ({}, {})", i + 1, x, y);
    // }
    
     // 条件 Xi <= L && Yi >= R を満たす点の数をカウント
    let mut count = 0;
    for (x, y) in &coordinates {
        if *x <= l && *y >= r {
            count += 1;
        }
    }

    // 結果を出力
    println!("{}", count);
}
