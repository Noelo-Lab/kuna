"""Did anything move relative to a call or a memory access?

`callorder.py` compares call-to-call order only, so a call reordered against a
memory read is invisible to it.  This is the stronger check: per function, the
whole ordered stream of observable events -- every call, plus one marker per
memory touch (a dereference, an index, a `->`, or a `dat_`/`unk_` global) -- has
to be identical with the option off and on.  Folding a call into its use must
not change that stream: the call keeps its place in it, and so does every load
and store around it.

Events inside one statement are ordered innermost-first then left to right,
which is how C evaluates them, except that a plain assignment's target is
appended last because its store happens after the right-hand side.  That is what
makes a statement merged into another compare equal to the two statements it
replaced, so only a real reordering shows up.

Two levels are reported.  A changed event *set* means a call or a memory touch
was gained or lost -- always a defect.  A changed event *order* with the same set
is a screen, not a verdict: this is a textual metric with no C parser, so two
statements merged into one legitimately re-sort their own events, and each hit
has to be read by hand (`foldmove.py` measures statement-level movement, which is
the thing the option can actually do).

    python3 evalorder.py <sweep-dir> <name>...
      # reads <sweep-dir>/<name>.off.c and <sweep-dir>/<name>.on.c
"""
import re, sys, collections

FUNC = re.compile(r"^// Function: (\S+) @ (\S+)$")
CALL = re.compile(r"\b([A-Za-z_]\w*)\s*\(")
KW = {"if", "while", "for", "switch", "return", "do", "else", "sizeof", "break",
      "continue", "goto"}
GLOBAL = re.compile(r"\b(dat|unk|byte|word|dword|qword)_[0-9a-fA-F]+\b")
DEREF = re.compile(r"(?<![\w\)\]])\*\s*[\w\(]")
INDEX = re.compile(r"\[|->")
CAST = re.compile(r"\(\s*[A-Za-z_][\w \*]*\*\s*\)")
ASSIGN = re.compile(r"(?<![=!<>+\-*/%&|^])=(?!=)")


DECL = re.compile(r"^\s+[A-Za-z_][\w\s\*]*\w+\s*(\[\d*\])?\s*;\s*(//.*)?$")
NOTDECL = re.compile(r"^\s*(return|goto|break|continue|else|do|case|default)\b")


def body_start(lines):
    """First body line: a function with no locals has no blank line to find."""
    try:
        i = lines.index("{") + 1
    except ValueError:
        return 0
    while i < len(lines):
        t = lines[i].strip()
        if t == "":
            return i + 1
        if ("=" in t or "(" in t.split("//")[0] or NOTDECL.match(lines[i])
                or not DECL.match(lines[i])):
            return i
        i += 1
    return i

def split_funcs(path):
    out, cur, name = {}, [], None
    for line in open(path, errors="replace").read().splitlines():
        m = FUNC.match(line)
        if m:
            if name:
                out[name] = cur
            name, cur = f"{m.group(1)}@{m.group(2)}", []
        elif name is not None:
            cur.append(line)
    if name:
        out[name] = cur
    return out


def strip(line):
    s = re.sub(r'"(\\.|[^"\\])*"', '""', line)
    s = re.sub(r"'(\\.|[^'\\])*'", "''", s)
    return CAST.sub("(cast)", s)


def scan(s):
    opens = [s.count("(", 0, p) - s.count(")", 0, p) for p in range(len(s) + 1)]
    found = []
    for m in CALL.finditer(s):
        if m.group(1) not in KW:
            found.append((-opens[m.start()], m.start(), "C:" + m.group(1)))
    for pat in (DEREF, GLOBAL, INDEX):
        for m in pat.finditer(s):
            found.append((-opens[m.start()], m.start(), "M"))
    found.sort()
    return [e for _, _, e in found]


def stmt_events(s):
    m = ASSIGN.search(s)
    if m and s.count("(", 0, m.start()) == s.count(")", 0, m.start()):
        return scan(s[m.end():]) + scan(s[:m.start()])
    return scan(s)


def events(lines):
    i = body_start(lines)
    out = []
    for raw in lines[i:]:
        if raw.strip():
            out.extend(stmt_events(strip(raw)))
    return out


bad, tot, changed, order = 0, 0, 0, 0
for b in sys.argv[2:]:
    fo = split_funcs(f"{sys.argv[1]}/{b}.off.c")
    fn = split_funcs(f"{sys.argv[1]}/{b}.on.c")
    for name in fo:
        if name not in fn:
            continue
        tot += 1
        if fo[name] != fn[name]:
            changed += 1
        a, c = events(fo[name]), events(fn[name])
        if a == c:
            continue
        ca, cc = collections.Counter(a), collections.Counter(c)
        if ca != cc:
            bad += 1
            print(f"EVENT SET CHANGED {b} {name}")
            print("   only off:", dict(ca - cc), " only on:", dict(cc - ca))
            continue
        order += 1
        print(f"event order differs (read by hand) {b} {name}")
        for i, (x, y) in enumerate(zip(a, c)):
            if x != y:
                print("   at", i, "off:", a[max(0, i - 3):i + 4],
                      "on:", c[max(0, i - 3):i + 4])
                break
print(f"functions compared={tot} changed={changed} "
      f"event-set-changed={bad} event-order-differs={order}")
