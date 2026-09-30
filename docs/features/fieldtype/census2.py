"""Split every (T *)&... cast on the shared set by the operand's form and base record kind."""
import sys, re, collections, json
from pathlib import Path
sys.path.insert(0, '/home/mahaloz/kwt/castbench')
import castcount as CC, castbench as CB
arm = Path(sys.argv[1]) if len(sys.argv)>1 else None; which = sys.argv[2] if len(sys.argv) > 2 else 'full'
LIBC = set("DIR FILE _IO_FILE dirent group lconv mbstate_t obstack option passwd pthread_mutex_t re_pattern_buffer sigaction sigset_t sockaddr spwd stat statfs termios timespec timeval tm utmp utmpx".split())
def ptr_level(t): return t.count('*')
cnt = collections.Counter(); ida_cnt = collections.Counter(); ex = collections.defaultdict(list)
def decls(text):
    d = {}
    for m in re.finditer(r'([A-Za-z_][A-Za-z0-9_]*)\s*(\*+)\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?=[;,)\[])', text):
        d.setdefault(m.group(3), m.group(1))
    return d
def find_casts_j(toks, vocab):
    n = len(toks); match = CC.matching(toks); out = []; cc = set()
    for i in range(n):
        if toks[i].text != "(": continue
        j = match.get(i)
        if j is None or j == i + 1: continue
        if not CC.cast_allowed_before(toks, i, cc): continue
        k = j + 1
        if k >= n: continue
        nt = toks[k]
        if not (nt.kind in ("id", "num", "str", "chr") or nt.text in CC.UNARY_START_PUNCT): continue
        ty = CC.parse_type_name(toks, i + 1, j, vocab)
        if ty is None: continue
        cc.add(j)
        out.append((ty, CC.operand_shape(toks, k, n), toks[i].line, "", j))
    return out
def classify(text, vocab, is_kuna):
    toks = CC.tokenize(text); out = []
    d = decls(text)
    lines = text.splitlines()
    for ty, shape, line, span, j in find_casts_j(toks, vocab):
        if shape != '<addr>' or '*' not in ty: continue
        # find the tokens after this cast: locate span position
        idx = text.find(span)
        rest = text[text.find(span, 0) + len(span):]
        # use regex on tokens after the cast in this line
        m = None
        L = lines[line - 1] if 0 < line <= len(lines) else ''
        after = ''.join(t.text for t in toks[j+1:j+12])
        m1 = re.match(r'&\s*([A-Za-z_][A-Za-z0-9_]*)\s*->\s*(field_0x[0-9a-f]+)', after)
        m2 = re.match(r'&\s*([A-Za-z_][A-Za-z0-9_]*)\s*->\s*([A-Za-z_][A-Za-z0-9_]*)', after)
        m3 = re.match(r'&\s*([A-Za-z_][A-Za-z0-9_]*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)', after)
        m4 = re.match(r'&\s*([A-Za-z_][A-Za-z0-9_]*)\s*\[', after)
        m5 = re.match(r'&\s*([A-Za-z_][A-Za-z0-9_]*)', after)
        if m1:
            b = d.get(m1.group(1), 'FILE' if m1.group(1) in ('stdout','stderr','stdin') else '?')
            k = 'struct_N' if re.match(r'struct_\d+$', b) else ('libc' if b in LIBC else b)
            key = ('->field_0x', k)
        elif m2: key = ('->named', '')
        elif m3: key = ('.field', '')
        elif m4:
            nm = m4.group(1); b = d.get(nm, 'glob?' if nm.startswith('dat_') else '?')
            k = 'struct_N' if re.match(r'struct_\d+$', b) else ('libc' if b in LIBC else b)
            key = ('&x[i]', ('G:' if nm.startswith('dat_') else '') + k)
        elif m5:
            nm = m5.group(1)
            key = ('&global' if nm.startswith(('dat_','g_','stru_','qword_','dword_','byte_','unk_','off_','word_')) else '&local', '')
        else: key = ('other', '')
        out.append((key, ty, L.strip()))
    return out
def main():
    for opt, proj, b in CB.corpus(which):
        kp = arm / opt / proj / f'{b}.c'; ip = CB.RES / opt / proj / 'decompiled' / f'ida_{b}.c'
        if not kp.exists() or not ip.exists(): continue
        ks = kp.read_text(errors='replace'); is_ = ip.read_text(errors='replace')
        kv = CC.harvest_types(CC.tokenize(ks), ks); iv = CC.harvest_types(CC.tokenize(is_), is_)
        kf = {a: t for _, a, t in CC.split_functions(ks)}; iff = {a: t for _, a, t in CC.split_functions(is_)}
        for a in set(kf) & set(iff):
            for key, ty, L in classify(kf[a], kv, True):
                cnt[key] += 1
                if len(ex[key]) < 6: ex[key].append(f'{opt}/{b}@{hex(a)}: {L}')
            for key, ty, L in classify(iff[a], iv, False):
                ida_cnt[key[0]] += 1
    tot = sum(cnt.values())
    print('kuna (T *)&... casts on shared set:', tot)
    for k, v in cnt.most_common(): print(f'{v:6d}  {k}')
    print('ida by operand form:', dict(ida_cnt))
    for k in ex:
        print('==', k); [print('   ', e) for e in ex[k]]
if __name__ == "__main__":
    main()
