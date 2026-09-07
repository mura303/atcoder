// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
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
        _n: usize,
        s: Chars,
    }

    let d: Vec<usize> = s.iter().map(|c| *c as usize - '0' as usize).collect();

    let mut ans = 0usize;

    // 0 から 9 までの二重ループ i j
    for i in 0..10 {
        for j in 0..10 {
            // i を s から探す
            let pi = match d.iter().position(|&x| x == i) {
                Some(p) => p,
                None => continue,
            };

            // 見つかったらその index の次から j を s から探す
            let pj = match d[pi + 1..].iter().position(|&x| x == j) {
                Some(off) => pi + 1 + off,
                None => continue,
            };

            // j が見つかったらその次以降の 0 から 9 のユニーク数を
            // 最終結果カウンターに追加
            let mut seen = [false; 10];
            for &x in &d[pj + 1..] {
                seen[x] = true;
            }
            ans += seen.iter().filter(|&&b| b).count();
        }
    }

    println!("{}", ans);
}
