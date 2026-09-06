// https://atcoder.jp/contests/abc122/tasks/abc122_b
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
        s: String,
    }

    let mut ans = 0usize;
    let mut cur = 0usize;

    for ch in s.chars() {
        if matches!(ch, 'A' | 'C' | 'G' | 'T') {
            cur += 1;
            ans = ans.max(cur);
        } else {
            cur = 0;
        }
    }

    println!("{}", ans);
}
