"""bin/t と bin/r の共通部分: 言語の選択とビルド。

解答は contests/<contest>/<problem>.cpp か .rb に置く。
C++ は build/<contest>_<problem> にビルドし、Ruby はそのまま実行する。
"""
import os
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXT = {"cpp": ".cpp", "rb": ".rb"}


def pick_lang(contest, problem, lang):
    """使う言語を決める。--lang 指定が無く両方あるときは止める。"""
    base = os.path.join(ROOT, "contests", contest, problem)
    found = [l for l, e in EXT.items() if os.path.exists(base + e)]
    if lang:
        if lang not in EXT:
            sys.exit(f"--lang は cpp か rb: {lang}")
        if lang not in found:
            sys.exit(f"{base}{EXT[lang]} が無い。")
        return lang, base + EXT[lang]
    if not found:
        sys.exit(f"{base}.cpp / .rb が無い。bin/new {contest} {problem} で作る。")
    if len(found) > 1:
        sys.exit(f"{contest} {problem} は cpp と rb の両方がある。--lang cpp か --lang rb で選ぶ。")
    return found[0], base + EXT[found[0]]


def cxx():
    # brew の gcc があればジャッジに近いのでそちらを使う。無ければ Apple clang。
    for name in ("g++-15", "g++-14", "g++-13", "g++"):
        path = shutil.which(name)
        if path:
            return path
    sys.exit("C++ コンパイラが無い。")


def build(contest, problem, lang, src):
    """実行コマンド (argv のリスト) を返す。ビルドに失敗したら終了する。"""
    if lang == "rb":
        return ["ruby", src]
    out_dir = os.path.join(ROOT, "build")
    os.makedirs(out_dir, exist_ok=True)
    out = os.path.join(out_dir, f"{contest}_{problem}")
    cmd = [
        cxx(), "-std=c++23", "-O2", "-g", "-Wall", "-Wextra",
        "-fsanitize=address,undefined", "-fno-sanitize-recover=undefined",
        "-I", os.path.join(ROOT, "lib", "compat"),
        "-I", os.path.join(ROOT, "lib", "ac-library"),
        src, "-o", out,
    ]
    r = subprocess.run(cmd, cwd=ROOT)
    if r.returncode != 0:
        sys.exit(r.returncode)
    return [out]
