# AtCoder 練習環境 (Rust)

## セットアップ済みのもの

| | |
|---|---|
| ツールチェーン | `1.89.0` — AtCoder の 2025/10 新ジャッジと同じ rustc を `rust-toolchain.toml` で固定 |
| クレート | ジャッジに存在するものだけを `Cargo.toml` の `[workspace.dependencies]` にバージョン固定で列挙 |
| overflow-checks | release でも **有効**。ジャッジでは無効なので、ここで panic したら本番では黙って誤答になるバグ |

## 使い方

```sh
bin/new abc428          # コンテスト一式を作り、サンプルを取得
bin/new abc428 d e      # 一部の問題だけ
bin/t   abc428 d        # サンプル全件でローカル判定
bin/t   abc428 d 2      # 2 番のケースだけ
bin/t   abc428 d --tl 3 # 実行時間制限を変える (既定 2 秒)
```

生成物:

```
contests/abc428/
├── Cargo.toml
├── src/bin/{a,b,c,...}.rs   # ここを編集して、中身をそのまま提出する
└── tests/{a,b,c,...}/N.in   # サンプル入力
                    /N.out   # サンプル出力
```

開催中のコンテストは問題ページが非公開でサンプルを取れない。
その場合は `bin/new abc999 --no-fetch a b c d` で枠だけ作り、
`tests/a/1.in` `tests/a/1.out` に手で貼る。

## Rust 固有のハマりどころ

- **`use ac_library::...`** — クレート名は `ac-library-rs` だが、`[lib] name` は `ac_library`。
  0.1.1 時代の `ac_library_rs` は 0.2.0 で通らない。
  例: `use ac_library::{Dsu, ModInt998244353 as Mint, Segtree};`
- **`usize` の引き算** — `a - b` で `b > a` だと panic (release でも本環境は overflow-checks 有効)。
  比較は `if a > b`、差は `a.saturating_sub(b)` か `as i64` に上げてから。
- **オーバーフロー** — `N ≤ 10^5` でも和が `10^18` に届く問題は多い。既定で `i64` を使う。
- **`#[fastout]`** — 大量出力の TLE 対策。ただし**インタラクティブ問題では外す**
  (出力がバッファに溜まってジャッジに届かない)。
- **`HashMap` が遅い** — 標準の hasher は攻撃耐性のぶん重い。`rustc_hash::FxHashMap` に置き換える。
- **深い再帰** — 既定スタックで落ちることがある。DFS は明示スタックにするか、
  `std::thread::Builder::new().stack_size(64 << 20)` で回す。

## 精進の進め方

`ROADMAP.md` を参照。
