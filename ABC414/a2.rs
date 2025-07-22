use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        coordinates: [(i32, i32); n],
    }

    let count = coordinates.iter().filter(|&&(x, y)| x <= l && y >= r).count();
    println!("{}", count);
}
