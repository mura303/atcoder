// {{PROBLEM}}
//
// 提出時はこのファイルの中身をそのまま貼る。
// ローカルは -fsanitize=address,undefined 付き。符号付き溢れや配列外参照はここで落ちる
// (ジャッジでは黙って誤答になる)。インタラクティブ問題では sync_with_stdio の行に注意。
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
using ll = long long;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    vector<ll> a(n);
    rep(i, n) cin >> a[i];

    ll ans = accumulate(a.begin(), a.end(), 0LL);
    cout << ans << "\n";
}
