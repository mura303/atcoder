// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d
//
// 提出時はこのファイルの中身をそのまま貼る。
// #[fastout] は大量出力を高速化するが、インタラクティブ問題では外すこと
// (出力がバッファに溜まって相手に届かなくなるため)。
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};



// seizaの全要素がstar_mapの中に含まれるかどうかを判定する
fn is_contained(seiza: &[(i64, i64)], star_map: &HashSet<(i64, i64)>) -> bool {
    for &(x, y) in seiza {
        if !star_map.contains(&(x, y)) {
            return false;
        }
    }
    true
}


#[fastout]
fn main() {
    input! {
        m: usize,
        seiza: [(i64, i64); m],
        n: usize,
        star: [(i64, i64); n],
    }

    let star_map: HashSet<(i64, i64)> = star.iter().copied().collect();

    let mut ans_x = 0;
    let mut ans_y = 0;

    for i in 0..n {
        let (sx, sy) = seiza[0];
        let (tx, ty) = star[i];
        let dx = tx - sx;
        let dy = ty - sy;

        let moved_seiza: Vec<(i64, i64)> = seiza.iter().map(|(x, y)| (x + dx, y + dy)).collect();
        if is_contained(&moved_seiza, &star_map) {
            ans_x = dx;
            ans_y = dy;
            break;
        }
    }

    println!("{} {}", ans_x, ans_y);
}
