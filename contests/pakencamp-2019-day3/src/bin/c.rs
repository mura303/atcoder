// https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c
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
        a: [[i64; m]; n],
    }

    let mut ans = 0i64;
    for i in 0..m {
        for j in (i + 1)..m {
            let mut total = 0i64;
            for k in 0..n {
                total += a[k][i].max(a[k][j]);
            }
            ans = ans.max(total);
        }
    }

    println!("{}", ans);
}
