// https://atcoder.jp/contests/abc428/tasks/abc428_a
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: i64,
        a: i64,
        b: i64,
        x: i64,
    }

    let cycle = a + b;
    let full = x / cycle;
    let rest = x % cycle;
    let running = full * a + rest.min(a);
    println!("{}", running * s);
}
