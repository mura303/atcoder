// https://atcoder.jp/contests/abc141/tasks/abc141_a
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
        weather: String,
    }

    let next = match weather.as_str() {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        "Rainy" => "Sunny",
        _ => unreachable!(),
    };
    println!("{}", next);
}
