#!/usr/bin/env python3
"""Compare oracle outputs with upstream testData/results/*.dec the way TestDefinition.run does:
getContent() = UTF-8 text, "\\r\\n" -> "\\n", String.trim(); on mismatch retry against <name>_1.dec.

Usage: validate.py --manifest M --vf-src VF --out-root OUT [--categories FILE]
Writes <out-root>/reports/match-<mode>.json and unified diffs under <out-root>/reports/diffs-<mode>/.
"""
import argparse
import difflib
import json
import os


def java_trim(s):
    # String.trim(): strip chars <= U+0020 from both ends
    i, j = 0, len(s)
    while i < j and s[i] <= ' ':
        i += 1
    while j > i and s[j - 1] <= ' ':
        j -= 1
    return s[i:j]


def get_content(path):
    with open(path, 'rb') as f:
        return java_trim(f.read().decode('utf-8', errors='replace').replace('\r\n', '\n'))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--manifest', required=True)
    ap.add_argument('--vf-src', required=True)
    ap.add_argument('--out-root', required=True)
    ap.add_argument('--mode', default='fixture', choices=['fixture', 'defaults'])
    ap.add_argument('--categories', help='JSON {id: category} explaining known mismatches')
    a = ap.parse_args()
    m = json.load(open(a.manifest, encoding='utf-8'))
    cats = json.load(open(a.categories)) if a.categories else {}
    diff_dir = os.path.join(a.out_root, 'reports', f'diffs-{a.mode}')
    rows = []
    for t in m['tests']:
        row = dict(id=t['id'], status=t['status'])
        if t['status'] != 'ok':
            row['result'] = 'skipped'
            rows.append(row)
            continue
        out = os.path.join(a.out_root, t['oracle'] if a.mode == 'fixture' else t['defaults_oracle'])
        if not os.path.isfile(out):
            row['result'] = 'no-output'
            rows.append(row)
            continue
        got = get_content(out)
        ref = get_content(os.path.join(a.vf_src, t['reference']))
        if got == ref:
            row['result'] = 'match'
        elif t['reference_alt'] and got == get_content(os.path.join(a.vf_src, t['reference_alt'])):
            row['result'] = 'match-alt'
            row['category'] = cats.get(t['id'])
        else:
            row['result'] = 'mismatch'
            d = list(difflib.unified_diff(ref.split('\n'), got.split('\n'), 'upstream/' + t['reference'],
                                          'oracle/' + os.path.relpath(out, a.out_root), lineterm='', n=2))
            row['diff_lines'] = sum(1 for x in d if x[:1] in '+-' and not x.startswith(('+++', '---')))
            row['failable'] = t['failable']
            row['category'] = cats.get(t['id'])
            p = os.path.join(diff_dir, t['id'] + '.diff')
            os.makedirs(os.path.dirname(p), exist_ok=True)
            with open(p + '.tmp', 'w', encoding='utf-8') as f:
                f.write('\n'.join(d) + '\n')
            os.replace(p + '.tmp', p)
            row['diff'] = os.path.relpath(p, a.out_root)
        rows.append(row)
    summ = {}
    for r in rows:
        summ[r['result']] = summ.get(r['result'], 0) + 1
    by_cat = {}
    for r in rows:
        if r['result'] == 'mismatch':
            c = r.get('category') or 'UNEXPLAINED'
            by_cat[c] = by_cat.get(c, 0) + 1
    rep = os.path.join(a.out_root, 'reports', f'match-{a.mode}.json')
    with open(rep + '.tmp', 'w') as f:
        json.dump(dict(summary=summ, mismatch_categories=by_cat, rows=rows), f, indent=1)
    os.replace(rep + '.tmp', rep)
    print(json.dumps(dict(summary=summ, mismatch_categories=by_cat)))


if __name__ == '__main__':
    main()
