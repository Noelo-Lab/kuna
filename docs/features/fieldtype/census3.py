"""Every cast whose operand is a record member access (p->field_0xK / &p->field_0xK), by base kind, shared set."""
import sys, re, collections
from pathlib import Path
sys.path.insert(0, '/home/mahaloz/kwt/castbench')
import castcount as CC, castbench as CB
sys.path.insert(0, str(Path(__file__).resolve().parent))
from census2 import find_casts_j, decls, LIBC
arm = Path(sys.argv[1]); which = sys.argv[2] if len(sys.argv) > 2 else 'full'
cnt = collections.Counter(); ex = collections.defaultdict(list); fields = collections.Counter()
for opt, proj, b in CB.corpus(which):
    kp = arm / opt / proj / f'{b}.c'; ip = CB.RES / opt / proj / 'decompiled' / f'ida_{b}.c'
    if not kp.exists() or not ip.exists(): continue
    ks = kp.read_text(errors='replace'); is_ = ip.read_text(errors='replace')
    kv = CC.harvest_types(CC.tokenize(ks), ks)
    kf = {a: t for _, a, t in CC.split_functions(ks)}; iff = {a for _, a, t in CC.split_functions(is_)}
    for a in set(kf) & iff:
        text = kf[a]; toks = CC.tokenize(text); d = decls(text); lines = text.splitlines()
        for ty, shape, line, _, j in find_casts_j(toks, kv):
            after = ''.join(t.text for t in toks[j+1:j+8])
            m = re.match(r'(&?)([A-Za-z_][A-Za-z0-9_]*)(->|\.)(field_0x[0-9a-f]+)(\[)?', after)
            if not m: continue
            nm = m.group(2)
            bt = d.get(nm, 'FILE' if nm in ('stdout','stderr','stdin') else ('?'))
            if m.group(3) == '.': bt = 'byvalue'
            k = 'struct_N' if re.match(r'struct_\d+$', bt) else ('libc' if bt in LIBC else bt)
            form = ('&' if m.group(1) else 'val') + ('[i]' if m.group(5) else '')
            ptr = 'ptr' if '*' in ty else 'scalar'
            key = (k, form, ptr)
            cnt[key] += 1
            if len(ex[key]) < 5: ex[key].append(f'{opt}/{b}@{hex(a)}: {lines[line-1].strip()[:150] if 0<line<=len(lines) else ""}')
for k, v in cnt.most_common(): print(f'{v:6d}  {k}')
for k in list(cnt)[:40]:
    if k[0] in ('struct_N',):
        print('==', k); [print('   ', e) for e in ex[k]]
