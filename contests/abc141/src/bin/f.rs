// https://atcoder.jp/contests/abc141/tasks/abc141_f
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
        a: [u64; n],
    }

    let total_xor = a.iter().fold(0, |acc, &x| acc ^ x);
    let mut basis = [0_u64; 60];

    for mut x in a.into_iter().map(|x| x & !total_xor) {
        for bit in (0..60).rev() {
            if x >> bit & 1 == 0 {
                continue;
            }
            if basis[bit] == 0 {
                basis[bit] = x;
                break;
            }
            x ^= basis[bit];
        }
    }

    let mut best = 0;
    for bit in (0..60).rev() {
        best = best.max(best ^ basis[bit]);
    }

    println!("{}", total_xor + 2 * best);
}
