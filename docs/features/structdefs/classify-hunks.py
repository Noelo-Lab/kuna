"""Classify every ON-vs-OFF hunk of the structdefs corpus sweep.

The claim under test: turning `structdefs` on only INSERTS type-definition lines
above a function, and changes no line of any function body.  Every unified-diff
hunk is therefore expected to be a pure insertion (no deleted line) whose added
lines are all type-definition text (a forward typedef, a struct/union/enum body
line, a typedef line, a rendering comment, or a blank separator).
"""
import difflib, glob, os, re, sys, json

DEF_LINE = re.compile(
    r"^("
    r"typedef "                       # forward declaration / typedef line
    r"|(struct|union) \w+ \{"         # a composite body opening
    r"|\};"                           # ... and its close
    r"|\}\s*\w+;$"                    # an enum body's close
    r"|\s+/\* "                       # a rendering note (bitfield/pad comment)
    r"|\s+[\w\*\[\]\.\(\) ]+;$"     # a member declaration
    r"|\s+[\w\*\[\]\.\(\) ]+ : \d+;$"  # a bitfield member
    r"|\s+\w+ = -?[\dx][\w]*,?$"      # an enum constant
    r"|/\* "                          # a top-level rendering note
    r")")

def defish(line):
    s = line.rstrip("\n")
    if s == "":
        return True
    return bool(DEF_LINE.match(s))

rows = []
for on in sorted(glob.glob(".scratch/ab/*.on.txt")):
    off = on[:-len(".on.txt")] + ".mine.txt"
    a = open(off).read().splitlines(keepends=True)
    b = open(on).read().splitlines(keepends=True)
    hunks = ins = dels = odd = 0
    oddlines = []
    for group in difflib.SequenceMatcher(None, a, b, autojunk=False).get_grouped_opcodes(0):
        hunks += 1
        for tag, i1, i2, j1, j2 in group:
            if tag == "equal":
                continue
            if tag in ("delete", "replace"):
                dels += i2 - i1
                oddlines += [("-", l) for l in a[i1:i2][:3]]
            if tag in ("insert", "replace"):
                ins += j2 - j1
                for l in b[j1:j2]:
                    if not defish(l):
                        odd += 1
                        oddlines.append(("+", l))
    rows.append(dict(binary=os.path.basename(on)[:-len(".on.txt")], hunks=hunks,
                     added=ins, deleted=dels, non_definition_added=odd,
                     samples=[f"{s}{l.rstrip()}" for s, l in oddlines[:5]]))
print(json.dumps(rows, indent=2))
tot_del = sum(r["deleted"] for r in rows)
tot_odd = sum(r["non_definition_added"] for r in rows)
print(f"\nTOTAL hunks={sum(r['hunks'] for r in rows)} added={sum(r['added'] for r in rows)} "
      f"deleted={tot_del} non_definition_added={tot_odd}")
sys.exit(1 if (tot_del or tot_odd) else 0)
