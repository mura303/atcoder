// https://atcoder.jp/contests/abc474/tasks/
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
        n: i64,
        p: [i64; n],
    }

    for i in 0..n/10{
        for j in 0..10{
            if p[(i*10+j) as usize] > (i+1)*10 as i64 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
