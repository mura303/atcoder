// https://atcoder.jp/contests/abc198/tasks/abc198_c
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
        r: i64,
        x: i64,
        y: i64,
    }

    // r^2 が x^2 + y^2 と等しければ ans = 1
    let ans = if r * r == x * x + y * y {
        1
    } else if r * r > x * x + y * y {
        2
    } else {
        // 2から始めて、r^2 が x^2 + y^2 以上になるまでの最小の整数 n を求める
        let mut n = 2;
        while r * r * n * n < x * x + y * y {
            n += 1;
        }
        n
    };
    println!("{}", ans);
}
