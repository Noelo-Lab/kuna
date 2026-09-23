#!/usr/bin/env python3
"""Classify every function `structheadless closed` changes between two castbench arms.

    python3 hunks.py <off-arm-dir> <on-arm-dir> [--list]

Each arm is a `castbench.py run` directory (`<opt>/<project>/<binary>.c`, one
`// Function:` block per function, `kuna decompile-all` output). Buckets, in
order: a skeleton delta (a control-flow keyword or a call token moved), a
declaration-count delta, `record name only` (identical once every `struct_N`
number and every `vN`/`aN` is spelled the same way), `field accesses only`
(every differing line differs only where a field reference or a record
spelling appears on one side), and `read` for the rest. Everything outside the
last two buckets is printed for reading.
"""
import collections, difflib, pathlib, re, sys

KEYWORDS = "if|else|while|do|for|goto|return|switch|case|default|break|continue"
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}
RENUM = re.compile(r"\b[av]\d+\b")
SNUM = re.compile(r"\bstruct_\d+\b")


def split(path):
    out, cur, buf = {}, None, []
    p = pathlib.Path(path)
    if not p.exists():
        return out
    for line in p.read_text(errors="replace").splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur:
                out[cur] = buf
            cur, buf = m.group(2), [line]
        else:
            buf.append(line)
    if cur:
        out[cur] = buf
    return out


def parts(lines):
    try:
        start = next(i for i, l in enumerate(lines) if l.rstrip() == "{") + 1
    except StopIteration:
        return lines[:2], [], lines
    head, decls, i = lines[:start], [], start
    while i < len(lines) and lines[i].strip():
        decls.append(lines[i].strip())
        i += 1
    return head, decls, lines[i:]


def skeleton(lines):
    toks = []
    for line in lines:
        for m in re.finditer(rf"\b({KEYWORDS})\b|\b([A-Za-z_]\w*)\s*\(", line.split("//")[0]):
            if m.group(1):
                toks.append(m.group(1))
            elif m.group(2) not in NOTCALL:
                toks.append(m.group(2) + "()")
    return toks


def norm(ls):
    return [SNUM.sub("S", RENUM.sub("V", l)) for l in ls]


def classify(a, b):
    ha, da, ba = parts(a)
    hb, db, bb = parts(b)
    if skeleton(ba) != skeleton(bb):
        return "skeleton delta", []
    if len(da) != len(db):
        return "declaration-count delta", []
    if norm(ha + da + ba) == norm(hb + db + bb):
        return "record name only", []
    diffs = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, norm(ha + da + ba), norm(hb + db + bb),
                                                        autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        rem, add = norm(ha + da + ba)[i1:i2], norm(hb + db + bb)[j1:j2]
        for t in range(max(len(rem), len(add))):
            r = (rem[t] if t < len(rem) else "").strip()
            d = (add[t] if t < len(add) else "").strip()
            if r != d and not any(x in s for s in (r, d) for x in ("->field_0x", "S *", "S*", "(S)")):
                diffs.append((r, d))
    return ("field accesses only", []) if not diffs else ("read", diffs)


def main():
    off, on = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    rows, total, reads = [], collections.Counter(), []
    for f in sorted(off.rglob("*.c")):
        rel = f.relative_to(off)
        a, b = split(f), split(on / rel)
        per = collections.Counter()
        for k in sorted(set(a) | set(b)):
            if a.get(k) == b.get(k):
                continue
            c, diffs = classify(a.get(k, []), b.get(k, []))
            per[c] += 1
            if c not in ("record name only", "field accesses only"):
                reads.append((str(rel), k, c, diffs[:3]))
        rows.append((str(rel), len(set(a) | set(b)), sum(per.values()), per))
        total.update(per)
    cols = ["record name only", "field accesses only", "skeleton delta", "declaration-count delta", "read"]
    print("| binary | functions | changed | " + " | ".join(cols) + " |")
    print("|---|---:|---:|" + "---:|" * len(cols))
    for name, n, ch, per in rows:
        print(f"| {name} | {n} | {ch} | " + " | ".join(str(per[c]) for c in cols) + " |")
    print(f"| **total** | {sum(r[1] for r in rows)} | {sum(r[2] for r in rows)} | "
          + " | ".join(str(total[c]) for c in cols) + " |")
    if "--list" in sys.argv:
        for rel, k, c, diffs in reads:
            print(f"READ {rel} {k} [{c}]")
            for r, d in diffs:
                print(f"     - {r}\n     + {d}")


if __name__ == "__main__":
    main()
