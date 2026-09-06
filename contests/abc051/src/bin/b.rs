// https://atcoder.jp/contests/abc051/tasks/abc051_b
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
        k: i64, s: i64,
    }

    let mut count = 0;

    for x in 0..=k {
        for y in 0..=k{
            if s - x - y >= 0 && s - x - y <= k {
                count += 1
            }
        }
    }

    println!("{}", count);
}
