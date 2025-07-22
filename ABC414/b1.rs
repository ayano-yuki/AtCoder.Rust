use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut total: u64 = 0;
    let mut result = String::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let ch = iter.next().unwrap();
        let count: u64 = iter.next().unwrap().parse().unwrap();

        total += count;
        if total > 100 {
            println!("Too Long");
            return;
        }

        result.push_str(&ch.repeat(count as usize));
    }

    println!("{}", result);
}
