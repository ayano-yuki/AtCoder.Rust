use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut deque: VecDeque<(i64, i64)> = VecDeque::new(); 
    let mut out = String::new();

    for _ in 0..q {
        let t: i32 = it.next().unwrap().parse().unwrap();
        if t == 1 {
            let c: i64 = it.next().unwrap().parse().unwrap();
            let x: i64 = it.next().unwrap().parse().unwrap();
            if let Some(back) = deque.back_mut() {
                if back.0 == x {
                    back.1 += c;
                } else {
                    deque.push_back((x, c));
                }
            } else {
                deque.push_back((x, c));
            }
        } else {
            let mut k: i64 = it.next().unwrap().parse().unwrap();
            let mut sum: i128 = 0;
            while k > 0 {
                let (val, cnt) = deque.front().cloned().unwrap();
                if cnt <= k {
                    deque.pop_front();
                    sum += (cnt as i128) * (val as i128);
                    k -= cnt;
                } else {
                    if let Some(front) = deque.front_mut() {
                        front.1 -= k;
                    }
                    sum += (k as i128) * (val as i128);
                    k = 0;
                }
            }
            out.push_str(&format!("{}\n", sum));
        }
    }

    print!("{}", out);
}
