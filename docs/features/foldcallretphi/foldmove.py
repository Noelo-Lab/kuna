"""How far did a folded call actually move, and what did it move past?

`callorder.py` only compares call-to-call order, so a call reordered against a
memory read is invisible to it.  This script answers the stronger question the
option's soundness argument rests on: for every call whose spill statement
`foldcallretphi` deleted, what statements stood between that statement and the
one the call expression now lives in?

For each function of a `kuna decompile-all` off/on pair it

  1. masks every `vN` (the fold renumbers each later local, which otherwise
     swamps a textual comparison),
  2. finds each masked line `V = <expr containing a call>;` that the ON body no
     longer carries, and whose `<expr>` reappears inside an ON line,
  3. maps that ON line back to the OFF statement it came from (the ON line with
     `<expr>` put back as `V`), and
  4. classifies every OFF statement strictly between the two.

A statement is a HAZARD when it is a control-flow boundary (`if`/`while`/`do`/
`for`/`switch`/`else`/`goto`/`return`/`break`/a label/a brace) or when it
touches memory or calls out (a call, `*` dereference, `[...]` index, `->`, or a
`dat_`/`unk_` global).  Anything else is register arithmetic the call may pass.
Zero hazards over the whole sweep is the claim; a call that crosses any of them
has had its evaluation point moved and is a bug.

    python3 foldmove.py <sweep-dir> <name>...
      # reads <sweep-dir>/<name>.off.c and <sweep-dir>/<name>.on.c
"""
import re, sys, collections

FUNC = re.compile(r"^// Function: (\S+) @ (\S+)$")
VAR = re.compile(r"\bv\d+\b")
CALL = re.compile(r"\b([A-Za-z_]\w*)\s*\(")
KW = {"if", "while", "for", "switch", "return", "do", "else", "sizeof", "break",
      "continue", "goto"}
CF = re.compile(r"^\s*(\}|\{|if\b|else\b|while\b|do\b|for\b|switch\b|case\b|"
                r"default\b|goto\b|return\b|break\b|continue\b|\w+:\s*$)")
DEF = re.compile(r"^\s*V = (.+);\s*$")


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


def body(lines):
    """Everything after the declaration block (up to its trailing blank line)."""
    try:
        i = lines.index("{") + 1
    except ValueError:
        return [l for l in lines if l.strip()]
    while i < len(lines) and lines[i].strip() != "":
        i += 1
    return [VAR.sub("V", l) for l in lines[i:] if l.strip()]


def has_call(s):
    return any(m.group(1) not in KW for m in CALL.finditer(s))


def hazard(stmt):
    s = stmt.strip()
    if CF.match(s):
        return "control-flow"
    s = re.sub(r'"(\\.|[^"\\])*"', '""', s)
    if has_call(s):
        return "call"
    if "*" in s or "[" in s or "->" in s:
        return "memory"
    if re.search(r"\b(dat|unk|byte|word|dword|qword)_[0-9a-fA-F]+\b", s):
        return "global"
    return None


counts = collections.Counter()
hazards, absent, unmatched = [], [], []
moves = collections.Counter()

for b in sys.argv[2:]:
    fo = split_funcs(f"{sys.argv[1]}/{b}.off.c")
    fn = split_funcs(f"{sys.argv[1]}/{b}.on.c")
    for name in fo:
        if name not in fn:
            continue
        counts["functions"] += 1
        mo, mn = body(fo[name]), body(fn[name])
        if mo == mn:
            continue
        counts["changed_functions"] += 1
        co, cn = collections.Counter(mo), collections.Counter(mn)
        gone = {l for l in co if co[l] > cn.get(l, 0)}   # statements the ON body lost
        for i, line in enumerate(mo):
            if line not in gone:
                continue
            m = DEF.match(line)
            if not m or not has_call(m.group(1)):
                continue
            expr = m.group(1)
            hosts = [l for l in mn if expr in l]
            if not hosts:
                counts["spill_gone_call_text_absent"] += 1
                absent.append((b, name, expr[:70]))
                continue
            # Map the ON host line back to the OFF statement it came from.
            cands = set()
            for h in hosts:
                cands.add(h.replace(expr, "V", 1))
                if "(%s)" % expr in h:
                    cands.add(h.replace("(%s)" % expr, "V", 1))
            landing = [u for u in range(i + 1, len(mo))
                       if mo[u] in cands and mo[u] in gone]
            if not landing:
                counts["landing_unmatched"] += 1
                unmatched.append((b, name, expr[:70]))
                continue
            if len(landing) > 1:
                counts["landing_ambiguous"] += 1
            u = landing[0]
            counts["folds"] += 1
            between = mo[i + 1:u]
            moves[len(between)] += 1
            for stmt in between:
                k = hazard(stmt)
                if k:
                    counts["HAZARD_" + k] += 1
                    hazards.append((b, name, expr[:60], k, stmt.strip()[:70]))
                else:
                    counts["benign_intervening"] += 1

print("counters:", dict(counts))
print("statements between the deleted spill and the landing statement:",
      dict(sorted(moves.items())))
for a in absent[:20]:
    print("CALL-TEXT-ABSENT", a)
for x in unmatched[:20]:
    print("LANDING-UNMATCHED", x)
for h in hazards[:40]:
    print("HAZARD", h)
print("HAZARDS:", len(hazards))
