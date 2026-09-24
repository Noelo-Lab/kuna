"""Classify every changed line between two castbench arms (same build, castarith off vs on).

Reverse-applies the documented rewrite to the new line and requires it to reproduce the old
line exactly: ((T *)X)[k] -> *(T *)((long)X + k*sizeof T), &((T *)X)[k] -> (T *)((long)X + K),
and a bare base X[k] -> *(T *)((long)X + K) when the old line has that form.
"""
import re, sys, collections
from pathlib import Path

SIZES = {"char": 1, "unsigned char": 1, "bool": 1, "signed char": 1, "uchar": 1,
         "short": 2, "unsigned short": 2, "int": 4, "unsigned int": 4, "float": 4,
         "long": 8, "unsigned long": 8, "double": 8, "long long": 8, "unsigned long long": 8,
         "undefined1": 1, "undefined2": 2, "undefined4": 4, "undefined8": 8, "void": 1,
         "int1": 1, "uint1": 1, "int2": 2, "uint2": 2, "int4": 4, "uint4": 4, "int8": 8, "uint8": 8,
         "xunknown1": 1, "xunknown2": 2, "xunknown4": 4, "xunknown8": 8, "float4": 4, "float8": 8,
         "undefined": 1, "byte": 1, "uchar": 1, "ushort": 2, "uint": 4, "ulong": 8, "code": 1}


PTR = 8


def tsize(t):
    t = t.strip()
    if t.endswith("*"):
        return PTR
    if t in SIZES:
        return SIZES[t]
    return None


def fmt_int(v):
    if v < 0:
        return "-" + fmt_int(-v)
    return str(v) if v < 10 else hex(v)


def match_paren(s, i):
    """s[i] == '('; return index of the matching ')'."""
    d = 0
    for j in range(i, len(s)):
        if s[j] == "(":
            d += 1
        elif s[j] == ")":
            d -= 1
            if d == 0:
                return j
    return -1


def parse_int(tok):
    tok = tok.strip()
    neg = tok.startswith("-")
    if neg:
        tok = tok[1:]
    v = int(tok, 16) if tok.lower().startswith("0x") else int(tok)
    return -v if neg else v


def reverse(line):
    """Undo every ((T *)X)[k] / &((T *)X)[k] in line (innermost first)."""
    changed = True
    n = 0
    while changed:
        changed = False
        # find "((" TYPE " *)" X ")[" IDX "]"
        for m in re.finditer(r"\(\(([A-Za-z_][A-Za-z0-9_ ]*?)( ?\*+)\)", line):
            start = m.start()          # index of the outer '('
            end = match_paren(line, start)
            if end < 0 or end + 1 >= len(line) or line[end + 1] != "[":
                continue
            close = line.find("]", end + 1)
            idx = line[end + 2:close]
            if not re.fullmatch(r"-?(0x[0-9a-f]+|[0-9]+)", idx):
                continue
            name, stars = m.group(1), m.group(2)
            cast = "(" + name + stars + ")"
            sz = PTR if stars.count("*") >= 2 else tsize(name)
            if sz is None:
                continue
            X = line[m.end():end]
            off = parse_int(idx) * sz
            amp = start > 0 and line[start - 1] == "&"
            addend = f" + {fmt_int(off)}"
            if amp:
                repl = f"{cast}((long){X}{addend})"
                line = line[:start - 1] + repl + line[close + 1:]
            else:
                repl = f"*{cast}((long){X}{addend})"
                line = line[:start] + repl + line[close + 1:]
            changed = True
            n += 1
            break
    return line, n


def norm(s):
    s = s.strip()
    return re.sub(r"(?<![A-Za-z0-9_])(0x[0-9a-fA-F]+|\d+)(?![A-Za-z0-9_])", lambda m: str(int(m.group(1), 16 if m.group(1)[:2] in ("0x", "0X") else 10)), s)


def reverse_direct(new, old):
    """Undo a bare base NAME[k] where the old line read *(T *)((long)NAME + K)."""
    out = new
    for m in re.finditer(r"\*\(([A-Za-z_][A-Za-z0-9_ ]*?)( ?\*+)\)\(\(long\)([A-Za-z_][A-Za-z0-9_]*) \+ (-?(?:0x[0-9a-f]+|\d+))\)", old):
            name, stars, var, koff = m.group(1), m.group(2), m.group(3), m.group(4)
            sz = PTR if stars.count("*") >= 2 else tsize(name)
            if not sz:
                continue
            K = parse_int(koff)
            if K % sz:
                continue
            k = K // sz
            for idx in {fmt_int(k), str(k)}:
                pat = f"{var}[{idx}]"
                if pat in out:
                    out = out.replace(pat, m.group(0), 1)
                    break
    return out


def unsign(x):
    return re.sub(r"\(u(int[1248])", r"(\1", x.replace("(unsigned ", "("))


SPELL = {"int1": "char", "uint1": "unsigned char", "int2": "short", "uint2": "unsigned short",
         "int4": "int", "uint4": "unsigned int", "int8": "long", "uint8": "unsigned long"}


def spell(x):
    """The console spells integers int4/uint2/...; a large binary prints both spellings."""
    return re.sub(r"\b(u?int[1248])\b", lambda m: SPELL[m.group(1)], x)


def lhs_only(old, rev):
    """Is the only difference the element type of the store the statement makes,
    `*(T *)X = e;` against `*(U *)X = e;` with T and U one integer type up to sign?"""
    i, j = old.find(" = "), rev.find(" = ")
    if i <= 0 or j <= 0 or norm(old[i:]) != norm(rev[j:]):
        return False
    pat = r"^\s*\*\(([A-Za-z_][A-Za-z0-9_ ]*?) \*\)(.*)$"
    mo, mr = re.match(pat, old[:i]), re.match(pat, rev[:j])
    return bool(mo and mr and norm(mo.group(2)) == norm(mr.group(2))
                and mo.group(1) != mr.group(1) and unsign("(" + mo.group(1)) == unsign("(" + mr.group(1)))


def declared(func_lines):
    """name -> declared type text, from the signature and the local declarations."""
    d = {}
    if func_lines:
        sig = func_lines[0]
        m = re.search(r"\((.*)\)\s*$", sig)
        if m:
            for prm in m.group(1).split(","):
                mm = re.match(r"\s*(.*?)\s*(\**)\s*([A-Za-z_][A-Za-z0-9_]*)\s*$", prm)
                if mm:
                    d[mm.group(3)] = (mm.group(1) + " " + mm.group(2)).strip()
    for L in func_lines[1:]:
        mm = re.match(r"^\s+([A-Za-z_][A-Za-z0-9_ ]*?)\s*(\**)\s*([A-Za-z_][A-Za-z0-9_]*)\s*;", L)
        if mm:
            d[mm.group(3)] = (mm.group(1) + " " + mm.group(2)).strip()
    return d


def pointee_size(decl):
    if not decl or not decl.endswith("*"):
        return None
    inner = decl[:-1].strip()
    if inner.endswith("*"):
        return PTR
    return tsize(inner)


def direct_ok(old, new, decls):
    """Every bare NAME[k] / &NAME[k] new has and old lacks is NAME + k*sizeof(*NAME) in old."""
    news = set(re.findall(r"(?<![A-Za-z0-9_)\]])([A-Za-z_][A-Za-z0-9_]*)\[(-?(?:0x[0-9a-f]+|\d+))\]", new))
    olds = set(re.findall(r"(?<![A-Za-z0-9_)\]])([A-Za-z_][A-Za-z0-9_]*)\[(-?(?:0x[0-9a-f]+|\d+))\]", old))
    extra = news - olds
    if not extra:
        return False
    for name, idx in extra:
        sz = pointee_size(decls.get(name))
        if not sz:
            return False
        K = parse_int(idx) * sz
        if not re.search(r"\(long\)" + re.escape(name) + r" \+ " + re.escape(fmt_int(K)) + r"\b", norm_hex(old)):
            return False
    return True


def norm_hex(s):
    return re.sub(r"(?<![A-Za-z0-9_])(0x[0-9a-fA-F]+|\d+)(?![A-Za-z0-9_])", lambda m: fmt_int(int(m.group(1), 0)), s)


def strip_parens(s):
    return re.sub(r"[() ]", "", s)


DECLS = {}


def classify(old, new):
    old = old.replace("(int8)", "(long)")
    new = new.replace("(int8)", "(long)")
    if PTR == 4:
        old = old.replace("(int4)", "(long)").replace("(int)", "(long)")
        new = new.replace("(int4)", "(long)").replace("(int)", "(long)")
    decl = re.match(r"^\s*[A-Za-z_][A-Za-z0-9_ ]*[ *]+[A-Za-z_][A-Za-z0-9_]*\s*(\(|;|\[)", old)
    if decl and not re.search(r"[=(]\s*\(|\[", old.split("(")[0] if "(" in old else old):
        pass
    old, new = spell(old), spell(new)
    r, n = reverse(new)
    if n and norm(r) == norm(old):
        return "exact", n
    rd = reverse_direct(r, old)
    if rd != r and norm(rd) == norm(old):
        return "direct", n + 1
    # void target: old spelled the sum (void *)(...) or with no outer cast
    if n and unsign(norm(r)) == unsign(norm(old)):
        return ("sign-of-store-element" if lhs_only(old, r) else "sign-of-element-elsewhere"), n
    if n and unsign(norm(rd)) == unsign(norm(old)):
        return "direct-sign", n + 1
    if n and strip_parens(norm(r)) == strip_parens(norm(old)):
        return "parens-only", n
    m_new = re.match(r"^(\s*[A-Za-z_][A-Za-z0-9_]* = )\([^()]*\)(\(\(long\).*)$", r)
    m_old = re.match(r"^(\s*[A-Za-z_][A-Za-z0-9_]* = )\([^()]*\)(\(\(long\).*)$", old)
    if n and m_new and m_old and m_new.group(1) == m_old.group(1) and norm(m_new.group(2)) == norm(m_old.group(2)):
        return "assignment-outer-cast", n
    if direct_ok(old, r, DECLS):
        return "direct-declared", n + 1
    r2 = r.replace("(char *)((long)", "(void *)((long)")
    if n and norm(r2) == norm(old):
        return "void-bytes", n
    r3 = re.sub(r"\(char \*\)\(\(long\)([^()]*(?:\([^()]*\))*[^()]*) \+ ([^)]*)\)", r"(long)\1 + \2", r)
    if n and norm(r3) == norm(old):
        return "void-bytes-nocast", n
    return None, n


def funcs(path):
    out = {}
    cur = None
    for L in Path(path).read_text(errors="replace").splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", L)
        if m:
            cur = m.group(2)
            out[cur] = []
            continue
        if cur is not None:
            out[cur].append(L)
    return out


def main(a, b):
    stats = collections.Counter()
    bad = []
    nfun = 0
    for fa in sorted(Path(a).rglob("*.c")):
        fb = Path(b) / fa.relative_to(a)
        A, B = funcs(fa), funcs(fb)
        global PTR
        text = fa.read_text(errors="replace")
        wide = text.count("((long)") + text.count("((int8)")
        narrow = text.count("((int)") + text.count("((int4)")
        PTR = 4 if narrow > wide else 8
        for addr in A:
            if A[addr] == B.get(addr):
                continue
            nfun += 1
            la, lb = A[addr], B.get(addr, [])
            if len(la) != len(lb):
                stats["LINECOUNT"] += 1
                bad.append((str(fa), addr, "line count differs", "", ""))
                continue
            global DECLS
            DECLS = declared(lb)
            for x, y in zip(la, lb):
                if x == y:
                    continue
                cls, n = classify(x, y)
                if cls:
                    stats[cls] += 1
                    stats["sites"] += n
                    if cls == "sign-of-element-elsewhere":
                        bad.append((str(fa), addr, x.strip(), y.strip(), "[" + cls + "] " + reverse(spell(y))[0].strip()))
                else:
                    stats["UNCLASSIFIED"] += 1
                    bad.append((str(fa), addr, x.strip(), y.strip(), reverse(y)[0].strip()))
    print(f"changed functions {nfun}")
    for k, v in stats.most_common():
        print(f"  {k:22} {v}")
    for f, addr, x, y, r in bad[:int(sys.argv[3]) if len(sys.argv) > 3 else 30]:
        print(f"--- {f} {addr}\n  old: {x}\n  new: {y}\n  rev: {r}")


main(sys.argv[1], sys.argv[2])
