"""Classify every hunk of the whole-binary `decompile-all` before/after diff."""
import difflib, glob, os, re, sys, collections

NAMES = r"(FILE|DIR|dirent|group|lconv|mbstate_t|obstack|option|passwd|pthread_mutex_t|re_pattern_buffer|sigaction|sigset_t|sockaddr|spwd|stat|statfs|termios|timespec|timeval|tm|utmp|utmpx)"
FUNC = re.compile(r'^// Function: (\S+) @ (\S+)')

def fns(path):
    out, cur, buf = {}, None, []
    for line in open(path, errors='replace'):
        m = FUNC.match(line)
        if m:
            if cur: out[cur] = buf
            cur, buf = m.group(2), [line]
        elif cur:
            buf.append(line)
    if cur: out[cur] = buf
    return out

def classify(minus, plus):
    m, p = "".join(minus), "".join(plus)
    def has(pat, s): return re.search(pat, s) is not None
    # a signature line
    if any(l.startswith(('void ', 'int ', 'char ', 'long ', 'unsigned ', 'short ', 'float ', 'double ')) or ' sub_' in l
           for l in minus + plus) and any(re.search(r'^\w[\w \*]*\**\s*\w+\(', l) for l in minus + plus):
        if has(NAMES + r' \*', p) and not has(NAMES + r' \*', m): return "signature gains a named pointee"
    if re.search(r'^\s*' + NAMES + r' \*?\w+;', p, re.M) and not re.search(r'^\s*' + NAMES + r' \*?\w+;', m, re.M):
        return "declaration gains a named pointee"
    if has(r'->field_0x', p) and not has(r'->field_0x', m): return "cast at an offset becomes a field"
    if has(r'->field_0x', m) and not has(r'->field_0x', p): return "field becomes a cast at an offset"
    if has(r'\(' + NAMES + r' \*\)', p) and not has(r'\(' + NAMES + r' \*\)', m): return "a named cast appears"
    if re.sub(r'\bstruct_\d+\b', 'S', m) == re.sub(r'\bstruct_\d+\b', 'S', p) and has(r'struct_\d', m):
        return "synthesized struct_N renumbered (one fewer slot in the ledger)"
    if re.sub(r'\bv\d+\b', 'V', m) == re.sub(r'\bv\d+\b', 'V', p): return "local renumbering only"
    if re.sub(r'\b(v\d+|struct_\d+)\b', 'X', m) == re.sub(r'\b(v\d+|struct_\d+)\b', 'X', p):
        return "synthesized struct_N renumbered (one fewer slot in the ledger)"
    if has(r'jump-as-call', m) and has(r'jump-as-call', p):
        return "a PLT thunk gains the return the declaration states"
    def defield(t):
        return re.sub(r'\*\(\s*[A-Za-z_][\w \*]*\)&(\w+)->', r'\1->', t)
    if defield(m) == defield(p): return "a field access gains or loses its width cast"
    if re.sub(r'\b(v\d+|struct_\d+|[A-Za-z_]\w*\s+\*?)\b', 'X', m) == \
       re.sub(r'\b(v\d+|struct_\d+|[A-Za-z_]\w*\s+\*?)\b', 'X', p):
        return "a local's spelling or width moves under the new pointee"
    if has(NAMES, p) or has(NAMES, m): return "other, mentions a named type"
    return "other, downstream of a changed pointee"

tot = collections.Counter(); per = {}
for off in sorted(glob.glob(sys.argv[1] + "/*.off.c")):
    tag = os.path.basename(off)[:-len(".off.c")]
    on = off[:-len(".off.c")] + ".on.c"
    if not os.path.exists(on): continue
    a, b = fns(off), fns(on)
    changed = 0; hunks = collections.Counter()
    for k in sorted(set(a) & set(b)):
        if a[k] == b[k]: continue
        changed += 1
        sm = difflib.SequenceMatcher(None, a[k], b[k])
        for op, i1, i2, j1, j2 in sm.get_opcodes():
            if op == 'equal': continue
            c = classify(a[k][i1:i2], b[k][j1:j2])
            hunks[c] += 1; tot[c] += 1
    per[tag] = (len(a), changed, sum(hunks.values()), hunks)
w = max(len(t) for t in per)
print(f"{'binary':{w}s} {'fns':>5s} {'changed':>8s} {'hunks':>6s}")
for t, (n, c, h, _) in sorted(per.items()):
    print(f"{t:{w}s} {n:5d} {c:8d} {h:6d}")
print(f"\n{'total':{w}s} {sum(v[0] for v in per.values()):5d} {sum(v[1] for v in per.values()):8d} {sum(tot.values()):6d}")
print("\nhunks by class:")
for k, v in tot.most_common(): print(f"  {v:6d}  {k}")
