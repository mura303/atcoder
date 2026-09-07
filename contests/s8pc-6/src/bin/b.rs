// https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b
//
// 提出時はこのファイルの中身をそのまま貼る。
// #[fastout] は大量出力を高速化するが、インタラクティブ問題では外すこと
// (出力がバッファに溜まって相手に届かなくなるため)。
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            ai: i64,
            bi: i64,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut sa = a.clone();
    let mut sb = b.clone();
    sa.sort_unstable();
    sb.sort_unstable();

    let entrance = sa[n / 2];
    let exit = sb[n / 2];

    let travel_a: i64 = a.iter().map(|&x| (x - entrance).abs()).sum();
    let travel_b: i64 = b.iter().map(|&x| (x - exit).abs()).sum();
    let fixed: i64 = a.iter().zip(&b).map(|(&x, &y)| y - x).sum();

    let ans = fixed + travel_a + travel_b;
    println!("{}", ans);
}
