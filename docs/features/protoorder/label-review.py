#!/usr/bin/env python3
"""Label every function `corpus-diff.py` leaves in REVIEW.

    python3 label-review.py <dir> [<dir> ...] > review-labels.json

For each REVIEW function the two bodies are reduced to the lines that still
differ after the deepest `corpus-diff.py` normalizer, and the residue is labelled:

  store-width   the ON arm stores the same constant bytes through a narrower
                pointer: one wide store becomes several byte stores at
                consecutive offsets (`*(int *)p = 0xefbeadde` -> `p[0] = 0xde`...)
  ternary       an if/else that assigns one variable becomes a `?:`, or back
  spelling      the residue is equal once every parenthesis and every `*1`
                factor is dropped: the same arithmetic, grouped differently
  literal       the residue is equal once every number is a constant token: a
                small address (a firmware string below 0x1000) printed as its
                string, or a constant printed at another width
  code-subscript  DEFECT, checked first: the ON arm prints a code address as a
                subscript of a function (`&sub_22c[1]` for the Thumb address
                0x22d), which is not C.  Never folded into `literal`.
  HAND          anything else, labelled from `hand-labels.json` beside this
                script (a class and a one-line reason per function)
"""
import collections
import difflib
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('cd', os.path.join(HERE, 'corpus-diff.py'))
cd = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cd)


def residue(f0, f1, ptr):
    f0, f1 = dict(f0, code=cd.renumber(f0['code'])), dict(f1, code=cd.renumber(f1['code']))
    z0, z1 = cd.pointee_sizes(f0['code'], ptr), cd.pointee_sizes(f1['code'], ptr)
    def lines(c, z):
        raw = [x for x in (re.sub(r'\s+', ' ', re.sub(r'//.*$', '', l)).strip() for l in c.splitlines()[1:])
               if x and not cd.DECL.match(x)]
        return [cd.merged(cd.literal(cd.member_reach(x, z))) for x in raw]
    a, b = lines(f0['code'], z0), lines(f1['code'], z1)
    sm = difflib.SequenceMatcher(a=a, b=b, autojunk=False)
    minus, plus = [], []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag != 'equal':
            minus += a[i1:i2]
            plus += b[j1:j2]
    return minus, plus


CODE_SUBSCRIPT = re.compile(r'&(?:sub_[0-9a-f]+|FUN_[0-9a-f]+)\[')
STORE = re.compile(r'^\*\(?([VA][^=]*?)(?:\+(\d+))?\)?=(\d+|K);$')


def store_width(minus, plus):
    if not plus or len(plus) <= len(minus):
        return False
    if not all(STORE.match(x) for x in minus + plus):
        return False
    bases = {STORE.match(x).group(1) for x in minus + plus}
    return len(bases) <= 2


def ternary(minus, plus):
    joined0, joined1 = ''.join(minus), ''.join(plus)
    has_if = lambda s: s.startswith('if') or 'else' in s
    return ('?' in joined0) != ('?' in joined1) and (has_if(joined0) or has_if(joined1))


def literal(minus, plus):
    k = lambda xs: [re.sub(r'(?<![\w.])\d+(?![\w.])', 'K', x) for x in xs]
    return k(minus) == k(plus)


def spelling(minus, plus):
    flat = lambda xs: re.sub(r'[()]|\*1(?!\d)', '', ''.join(xs))
    return flat(minus) == flat(plus)


def main(dirs):
    hand = {}
    hp = os.path.join(HERE, 'hand-labels.json')
    if os.path.exists(hp):
        hand = json.load(open(hp))
    rows, tally = [], collections.Counter()
    for d in dirs:
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.off.json'):
                continue
            stem = fn[:-len('.off.json')]
            offdoc = json.load(open(os.path.join(d, fn)))
            on = {f['address']: f for f in json.load(open(os.path.join(d, stem + '.on.json')))['functions']}
            ptr = next((v.get('size') for f in offdoc['functions'] for v in f.get('variables') or []
                        if str(v.get('type') or '').endswith('*') and v.get('size')), 8)
            for f0 in offdoc['functions']:
                f1 = on.get(f0['address'])
                if not f1 or f0.get('code') == f1.get('code') or cd.classify(f0, f1, ptr) != 'REVIEW':
                    continue
                minus, plus = residue(f0, f1, ptr)
                key = f"{stem} {f0['name']}"
                if len(CODE_SUBSCRIPT.findall(f1['code'])) > len(CODE_SUBSCRIPT.findall(f0['code'])):
                    label, why = 'code-subscript', 'DEFECT: a function subscripted as data'
                elif key in hand:
                    label, why = hand[key]['class'], hand[key]['why']
                elif store_width(minus, plus):
                    label, why = 'store-width', ''
                elif ternary(minus, plus):
                    label, why = 'ternary', ''
                elif spelling(minus, plus):
                    label, why = 'spelling', ''
                elif literal(minus, plus):
                    label, why = 'literal', ''
                else:
                    label, why = 'HAND', ''
                tally[label] += 1
                rows.append({'function': key, 'class': label, 'why': why,
                             'off': minus[:8], 'on': plus[:8]})
    json.dump({'tally': dict(sorted(tally.items())), 'functions': rows}, sys.stdout, indent=1)
    print()


if __name__ == '__main__':
    main(sys.argv[1:])
