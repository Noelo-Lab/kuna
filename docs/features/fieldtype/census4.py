"""struct_N field cast census over decompile-project exports: field declared type vs cast type, shared set."""
import sys, re, collections, json
from pathlib import Path
sys.path.insert(0, '/home/mahaloz/kwt/castbench'); sys.path.insert(0, str(Path(__file__).resolve().parent))
import castcount as CC, castbench as CB
from census2 import find_casts_j, decls
root = Path(sys.argv[1]); which = sys.argv[2] if len(sys.argv) > 2 else 'full'
STRUCT = re.compile(r'^struct (struct_\d+) \{\n(.*?)^\};', re.M | re.S)
FLD = re.compile(r'^\s*(.+?)\s*\b(field_0x[0-9a-f]+)(\[\d+\])?;', re.M)
cnt = collections.Counter(); ex = collections.defaultdict(list); pairs = collections.Counter()
rows = []
for opt, proj, b in CB.corpus(which):
    d = root / opt / proj / b
    hs = list(d.glob('*.h')); cs = list(d.glob('*.c'))
    ip = CB.RES / opt / proj / 'decompiled' / f'ida_{b}.c'
    if not hs or not cs or not ip.exists(): continue
    h = hs[0].read_text(errors='replace'); c = cs[0].read_text(errors='replace')
    structs = {}
    for m in STRUCT.finditer(h):
        fl = {}
        for f in FLD.finditer(m.group(2)):
            fl[f.group(2)] = f.group(1).strip() + (f.group(3) or '')
        structs[m.group(1)] = fl
    kv = CC.harvest_types(CC.tokenize(c), c)
    ida = {a for _, a, _ in CC.split_functions(ip.read_text(errors='replace'))}
    for name, a, text in CC.split_functions(c):
        if a not in ida: continue
        toks = CC.tokenize(text); dd = decls(text); lines = text.splitlines()
        for ty, shape, line, _, j in find_casts_j(toks, kv):
            after = ''.join(t.text + (' ' if t.kind == 'id' else '') for t in toks[j+1:j+8])
            m = re.match(r'(&?)([A-Za-z_]\w*) ?(->|\.)(field_0x[0-9a-f]+) ?(\[)?', after)
            if not m or m.group(3) != '->': continue
            bt = dd.get(m.group(2), '?')
            if not re.match(r'struct_\d+$', bt): continue
            ft = structs.get(bt, {}).get(m.group(4), '<hole>')
            form = ('&' if m.group(1) else 'val') + ('[i]' if m.group(5) else '')
            key = (form, ft, ty)
            cnt[key] += 1
            L = lines[line-1].strip()[:160] if 0 < line <= len(lines) else ''
            rows.append(dict(opt=opt, bin=b, addr=hex(a), fn=name, rec=bt, field=m.group(4), decl=ft, cast=ty, form=form, line=L))
            if len(ex[key]) < 4: ex[key].append(f'{opt}/{b}@{hex(a)} {bt}.{m.group(4)}: {L}')
print('struct_N member casts on shared set:', sum(cnt.values()))
for k, v in cnt.most_common(60): print(f'{v:5d}  {k}')
json.dump(rows, open(root / 'census4.json', 'w'), indent=0)
for k, _ in cnt.most_common(25):
    print('==', k); [print('   ', e) for e in ex[k]]
