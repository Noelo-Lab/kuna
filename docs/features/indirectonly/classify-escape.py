"""Classify indirectonly A/B hunks with an escape predicate.

The first round of this A/B only asked whether a hunk removed a declaration or a
copy.  That cannot see the defect that keeps the option off: a merge in the
unsound direction makes the emitted C write a frame slot the machine never
writes, and the damage only shows at a LATER call that was handed the enclosing
object's address.

This pass works on storage, not on variable names.  Every `vN` becomes its
declaration comment -- `S[stack-0xNN]` for a frame slot, `R[reg]` for a register,
`X` for anything unnamed -- which makes the two arms comparable.  Then, per
changed function:

  1. which frame slots does the ON arm ASSIGN that the OFF arm never assigns?
  2. is such a slot inside an object whose address escapes, i.e. is there a `&vB`
     in the function with vB a frame slot at an offset that covers it?
  3. does a call reach that object's address AFTER the new write?

A slot answering yes to all three is an ESCAPE-CANDIDATE: the emitted C now
writes an escaped slot it did not write before, and something downstream can see
it.  That is as far as the text can go.  BOTH directions of the merge produce
this shape -- the sound one, where the machine really does store and the OFF arm
had simply lost the store, looks identical on the page -- so every candidate is
settled by hand against the disassembly: if the machine has no store to that
frame displacement at the merge point, the ON arm fabricated one (BUG); if it
does, the ON arm restored one the OFF arm dropped (FIX).  Verdicts live in
corpus-escape-classification.txt.

Usage: classify-escape.py <off.c> <on.c> <label>
"""
import re, sys, difflib
from collections import OrderedDict

WINDOW = 0x100          # how far back from an escaped base an object may reach

DECL = re.compile(r'^  (?P<ty>[A-Za-z_][^=;]*?)\bv(?P<n>\d+)\b(?P<arr>\s*\[[^\]]*\])?;\s*(//\s*(?P<cm>.*))?$')
ASSIGN = re.compile(r'(?P<t>S\[stack-0x[0-9a-f]+\])\s*(?:=(?!=)|[-+*/%&|^]=|>>=|<<=|\+\+|--)')
CALL = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\s*\(')


def split_funcs(path):
    funcs, cur, buf = OrderedDict(), None, []
    for line in open(path):
        if not line.startswith((' ', '\t', '}', '{')) and '(' in line and ')' in line \
                and not line.rstrip().endswith(';'):
            if cur is not None:
                funcs[cur] = buf
            cur, buf = line.strip(), [line.rstrip('\n')]
        elif cur is not None:
            buf.append(line.rstrip('\n'))
    if cur is not None:
        funcs[cur] = buf
    return funcs


def fname(h):
    m = re.search(r'([A-Za-z_][A-Za-z0-9_]*)\s*\(', h)
    return m.group(1) if m else h


def storage_map(lines):
    m = {}
    for l in lines:
        d = DECL.match(l)
        if not d:
            continue
        cm = (d.group('cm') or '').strip()
        if cm.startswith('stack'):
            m[d.group('n')] = 'S[' + cm.replace(' ', '') + ']'
        elif cm:
            m[d.group('n')] = 'R[' + cm + ']'
        else:
            m[d.group('n')] = None
    return m


def subst(lines, sm):
    body = [l for l in lines[1:] if not DECL.match(l)]
    return [re.sub(r'\bv(\d+)\b', lambda mo: sm.get(mo.group(1)) or 'X', l) for l in body]


def slot_off(tok):
    m = re.match(r'S\[stack-(0x[0-9a-f]+)\]$', tok)
    return int(m.group(1), 16) if m else None


def written_slots(body):
    """every frame slot assigned anywhere on a line, not just at its head"""
    out = {}
    for i, l in enumerate(body):
        for m in ASSIGN.finditer(l):
            out.setdefault(m.group('t'), i)
    return out


def escaped_bases(body):
    bases = {}
    for i, l in enumerate(body):
        for tok in re.findall(r'&(S\[stack-0x[0-9a-f]+\])', l):
            bases.setdefault(tok, []).append(i)
    return bases


def covering(bases, off):
    out = []
    for tok, lines in bases.items():
        b = slot_off(tok)
        if b is not None and off <= b and b - off <= WINDOW:
            out.append((tok, lines))
    return out


def classify(off_lines, on_lines, label, key, report):
    sa, sb = storage_map(off_lines), storage_map(on_lines)
    off_body, on_body = subst(off_lines, sa), subst(on_lines, sb)
    ndecl = len([l for l in on_lines if DECL.match(l)]) - len([l for l in off_lines if DECL.match(l)])
    if off_body == on_body:
        report.append((label, key, 'DECL-ONLY', 'decl_delta=%+d' % ndecl, []))
        return
    off_w, on_w = written_slots(off_body), written_slots(on_body)
    bases = escaped_bases(on_body)
    verdicts = []
    for tok, idx in sorted(((t, i) for t, i in on_w.items() if t not in off_w), key=lambda kv: kv[1]):
        cov = covering(bases, slot_off(tok))
        if not cov:
            verdicts.append(('NEW-WRITE-NO-ESCAPE', tok))
            continue
        after = [b for b, ls in cov if any(l > idx for l in ls)]
        before = [b for b, ls in cov if any(l < idx for l in ls)]
        if after:
            verdicts.append(('ESCAPE-CANDIDATE', '%s inside %s, address passed again after the write'
                             % (tok, ','.join(after))))
        elif before and any(CALL.search(l) for l in on_body[idx + 1:]):
            verdicts.append(('ESCAPE-CANDIDATE-WEAK',
                             '%s inside %s, address escaped before the write, a call follows'
                             % (tok, ','.join(before))))
        else:
            verdicts.append(('NEW-WRITE-DEAD', tok))
    kinds = [v[0] for v in verdicts]
    kind = 'ESCAPE-CANDIDATE' if 'ESCAPE-CANDIDATE' in kinds else \
           'ESCAPE-CANDIDATE-WEAK' if 'ESCAPE-CANDIDATE-WEAK' in kinds else \
           'NEW-WRITE' if verdicts else 'SAFE'
    diff = [l for l in difflib.unified_diff(off_body, on_body, lineterm='', n=1)
            if not l.startswith(('---', '+++'))]
    report.append((label, key, kind, 'decl_delta=%+d new_slot_writes=%d' % (ndecl, len(verdicts)),
                   ['%s %s' % v for v in verdicts] + diff))


def main():
    off, on, label = sys.argv[1], sys.argv[2], sys.argv[3]
    fa, fb = split_funcs(off), split_funcs(on)
    ka = {fname(h): h for h in fa}
    kb = {fname(h): h for h in fb}
    report = []
    for k in ka:
        if k not in kb or fa[ka[k]] == fb[kb[k]]:
            continue
        classify(fa[ka[k]], fb[kb[k]], label, k, report)
    for lab, key, kind, stat, detail in report:
        print('%s %s %s %s' % (lab, key, kind, stat))
        for d in detail:
            print('    ' + d)
    kinds = {}
    for _, _, kind, _, _ in report:
        kinds[kind] = kinds.get(kind, 0) + 1
    print('%s changed_functions=%d %s' % (label, len(report), kinds))


main()
