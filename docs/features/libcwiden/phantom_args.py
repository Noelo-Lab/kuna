"""Count the phantom trailing argument the unported clearStackPlaceholders left behind.

A phantom is a call to a name the SAME whole-binary `kuna decompile-all` output
declares non-variadic, carrying exactly one argument more than that declared
arity, whose last argument is a bare stack-array element at a negative
displacement (the slot the `call` pushed its return address into).

Usage:  python3 phantom_args.py <file.c> [<file.c> ...]
        # prints one line per file plus a total and the top callee names

Measured over 12 whole binaries (coreutils ginstall/stty/chcon/df/stat,
grep, gzip, tar, findutils find, diffutils diff, bzip2, shadow chage):
origin/main 9e07ab931 = 148, main with `--option libcsigs off` = 100,
this branch = 0 in both arms.
"""
import collections
import re
import sys

HDR = re.compile(r'^[A-Za-z_][\w \*]*?\s\**([A-Za-z_]\w*)\((.*)\)\s*;?\s*$')
PLACEHOLDER = re.compile(r'^v\d+\[-\d+\]$')


def split_args(s):
    out, depth, cur = [], 0, ''
    for ch in s:
        if ch in '([':
            depth += 1
        if ch in ')]':
            depth -= 1
        if ch == ',' and depth == 0:
            out.append(cur.strip())
            cur = ''
            continue
        cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


def scan(path):
    lines = open(path).read().splitlines()
    arity = {}
    for line in lines:
        if line and not line.startswith(' ') and not line.startswith('//') and not line.endswith('{'):
            h = HDR.match(line)
            if h and '...' not in h.group(2):
                params = split_args(h.group(2))
                arity.setdefault(h.group(1), 0 if params in ([], ['void']) else len(params))
    n, by_name = 0, collections.Counter()
    for line in lines:
        for m in re.finditer(r'\b([A-Za-z_]\w*)\(([^;{}]*)\)', line):
            name = m.group(1)
            if name not in arity:
                continue
            args = split_args(m.group(2))
            if len(args) == arity[name] + 1 and PLACEHOLDER.match(args[-1]):
                n += 1
                by_name[name] += 1
    return n, by_name


def main(paths):
    total, agg = 0, collections.Counter()
    for p in paths:
        n, by_name = scan(p)
        total += n
        agg.update(by_name)
        print(f'{n:5d}  {p}')
    print(f'{total:5d}  TOTAL   top={agg.most_common(6)}')


if __name__ == '__main__':
    main(sys.argv[1:])
