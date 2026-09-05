// https://atcoder.jp/contests/abc071/tasks/arc081_a
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
        mut a: [i64; n],
    }

    // aを大きい順に並び替える
    a.sort_by(|a, b| b.cmp(a));

    // 整数の配列（動的サイズ）を宣言
    let mut b = Vec::new();
    
    let mut i = 0;
    while i < a.len() - 1 {
        if a[i] == a[i+1] {
            b.push(a[i]);
            i += 1;
        }

        // bの要素数が2つ以上になったらループを抜ける
        if b.len() >= 2 {
            break;
        }
        i +=1 ;
    }

    if b.len() < 2 {
        println!("0");
    }else{
        println!("{}", b[0] * b[1]);
    }

}
