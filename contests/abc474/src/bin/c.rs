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
        q: usize,
        p: [usize; n],
        a: [usize; q],
    }

    let mut moved = HashSet::new();
    let mut tail = Vec::new();

    for &x in a.iter().rev() {
        if moved.insert(x) {
            tail.push(x);
        }
    }

    let mut answer = Vec::with_capacity(n);

    for &x in &p {
        if !moved.contains(&x) {
            answer.push(x);
        }
    }

    tail.reverse();
    answer.extend(tail);

    for (i, x) in answer.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{x}");
    }
    println!();
}
