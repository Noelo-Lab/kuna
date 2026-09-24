"""decompile-all over binaries WITH DWARF, where every stack local is type-locked.

Writes <out>/<arm>/<name>.c in castbench's per-function layout so hunkclass.py
can classify every changed line.  hunkclass.py only recognizes a flip of a
`vN` declaration, so re-signing a DWARF-named (locked) local shows up as OTHER.
  python3 corpus-dwarf.py <kuna> <out> <arm> [kuna option args...]
"""
import concurrent.futures as cf
import json
import subprocess
import sys
from pathlib import Path

D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {
    "wc_O0": f"{D}/O0/coreutils/compiled/wc",
    "fmt_O0": f"{D}/O0/coreutils/compiled/fmt",
    "sort_O0": f"{D}/O0/coreutils/compiled/sort",
    "gzip_O0": f"{D}/O0/gzip/compiled/gzip",
    "find_O0": f"{D}/O0/findutils/compiled/find",
    "bzip2_O0": f"{D}/O0/bzip2/compiled/bzip2",
    "diff_O0": f"{D}/O0/diffutils/compiled/diff",
    "grep_O0": f"{D}/O0/grep/compiled/grep",
    "sort_O2": f"{D}/O2/coreutils/compiled/sort",
    "ls_O2ni": f"{D}/O2-noinline/coreutils/compiled/ls",
}
K, OUT, ARM, EXTRA = sys.argv[1], Path(sys.argv[2]), sys.argv[3], sys.argv[4:]


def one(name):
    p = subprocess.run([K, "decompile-all", BINS[name], "--json"] + EXTRA, capture_output=True, text=True)
    if p.returncode != 0:
        return f"{name} {ARM} rc={p.returncode}"
    d = json.loads(p.stdout)
    parts = [f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n" for f in d["functions"] if f.get("code")]
    (OUT / ARM).mkdir(parents=True, exist_ok=True)
    (OUT / ARM / f"{name}.c").write_text("\n".join(parts))
    vars_ = {f["address"]: f.get("variables") for f in d["functions"]}
    (OUT / ARM / f"{name}.vars.json").write_text(json.dumps(vars_, sort_keys=True))
    return f"{name} {ARM} ok {len(parts)} functions"


with cf.ThreadPoolExecutor(5) as ex:
    for r in ex.map(one, list(BINS)):
        print(r, flush=True)
print("CORPUS_DONE")
