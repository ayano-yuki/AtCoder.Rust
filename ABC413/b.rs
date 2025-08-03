use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let strings: Vec<String> = lines.take(n).collect();

    let mut unique_concats = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let combined = format!("{}{}", strings[i], strings[j]);
                unique_concats.insert(combined);
            }
        }
    }

    println!("{}", unique_concats.len());
}
