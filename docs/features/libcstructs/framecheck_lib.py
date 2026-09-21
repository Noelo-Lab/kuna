import re, sys, os, json, collections
SIZES = dict(DIR=1, FILE=216, dirent=280, group=32, lconv=96, mbstate_t=8, obstack=88, option=32,
             passwd=48, pthread_mutex_t=40, re_pattern_buffer=64, sigaction=152, sigset_t=128,
             sockaddr=16, spwd=72, stat=144, statfs=120, termios=60, timespec=16, timeval=16, tm=56,
             utmp=384, utmpx=384)
FN = re.compile(r'^// Function: (\S+) @ (0x[0-9a-f]+)', re.M)
DECL = re.compile(r'^  ([A-Za-z_][\w ]*?\**) ?(v\d+)(?: \[(\d+)\])?;')
def functions(text):
    ms = list(FN.finditer(text))
    for i, m in enumerate(ms):
        end = ms[i+1].start() if i+1 < len(ms) else len(text)
        yield m.group(1), m.group(2), text[m.start():end]
def args_of(s, i):
    depth = 0; n = 0; nonempty = False; j = i
    while j < len(s):
        c = s[j]
        if c in '([{': depth += 1
        elif c in ')]}':
            if depth == 0: return (n + 1 if nonempty else 0), j
            depth -= 1
        elif c == ',' and depth == 0: n += 1
        elif c == '"':
            j += 1
            while j < len(s) and s[j] != '"':
                if s[j] == '\\': j += 1
                j += 1
        if not c.isspace(): nonempty = True
        j += 1
    return None, j
def analyze(text):
    res = {}
    protos = {}
    for name, addr, body in functions(text):
        lines = body.split('\n')
        sig = lines[1] if len(lines) > 1 else ''
        m = re.match(r'^.*?\b' + re.escape(name) + r'\((.*)\)\s*(//.*)?$', sig)
        if m and not name.startswith('sub_'):
            ps = m.group(1).strip()
            void_void = ps in ('', 'void') and sig.startswith('void ')
            if '...' not in ps and not void_void:
                protos[name] = 0 if ps in ('', 'void') else ps.count(',') + 1
        decls = {}
        named = collections.Counter()
        for ln in lines[2:]:
            if ln.strip() == '': break
            d = DECL.match(ln)
            if d:
                t, v, k = d.group(1).strip(), d.group(2), d.group(3)
                decls[v] = (t, int(k) if k else None)
                if t in SIZES and k is None: named[t] += 1
        under = []
        for v, (t, k) in decls.items():
            if k is None or t not in ('char', 'undefined', 'undefined1'): continue
            for cm in re.finditer(r'\((\w+) \*\)(&?)' + v + r'(?:\[(0x[0-9a-f]+|\d+)\])?\b', body):
                T = cm.group(1)
                if T not in SIZES: continue
                off = int(cm.group(3), 0) if cm.group(3) else 0
                if off + SIZES[T] > k: under.append(f"{t} {v}[{k}] as ({T} *)+{off}")
        res[addr] = dict(name=name, named=dict(named), under=sorted(set(under)), body=body)
    return res, protos
def calls(res, protos):
    out = collections.Counter(); where = collections.defaultdict(list)
    if not protos: return out, where
    rx = re.compile(r'(?<![\w.>])(' + '|'.join(sorted(map(re.escape, protos), key=len, reverse=True)) + r')\(')
    for addr, f in res.items():
        b = f['body'].split('\n', 2)[-1]
        for cm in rx.finditer(b):
            name = cm.group(1); n = protos[name]
            k, _ = args_of(b, cm.end())
            if k is not None and k > n:
                out[name] += 1; where[name].append(addr)
    return out, where

SCAL = {'char':1,'bool':1,'uint1':1,'int1':1,'undefined':1,'undefined1':1,'unsigned char':1,
        'short':2,'int2':2,'uint2':2,'undefined2':2,'unsigned short':2,
        'int':4,'int4':4,'uint4':4,'undefined4':4,'unsigned int':4,'float':4,
        'long':8,'int8':8,'uint8':8,'undefined8':8,'unsigned long':8,'double':8}
SDECL = re.compile(r'^  ([A-Za-z_][\w ]*?)( \**)? ?(v\d+)(?: \[(\d+)\])?; // stack - (0x[0-9a-f]+)')
def stack_decls(body):
    out = {}
    for ln in body.split('\n')[2:]:
        if ln.strip() == '': break
        m = SDECL.match(ln)
        if not m: continue
        t, ptr, v, k, off = m.group(1).strip(), (m.group(2) or '').strip(), m.group(3), m.group(4), -int(m.group(5), 16)
        if ptr: el = 8
        elif t in SIZES: el = SIZES[t]
        elif t in SCAL: el = SCAL[t]
        else: el = None
        size = el * int(k) if (el and k) else (el if el else None)
        out[off] = (t + (' ' + ptr if ptr else ''), v, size, k)
    return out
def splits(body_a, body_b):
    a = stack_decls(body_a); b = stack_decls(body_b)
    res = []
    for off, (t, v, size, k) in a.items():
        if not size or size < 16: continue
        inner = sorted(o for o in b if off < o < off + size and o not in a)
        if inner:
            bt = b.get(off)
            res.append(f"{t} {v}{'['+k+']' if k else ''}@{hex(-off)} -> {bt[0] if bt else '?'}{'['+bt[3]+']' if bt and bt[3] else ''} + {len(inner)} new inner")
    return res
