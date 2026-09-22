// https://atcoder.jp/contests/abc475/tasks/
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
        s: usize,
        length: usize,
        a: [usize; n - 1],
    }

    let s = s - 1;

    let mut p = vec![0; n + 1];
    for i in 0..n - 1 {
        p[i + 1] = p[i] + a[i];
    }

    let mut ans = 1;

    for l in 0..=s {
        for r in s..n {
            let dist = (p[r] - p[l]) + std::cmp::min(p[s] - p[l], p[r] - p[s]);
            if dist <= length {
                ans = std::cmp::max(ans, (r - l + 1) as i64);
            }
        }
    }

    println!("{}", ans);        

}
