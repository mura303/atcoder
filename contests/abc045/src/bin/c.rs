// https://atcoder.jp/contests/abc045/tasks/arc061_a
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
        s: Chars,
    }

    let n = s.len();
    let mut total_sum = 0;

    for bit in 0..(1 << n-1) {
        let mut v: Vec<String> = Vec::new();
        let mut last_index = 0;
        for i in 0..n-1 {
            if bit & (1 << i) > 0 {
                v.push(s[last_index..=i].iter().collect());
                last_index = i+1;
            }
        }
        
        v.push(s[last_index..].iter().collect());


        // vの中身を確認する
//        println!("{:?}", v);

        // vを数値として解釈して合計する
        let sum: i64 = v.iter().map(|x| x.parse::<i64>().unwrap()).sum();
        total_sum += sum;

    }

    println!("{}", total_sum);
}
