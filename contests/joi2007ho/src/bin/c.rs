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
        a: [[i64; n]; n],
    }

    for i in 0..n {
        if a[i][i] != 0 {
            println!("-1");
            return;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if a[i][j] != a[j][i] {
                println!("-1");
                return;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if a[i][j] > a[i][k] + a[k][j] {
                    println!("-1");
                    return;
                }
            }
        }
    }

    let mut ans = 0i64;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut needed = true;
            for k in 0..n {
                if k != i && k != j && a[i][j] == a[i][k] + a[k][j] {
                    needed = false;
                    break;
                }
            }
            if needed {
                ans += a[i][j];
            }
        }
    }

    println!("{}", ans);
}
