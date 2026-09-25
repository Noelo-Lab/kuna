#!/usr/bin/env python3
"""Whole-corpus hunk classifier for callpush: OFF_DIR vs ON_DIR (castbench arm layout).

For every changed function:
  1. push lines are dropped from OFF: any statement storing a constant that is a
     real return address of the binary (the address after a `call`, objdump -d),
     whatever its spelling -- a store through the stack pointer, a tracked slot
     `vN = 0x...;`, or the byte-split form (`builtin_strncpy(&X,"..",4)` whose
     bytes spell a return address, plus the `vN = &X;` / `vN[i] = '\\0';` lines
     around it);
  2. the same stores are dropped from ON (tracked return-address slots print in
     both arms), OFF's vN are renamed to ON's by the lines that match once names
     are erased, braces/comments/loop keywords are normalized, and declarations
     are compared as a sorted multiset;
  3. the residual unified diff is split into hunks and each hunk gets a class:
       rename        nothing left after steps 1-2
       reorder       the same lines in another order
       fold          a removed temporary `V = E;` whose E (or a call) reappears inside an added line
       taildup       a removed `goto`/label with the target block's statements added on other paths
       condmerge     an `if` rewritten into a combined `&&`/`||` or `else if` condition
       reanchor      only stack-offset constants moved inside otherwise-equal lines
       header        only the function header comment changed
       decl          only declarations changed
       OTHER         anything else -- read by hand (analysis.md lists what they are)
usage: hunks2.py OFF_DIR ON_DIR OUT.json [N_OTHER_TO_PRINT]
"""
import collections, difflib, json, re, subprocess, sys
from pathlib import Path

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")


def retaddrs(binary):
    out = subprocess.run(["objdump", "-d", "--no-show-raw-insn", str(binary)], capture_output=True, text=True).stdout
    addrs, prev = set(), False
    for L in out.splitlines():
        m = re.match(r"^\s*([0-9a-f]+):\s+(\S+)", L)
        if not m:
            continue
        a = int(m.group(1), 16)
        if prev:
            addrs.add(a)
        prev = m.group(2).startswith("call")
    return addrs


def split(path):
    d, cur = {}, None
    for L in Path(path).read_text(errors="replace").splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", L)
        if m:
            cur = (m.group(1), m.group(2))
            d[cur] = []
        elif cur:
            d[cur].append(L)
    return d


CONST_STORE = re.compile(r"^\s*([^=;]+?)\s*=\s*(0x[0-9a-f]+|\d+);(\s*//.*)?$")


def strip_pushes(lines, ra):
    out, n, skip_bytes = [], 0, 0
    for i, L in enumerate(lines):
        m = CONST_STORE.match(L)
        if m and int(m.group(2), 0) in ra and (int(m.group(2), 0) > 0x1000):
            n += 1
            continue
        m = re.match(r'^\s*builtin_strncpy\([^,]+,"((?:[^"\\]|\\.)*)",4\);', L)
        if m:
            b = m.group(1).encode().decode("unicode_escape").encode("latin-1") + b"\0"
            val = int.from_bytes(b[:4], "little")
            if val in ra:
                n += 1
                skip_bytes = 4
                if out and re.match(r"^\s*v\d+ = &", out[-1]):
                    out.pop()
                continue
        if skip_bytes and re.match(r"^\s*v\d+\[\d\] = '\\0';$", L):
            skip_bytes -= 1
            continue
        skip_bytes = 0
        out.append(L)
    return out, n


def strip_bytepush_long(lines, ra):
    """the 8-store form: vN = &X; vN[0] = 'a'; ... vN[7] = '\\0';"""
    out, i, n = [], 0, 0
    while i < len(lines):
        m = re.match(r"^\s*(v\d+) = &", lines[i])
        if m and i + 8 < len(lines):
            v = m.group(1)
            bs = []
            for j in range(8):
                mm = re.match(r"^\s*%s\[%d\] = '((?:[^'\\]|\\.)*)';$" % (v, j), lines[i + 1 + j])
                if not mm:
                    break
                bs.append(mm.group(1).encode().decode("unicode_escape").encode("latin-1"))
            if len(bs) == 8 and all(len(x) == 1 for x in bs) and int.from_bytes(b"".join(bs), "little") in ra:
                i += 9
                n += 1
                continue
        out.append(lines[i])
        i += 1
    return out, n


def alpha(lines):
    names = {}

    def rn(m):
        k = m.group(0)
        if k not in names:
            names[k] = f"V{len(names)}"
        return names[k]
    decl, stmt = [], []
    for L in lines:
        (decl if re.match(r"^\s{2}[A-Za-z_][\w ]*?[\s*]+v\d+\s*(\[[^\]]*\])?;", L) else stmt).append(L)
    s = [re.sub(r"\bv\d+\b", rn, L) for L in stmt]
    d = sorted(re.sub(r"\s*//.*$", "", re.sub(r"\bv\d+\b", rn, L)) for L in decl)
    return s, d


NAME = re.compile(r"\bv\d+\b")


def mapped(A2, C):
    """rename OFF's vN to ON's vN by the lines that match once names are erased"""
    ea = [NAME.sub("V", l) for l in A2]
    ec = [NAME.sub("V", l) for l in C]
    votes = collections.defaultdict(collections.Counter)
    sm = difflib.SequenceMatcher(a=ea, b=ec, autojunk=False)
    for blk in sm.get_matching_blocks():
        for k in range(blk.size):
            na, nc = NAME.findall(A2[blk.a + k]), NAME.findall(C[blk.b + k])
            if len(na) == len(nc):
                for x, y in zip(na, nc):
                    votes[x][y] += 1
    m = {x: c.most_common(1)[0][0] for x, c in votes.items()}
    ren = [NAME.sub(lambda t: m.get(t.group(0), t.group(0) + "_off"), l) for l in A2]
    return ren


def classify(rem, add):
    R, A = "\n".join(rem), "\n".join(add)
    if not rem and not add:
        return "rename"
    if rem and sorted(l.strip() for l in rem) == sorted(l.strip() for l in add):
        return "reorder"
    if rem and not add and all(re.match(r"^\s*v\d+(_off)? = [^;]+;$", l) or re.match(r"^\s*(goto|label_)", l.strip()) for l in rem):
        return "fold"
    if any(re.search(r"\bgoto label_", l) for l in rem) and not any(re.search(r"\bgoto label_", l) for l in add):
        return "taildup"
    if any(re.search(r"\bif \(\(", l) and ("&&" in l or "||" in l) for l in add) or any("else if" in l for l in add):
        return "condmerge"
    temps = [m.group(1) for l in rem for m in [re.match(r"^\s*(v\d+(?:_off)?) = ([^;]+);$", l)] if m]
    if temps:
        return "fold"
    def z(s):
        return re.sub(r"0x[0-9a-f]+|\b\d+\b", "K", s)
    if [z(l) for l in rem] == [z(l) for l in add]:
        return "reanchor"
    if all(l.strip() in ("{", "}", "") for l in rem + add):
        return "braces"
    return "OTHER"


def main():
    off, on = Path(sys.argv[1]), Path(sys.argv[2])
    tot, per = collections.Counter(), []
    others = []
    for fo in sorted(off.rglob("*.c")):
        rel = fo.relative_to(off)
        opt, proj, b = rel.parts[0], rel.parts[1], rel.stem
        a, c = split(fo), split(on / rel)
        changed = [k for k in a if a[k] != c.get(k)]
        if not changed:
            continue
        ra = retaddrs(RES / opt / proj / "stripped" / b)
        for k in changed:
            A = a[k]
            hdr_a, hdr_c = A[0], c[k][0]
            A2, n1 = strip_pushes(A[1:], ra)
            A2, n2 = strip_bytepush_long(A2, ra)
            C, m1 = strip_pushes(c[k][1:], ra)
            C, m2 = strip_bytepush_long(C, ra)
            A3 = mapped(A2, C)
            isdecl = lambda L: re.match(r"^\s{2}[A-Za-z_][\w ]*?[\s*]+v\d+(_off)?\s*(\[[^\]]*\])?;", L)
            da = sorted(re.sub(r"\s*//.*$", "", L) for L in A3 if isdecl(L))
            dc = sorted(re.sub(r"\s*//.*$", "", L) for L in C if isdecl(L))
            def norm(L):
                L = re.sub(r"\s*//.*$", "", L).rstrip()
                L = re.sub(r"\s*\{$", "", L)
                L = L.replace("while( true )", "LOOP").replace("do", "LOOP") if L.strip() in ("do", "while( true )") else L
                return L
            sa = [norm(L) for L in A3 if not isdecl(L)]
            sc = [norm(L) for L in C if not isdecl(L)]
            sa = [L for L in sa if L.strip() not in ("}", "} LOOP;", "} while( true );", "")]
            sc = [L for L in sc if L.strip() not in ("}", "} LOOP;", "} while( true );", "")]
            classes = collections.Counter()
            if hdr_a != hdr_c:
                classes["header"] += 1
            if da != dc:
                classes["decl"] += 1
            sm = difflib.SequenceMatcher(a=sa, b=sc, autojunk=False)
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag == "equal":
                    continue
                cl = classify(sa[i1:i2], sc[j1:j2])
                classes[cl] += 1
                if cl == "OTHER":
                    others.append((f"{opt}/{b}/{k[0]}", sa[i1:i2], sc[j1:j2]))
            per.append({"fn": f"{opt}/{b}/{k[0]}@{k[1]}", "pushes_off": n1 + n2, "pushes_on": m1 + m2, "lines": [len(A), len(c[k])], "hunks": dict(classes)})
            tot["functions"] += 1
            tot["pushes_off"] += n1 + n2
            tot["pushes_on"] += m1 + m2
            for cl, n in classes.items():
                tot["hunk:" + cl] += n
    json.dump({"total": tot, "functions": per}, open(sys.argv[3], "w"), indent=1)
    print(dict(tot))
    print(f"OTHER hunks: {len(others)}")
    for f, r, a_ in others[: int(sys.argv[4]) if len(sys.argv) > 4 else 30]:
        print("==", f)
        for l in r:
            print("  -", l)
        for l in a_:
            print("  +", l)


if __name__ == "__main__":
    main()
