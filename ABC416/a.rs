use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let s = iter.next().unwrap();

    let sub = &s[(l - 1)..r];

    if sub.chars().all(|c| c == 'o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
