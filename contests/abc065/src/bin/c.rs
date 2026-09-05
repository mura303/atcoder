// https://atcoder.jp/contests/abc065/tasks/arc076_a
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
        m: usize,
    }
    const MOD: i64 = 1_000_000_007;

    let fac = |x: usize| -> i64 {
        let mut res = 1_i64;
        for i in 1..=x {
            res = res * i as i64 % MOD;
        }
        res
    };

    let ans = if n == m {
        fac(n) * fac(m) % MOD * 2 % MOD
    } else if n + 1 == m || m + 1 == n {
        fac(n) * fac(m) % MOD
    } else {
        0
    };

    println!("{}", ans);
}
