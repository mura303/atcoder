// https://atcoder.jp/contests/abc106/tasks/abc106_b
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

    let mut cnt = 0;

    // 奇数だけ1からnまでループ
    for i in (1..=n).step_by(2) {
        // println!("{} {}", i, count_divisors(i));
        if count_divisors(i) == 8 {
            cnt += 1;
            //println!("hit {} ", i);
        }
    }

    println!("{} ", cnt);

}

fn count_divisors(n: usize) -> usize {
    let mut count = 1;
    
    for i in 1..=n/2 {
        if n % i == 0 {
            count += 1; // iは約数
        }
    }
    return count;
}
