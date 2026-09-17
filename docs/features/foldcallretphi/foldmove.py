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

This is a text matcher with no C parser, so it maps only the folds it can map,
and it says so.  Every removed spill statement is counted as LOCATED, and each
one ends up in exactly one bucket:

  classified              the landing statement was found; hazards are counted
                          over the statements in between
  call_text_absent        the ON body carries no line containing the call text,
                          so the fold cannot be followed (the surrounding
                          expression was rewritten, or the statement went away
                          with the value)
  landing_unmatched       the call text is in the ON body but no OFF statement
                          maps onto it within the diff hunk
  landing_ambiguous       more than one OFF statement maps onto it, or the same
                          masked spill text occurs more than once in the
                          function, so which statement is the landing is a guess

The last bucket is why a bare "0 hazards" is not the whole claim: an ambiguous
mapping can point at a statement far from the real landing and then report every
statement in between as a hazard.  On `tar` (O2) that happens twice, each time
because `V = sub_4ce40(dat_82b28);` occurs twice in one function, and the older
form of this script reported 65 hazards from those two functions alone, all
false.  Ambiguous mappings are therefore not classified here, and the
unclassified buckets are covered by `evalorder.py`, which needs no mapping: it
compares the whole ordered stream of calls and memory touches per function.

    python3 foldmove.py <sweep-dir> <name>...
      # reads <sweep-dir>/<name>.off.c and <sweep-dir>/<name>.on.c
"""
import re, sys, collections, difflib

FUNC = re.compile(r"^// Function: (\S+) @ (\S+)$")
VAR = re.compile(r"\bv\d+\b")
CALL = re.compile(r"\b([A-Za-z_]\w*)\s*\(")
KW = {"if", "while", "for", "switch", "return", "do", "else", "sizeof", "break",
      "continue", "goto"}
CF = re.compile(r"^\s*(\}|\{|if\b|else\b|while\b|do\b|for\b|switch\b|case\b|"
                r"default\b|goto\b|return\b|break\b|continue\b|\w+:\s*$)")
DEF = re.compile(r"^\s*V = (.+);\s*$")
CAST = re.compile(r"\(\s*[A-Za-z_][\w \*]*\*\s*\)")


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


def body(lines):
    """Everything after the declaration block, with every `vN` masked."""
    return [VAR.sub("V", l) for l in lines[body_start(lines):] if l.strip()]


def has_call(s):
    return any(m.group(1) not in KW for m in CALL.finditer(s))


def hazard(stmt):
    s = stmt.strip()
    if CF.match(s):
        return "control-flow"
    s = re.sub(r'"(\\.|[^"\\])*"', '""', s)
    s = CAST.sub("", s)          # a cast's `*` is not a dereference
    if has_call(s):
        return "call"
    if "*" in s or "[" in s or "->" in s:
        return "memory"
    if re.search(r"\b(dat|unk|byte|word|dword|qword)_[0-9a-fA-F]+\b", s):
        return "global"
    return None


counts = collections.Counter()
hazards, absent, unmatched, ambiguous, far = [], [], [], [], []
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
        # The landing statement has itself changed, so it lies in this hunk or
        # the next one; that bound keeps a textually identical statement much
        # later in the function from being mistaken for it.
        blocks = [(i1, i2) for tag, i1, i2, _, _ in
                  difflib.SequenceMatcher(None, mo, mn, autojunk=False).get_opcodes()
                  if tag != "equal"]

        def reach(i):
            for n, (i1, i2) in enumerate(blocks):
                if i1 <= i < i2:
                    return blocks[n + 1][1] if n + 1 < len(blocks) else len(mo)
            return len(mo)
        # Group the removed spill statements by their masked text: kuna reuses
        # one `vN` for several calls, so the same masked line can appear more
        # than once and only the nearest one owns a given landing statement.
        spills = collections.defaultdict(list)
        for i, line in enumerate(mo):
            if line not in gone:
                continue
            m = DEF.match(line)
            if m and has_call(m.group(1)):
                spills[line].append(i)
        for line, idxs in spills.items():
            expr = DEF.match(line).group(1)
            # The call text has to name one statement on each side, or which
            # statement is the landing is a guess.  Two spills of the same call
            # in one function (kuna prints them at different indents, so they
            # are different lines) is the shape that makes a text matcher point
            # at a statement far from the real landing and report everything in
            # between as a hazard.
            if (sum(1 for l in mo if expr in l) > 1
                    or sum(1 for l in mn if expr in l) > 1):
                counts["located"] += len(idxs)
                counts["landing_ambiguous"] += len(idxs)
                ambiguous.append((b, name, expr[:70], "call text is not unique"))
                continue
            # An unchanged copy of the spill statement elsewhere in the ON body
            # is not a landing: it would map back onto itself.
            hosts = [l for l in mn if expr in l and l != line]
            if not hosts:
                counts["located"] += len(idxs)
                counts["call_text_absent"] += len(idxs)
                absent.append((b, name, expr[:70]))
                continue
            cands = set()
            for h in hosts:
                for c in (h.replace(expr, "V", 1),
                          h.replace("(%s)" % expr, "V", 1) if "(%s)" % expr in h
                          else None):
                    if c is not None and c != line:
                        cands.add(c)
            for n, i in enumerate(idxs):
                counts["located"] += 1
                stop = min(idxs[n + 1] if n + 1 < len(idxs) else len(mo), reach(i))
                # Prefer a landing the ON body no longer carries verbatim; fall
                # back to any match for a statement that also appears elsewhere.
                landing = ([u for u in range(i + 1, stop)
                            if mo[u] in cands and mo[u] in gone]
                           or [u for u in range(i + 1, stop) if mo[u] in cands])
                if not landing:
                    counts["landing_unmatched"] += 1
                    unmatched.append((b, name, expr[:70]))
                    continue
                # Which statement is the landing has to be forced, not picked:
                # one candidate, and one occurrence of this spill text.
                if len(landing) > 1 or len(idxs) > 1:
                    counts["landing_ambiguous"] += 1
                    ambiguous.append((b, name, expr[:70], len(idxs), len(landing),
                                      [u - i for u in landing[:4]]))
                    continue
                u = landing[0]
                counts["classified"] += 1
                between = mo[i + 1:u]
                moves[len(between)] += 1
                if len(between) > 1:
                    far.append((b, name, i, u, expr[:70], mo[u].strip()[:70]))
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
for x in ambiguous[:20]:
    print("LANDING-AMBIGUOUS", x)
for f in far:
    print("FAR", f)
for h in hazards[:40]:
    print("HAZARD", h)
loc = counts["located"]
unc = counts["call_text_absent"] + counts["landing_unmatched"] + counts["landing_ambiguous"]
print("LOCATED: %d = classified %d + call_text_absent %d + landing_unmatched %d"
      " + landing_ambiguous %d  (unclassified %.1f%%)"
      % (loc, counts["classified"], counts["call_text_absent"],
         counts["landing_unmatched"], counts["landing_ambiguous"],
         100.0 * unc / loc if loc else 0.0))
print("HAZARDS:", len(hazards))
