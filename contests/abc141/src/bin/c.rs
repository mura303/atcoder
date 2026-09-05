// https://atcoder.jp/contests/abc141/tasks/abc141_c
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
        k: i64,
        q: usize,
        winners: [Usize1; q],
    }

    let mut correct = vec![0_i64; n];
    for winner in winners {
        correct[winner] += 1;
    }

    for count in correct {
        println!(
            "{}",
            if k - q as i64 + count > 0 {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
