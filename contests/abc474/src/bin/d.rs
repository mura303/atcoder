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
        n: usize,
        a: [i64; n],
        b: [i64; n],        
    }

    // aとbの添字が同じ要素を比較し、aのほうが大きい添字を見つける
    let mut max_index = -;
    for i in 1..n {
        if a[i] > a[max_index] {
            max_index = i;
            break;
        }
    }

    let mut c = vec![1; n];
    c[max_index] = 

    pb[max_index] 
    for i in 0..n {
        if i != max_index {
            c[i] = b[i];
        }



    let ans: i64 = a.iter().sum();
    println!("{}", ans);
}
