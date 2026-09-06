# 分野別 初中級者が解くべき過去問精選 100 問

出典: https://qiita.com/e869120/items/eb50fdaece12be418faa（E869120 さん）§2-3

問題の選定と分野分けは上の記事による。記事本文は転載していない。
AOJ の問題を AtCoder の同等問題に読み替えた部分はこのリポジトリの判断。

**進捗 7 / 92**（AtCoder 分のみ。目安は 100 問中 70 問、
全部解けたら青相当）。`bin/pick --refresh` で作り直される。

毎日の 1 問はこのリストから**分野順に**出る。手で編集しても次の
`--refresh` で上書きされるので、記録は `log.md` に書く。

AtCoder 92 問 / AOJ 8 問。うち 21 問は、
AOJ の基本問題を AtCoder の同等問題に読み替えてある（`bin/new`・`bin/t`・
スマホからの実装をそのまま使うため）。元の問題名を併記した。

AOJ のまま残した問題は代替が見つからなかったもの。ブラウザで解いて
`log.md` に記録する。`bin/pick` の出題からは外れる。

## 全探索：全列挙

- [x] **1** [ABC051 B — Sum of Three Integers](https://atcoder.jp/contests/abc051/tasks/abc051_b) `784` ← ITP1_7_B - How Many Ways? の読み替え
- [x] **2** [ABC106 B — 105](https://atcoder.jp/contests/abc106/tasks/abc106_b) `-75`
- [x] **3** [ABC122 B — ATCoder](https://atcoder.jp/contests/abc122/tasks/abc122_b) `-122`
- [x] **4** [PAKENCAMP-2019-DAY3 C — カラオケ](https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c) `—`

## 全探索：工夫して通り数を減らす全列挙

- [x] **5** [ABC095 C — Half and Half](https://atcoder.jp/contests/abc095/tasks/arc096_a) `273`
- [ ] **6** [SUMITRUST2019 D — Lucky PIN](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d) `838`
- [ ] **7** [JOI2007HO C — 最古の遺跡](https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_c) `—`
- [ ] **8** [S8PC-6 B — AtCoder Market](https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b) `—`
- [ ] **9** [JOI2008YO D — 星座探し](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d) `—`

## 全探索：ビット全探索

- [ ] **10** [ABC045 C — Many Formulas](https://atcoder.jp/contests/abc045/tasks/arc061_a) `1089` ← ALDS_5_A - 総当たり の読み替え
- [ ] **11** [ABC128 C — Switches](https://atcoder.jp/contests/abc128/tasks/abc128_c) `805`
- [ ] **12** [ABC002 D — 派閥](https://atcoder.jp/contests/abc002/tasks/abc002_4) `1418`
- [ ] **13** [JOI2008YO E — おせんべい](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e) `—`
- [ ] **14** [S8PC-4 B — Buildings are Colorful!](https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_b) `—`

## 全探索：順列全探索

- [ ] **15** [ABC145 C — Average Length](https://atcoder.jp/contests/abc145/tasks/abc145_c) `335`
- [ ] **16** [ABC150 C — Count Order](https://atcoder.jp/contests/abc150/tasks/abc150_c) `422`
- [ ] **17** [ABC054 C — One-stroke Path](https://atcoder.jp/contests/abc054/tasks/abc054_c) `1244` ← ALDS_13_A - 8 クイーン問題 の読み替え

## 二分探索

- [ ] **18** [ABC143 D — Triangles](https://atcoder.jp/contests/abc143/tasks/abc143_d) `686` ← ALDS_4_B - 二分探索 の読み替え
- [ ] **19** [JOI2009HO B — ピザ](https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b) `—`
- [ ] **20** [ABC077 C — Snuke Festival](https://atcoder.jp/contests/abc077/tasks/arc084_a) `1096`
- [ ] **21** [ABC023 D — 射撃王](https://atcoder.jp/contests/abc023/tasks/abc023_d) `1843`
- [ ] **22** [ARC054 B — ムーアの法則](https://atcoder.jp/contests/arc054/tasks/arc054_b) `1269`
- [ ] **23** [JOI2008HO C — ダーツ](https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_c) `—`

## 深さ優先探索

- [ ] **24** [ATC001 A — 深さ優先探索](https://atcoder.jp/contests/atc001/tasks/dfs_a) `—` ← ALDS_11_B - 深さ優先探索 の読み替え
- [ ] **25** [ABC284 C — Count Connected Components](https://atcoder.jp/contests/abc284/tasks/abc284_c) `108` ← AOJ 1160 - 島はいくつある？ の読み替え
- [ ] **26** [ABC138 D — Ki](https://atcoder.jp/contests/abc138/tasks/abc138_d) `920`
- [ ] **27** [JOI2009YO D — 薄氷渡り](https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_d) `—`

## 幅優先探索

- [ ] **28** [ABC168 D — .. (Double Dots)](https://atcoder.jp/contests/abc168/tasks/abc168_d) `804` ← ALDS_11_C - 幅優先探索 の読み替え
- [ ] **29** [ABC007 3 — 幅優先探索](https://atcoder.jp/contests/abc007/tasks/abc007_3) `1024`
- [ ] **30** [JOI2011YO E — チーズ (Cheese)](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_e) `—`
- [ ] **31** [JOI2012YO E — イルミネーション (Illumination)](https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_e) `—`
- [ ] **32** AOJ 1166 - 迷図と命ず — AOJ（ブラウザで解く）
- [ ] **33** [ABC088 D — Grid Repainting](https://atcoder.jp/contests/abc088/tasks/abc088_d) `999`

## 動的計画法：ナップザック DP

- [x] **34** [DP A — Frog 1](https://atcoder.jp/contests/dp/tasks/dp_a) `—` ← ALDS_10_A - フィボナッチ数 の読み替え
- [x] **35** [DP D — Knapsack 1](https://atcoder.jp/contests/dp/tasks/dp_d) `—` ← DPL_1_B - 0,1ナップザック問題 の読み替え
- [ ] **36** [ABC153 E — Crested Ibis vs Monster](https://atcoder.jp/contests/abc153/tasks/abc153_e) `1015` ← DPL_1_C - ナップザック問題 の読み替え
- [ ] **37** [ABC099 C — Strange Bank](https://atcoder.jp/contests/abc099/tasks/abc099_c) `1101` ← DPL_1_A - コイン問題 の読み替え
- [ ] **38** [DP F — LCS](https://atcoder.jp/contests/dp/tasks/dp_f) `—` ← ALDS_10_C - 最長共通部分列 の読み替え
- [ ] **39** [JOI2011YO D — 1 年生 (A First Grader)](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d) `—`
- [ ] **40** [JOI2012YO D — パスタ (Pasta)](https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_d) `—`
- [ ] **41** [JOI2013YO D — 暑い日々 (Hot days)](https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_d) `—`
- [ ] **42** [JOI2015YO D — シルクロード (Silk Road)](https://atcoder.jp/contests/joi2015yo/tasks/joi2015yo_d) `—`
- [ ] **43** [PAKENCAMP-2019-DAY3 D — パ研軍旗](https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_d) `—`
- [ ] **44** AOJ 1167 - ポロック予想 — AOJ（ブラウザで解く）
- [ ] **45** AOJ 2199 - 差分パルス符号変調 — AOJ（ブラウザで解く）

## 動的計画法：区間 DP

- [ ] **46** [DP N — Slimes](https://atcoder.jp/contests/dp/tasks/dp_n) `—` ← ALDS_10_B - 連鎖行列積 の読み替え
- [ ] **47** [JOI2015HO B — ケーキの切り分け２ (Cake 2)](https://atcoder.jp/contests/joi2015ho/tasks/joi2015ho_b) `—`
- [ ] **48** AOJ 1611 ダルマ落とし — AOJ（ブラウザで解く）

## 動的計画法：bit DP

- [ ] **49** [ABC180 E — Traveling Salesman among Aerial Cities](https://atcoder.jp/contests/abc180/tasks/abc180_e) `1256` ← DPL_2_A - 巡回セールスマン問題 の読み替え
- [ ] **50** [S8PC-1 G — Revenge of Traveling Salesman Problem](https://atcoder.jp/contests/s8pc-1/tasks/s8pc_1_g) `—`
- [ ] **51** [JOI2014YO D — 部活のスケジュール表 (Schedule)](https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_d) `—`
- [ ] **52** [JOI2017YO D — ぬいぐるみの整理 (Plush Toys) ](https://atcoder.jp/contests/joi2017yo/tasks/joi2017yo_d) `—`

## 動的計画法：その他

- [ ] **53** DPL_1_D - 最長増加部分列 — AOJ（ブラウザで解く）
- [ ] **54** [ABC006 D — トランプ挿入ソート](https://atcoder.jp/contests/abc006/tasks/abc006_4) `1696`
- [ ] **55** [ABC134 E — Sequence Decomposing](https://atcoder.jp/contests/abc134/tasks/abc134_e) `1320`

## 最短経路問題：ダイクストラ法

- [ ] **56** [ABC340 D — Super Takahashi Bros.](https://atcoder.jp/contests/abc340/tasks/abc340_d) `784` ← GRL_1_A - 単一始点最短経路 の読み替え
- [ ] **57** [JOI2008YO F — 船旅](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_f) `—`
- [ ] **58** [JOI2016YO E — ゾンビ島 (Zombie Island)](https://atcoder.jp/contests/joi2016yo/tasks/joi2016yo_e) `—`
- [ ] **59** [JOI2014YO E — タクシー (Taxis)](https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_e) `—`

## 最短経路問題：ワーシャルフロイド法

- [ ] **60** [ABC073 D — joisino's travel](https://atcoder.jp/contests/abc073/tasks/abc073_d) `1345` ← GRL_1_C - 全点対間最短経路 の読み替え
- [ ] **61** [ABC012 D — バスと避けられない運命](https://atcoder.jp/contests/abc012/tasks/abc012_4) `1166`
- [ ] **62** [ABC079 D — Wall](https://atcoder.jp/contests/abc079/tasks/abc079_d) `949`
- [ ] **63** [ABC074 D — Restoring Road Network](https://atcoder.jp/contests/abc074/tasks/arc083_b) `1563`

## 最小全域木問題

- [ ] **64** [ABC218 E — Destruction](https://atcoder.jp/contests/abc218/tasks/abc218_e) `1004` ← GRL_2_A - 最小全域木 の読み替え
- [ ] **65** [JOISC2010 finals — 本選会場 (Finals)](https://atcoder.jp/contests/joisc2010/tasks/joisc2010_finals) `—`
- [ ] **66** AOJ 1127 - Building a Space Station — AOJ（ブラウザで解く）
- [ ] **67** [ABC065 D — Built?](https://atcoder.jp/contests/abc065/tasks/arc076_b) `1615`

## 高速な素数判定法

- [ ] **68** [ABC142 D — Disjoint Set of Common Divisors](https://atcoder.jp/contests/abc142/tasks/abc142_d) `827` ← NTL_1_A - 素因数分解 の読み替え
- [ ] **69** [ABC084 D — 2017-like Number](https://atcoder.jp/contests/abc084/tasks/abc084_d) `980`

## 高速なべき乗計算

- [ ] **70** [ABC156 D — Bouquet](https://atcoder.jp/contests/abc156/tasks/abc156_d) `1014` ← NTL_1_B - べき乗 の読み替え
- [ ] **71** [S8PC-1 E — 散歩 (E869120 and Path Length)](https://atcoder.jp/contests/s8pc-1/tasks/s8pc_1_e) `—`

## 逆元を使う問題

- [ ] **72** [ABC034 C — 経路](https://atcoder.jp/contests/abc034/tasks/abc034_c) `1440`
- [ ] **73** [ABC145 D — Knight](https://atcoder.jp/contests/abc145/tasks/abc145_d) `1009`
- [ ] **74** [ABC021 D — 多重ループ](https://atcoder.jp/contests/abc021/tasks/abc021_d) `1704`
- [ ] **75** [ABC149 F — Surrounded Nodes](https://atcoder.jp/contests/abc149/tasks/abc149_f) `2208`

## 累積和

- [ ] **76** [NIKKEI2019-FINAL A — Abundant Resources](https://atcoder.jp/contests/nikkei2019-final/tasks/nikkei2019_final_a) `—`
- [ ] **77** [JOI2010HO A — 旅人](https://atcoder.jp/contests/joi2010ho/tasks/joi2010ho_a) `—`
- [ ] **78** [JOI2011HO A — 惑星探査 (Planetary Exploration)](https://atcoder.jp/contests/joi2011ho/tasks/joi2011ho1) `—`
- [ ] **79** [ABC106 D — AtCoder Express 2](https://atcoder.jp/contests/abc106/tasks/abc106_d) `1319`
- [ ] **80** [GIGACODE-2019 D — 家の建設](https://atcoder.jp/contests/gigacode-2019/tasks/gigacode_2019_d) `—`
- [ ] **81** [ABC014 C — AtColor](https://atcoder.jp/contests/abc014/tasks/abc014_3) `1276`
- [ ] **82** [ABC183 D — Water Heater](https://atcoder.jp/contests/abc183/tasks/abc183_d) `662` ← AOJ 2013 - 大崎 の読み替え
- [ ] **83** [JOI2015HO A — 鉄道旅行 (Railroad Trip)](https://atcoder.jp/contests/joi2015ho/tasks/joi2015ho_a) `—`
- [ ] **84** [JOI2012HO D — 釘 (Nails)](https://atcoder.jp/contests/joi2012ho/tasks/joi2012ho4) `—`

## Union-Find

- [ ] **85** [ATC001 B — Union Find](https://atcoder.jp/contests/atc001/tasks/unionfind_a) `—` ← DSL_1_A - 互いに素な集合 の読み替え
- [ ] **86** [ABC075 C — Bridge](https://atcoder.jp/contests/abc075/tasks/abc075_c?lang=ja) `1067`
- [ ] **87** [ABC120 D — Decayed Bridges](https://atcoder.jp/contests/abc120/tasks/abc120_d) `1355`

## その他のテクニック

- [ ] **88** [JOI2008HO A — 碁石ならべ](https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_a) `—`
- [ ] **89** [JOI2013HO 1 — 電飾 (Illumination)](https://atcoder.jp/contests/joi2013ho/tasks/joi2013ho1) `—`
- [ ] **90** [S8PC-5 B — Emblem](https://atcoder.jp/contests/s8pc-5/tasks/s8pc_5_b) `—`
- [ ] **91** [ABC144 D — Water Bottle](https://atcoder.jp/contests/abc144/tasks/abc144_d) `714`

## 実装問題

- [ ] **92** AOJ 1193 - 連鎖消滅パズル — AOJ（ブラウザで解く）
- [ ] **93** [S8PC-3 B — Falling Stone Game](https://atcoder.jp/contests/s8pc-3/tasks/s8pc_3_b) `—`
- [ ] **94** AOJ 1149 - ケーキカット — AOJ（ブラウザで解く）

## 数学的な問題

- [ ] **95** [ABC149 B — Greedy Takahashi](https://atcoder.jp/contests/abc149/tasks/abc149_b) `—`
- [ ] **96** [ABC139 D — ModSum](https://atcoder.jp/contests/abc139/tasks/abc139_d) `354`
- [ ] **97** [ABC150 D — Semi Common Multiple](https://atcoder.jp/contests/abc150/tasks/abc150_d) `1534`
- [ ] **98** [SUMITRUST2019 E — Colorful Hats 2](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_e) `1176`
- [ ] **99** [DDCC2020-QUAL D — Digit Sum Replace](https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_d) `1617`
- [ ] **100** [TENKA1-2018-BEGINNER D — Crossing](https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_d) `1351`

---

難易度は AtCoder Problems の推定値。`—` は提出数が足りず推定が出ていないもの
（JOI 予選や S8PC に多い）。難易度が付いていない＝易しい、ではない。
