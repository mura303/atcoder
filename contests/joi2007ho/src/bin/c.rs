// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_c
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

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        points.push((x, y));
    }

    let set: std::collections::HashSet<(i64, i64)> = points.iter().copied().collect();
    let mut ans = 0i64;

    for i in 0..n {
        let (x1, y1) = points[i];
        for j in (i + 1)..n {
            let (x2, y2) = points[j];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let len2 = dx * dx + dy * dy;

            let candidates = [
                ((x1 - dy, y1 + dx), (x2 - dy, y2 + dx)),
                ((x1 + dy, y1 - dx), (x2 + dy, y2 - dx)),
            ];

            for &(p3, p4) in &candidates {
                if set.contains(&p3) && set.contains(&p4) && len2 > ans {
                    ans = len2;
                }
            }
        }
    }

    println!("{}", ans);
}
