// https://atcoder.jp/contests/abc095/tasks/arc096_a
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
        a: i128,
        b: i128,
        c: i128,
        x: i128,
        y: i128,
    }

    let mut min_cost = 999999999999i128;

    for cnum in (0..=200_000).step_by(2) {
        let a_cost = a * (x - cnum / 2).max(0);
        let b_cost = b * (y - cnum / 2).max(0);
        let c_cost = c * cnum;
        min_cost = min_cost.min(a_cost + b_cost + c_cost);
    }

    println!("{}", min_cost);
}
