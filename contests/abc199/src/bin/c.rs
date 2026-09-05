// https://atcoder.jp/contests/abc199/tasks/abc199_c
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
        mut s: Chars,
        q: usize,
    }

    let mut n_count = 0;

    for _ in 0..q {
        input! {
            t: usize,
            mut a: usize,
            mut b: usize,
        }

        if t == 1 {
            a -= 1;
            b -= 1;
            if n_count % 2 == 1 {
                a = (a + n) % (n * 2);
                b = (b + n) % (n * 2);
            }
            s.swap(a, b);
        } else {
            n_count += 1;
        }
    }

    if n_count % 2 == 1 {
        s.rotate_right(n);
    }

    println!("{}", s.iter().collect::<String>());
}
