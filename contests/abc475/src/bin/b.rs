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
        a: [i64; n],
    }

    let mut one_yen = 0;
    let mut ten_yen = 0;
    let mut hundred_yen = 0;

    for i in 0..n {
        let price: i64 = a[i];
        let change = (1000 - price % 1000) % 1000;

        one_yen += change % 10;
        ten_yen += change / 10 % 10;
        hundred_yen += change / 100;
    }

    println!("{one_yen} {ten_yen} {hundred_yen}");

}
