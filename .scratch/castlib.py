import re, os, sys, json, collections

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"

# ---- function splitting -------------------------------------------------
HDR = re.compile(r'^// Function:\s*(\S+)\s*@\s*(0x[0-9a-fA-F]+)\s*$', re.M)

def split_funcs(text):
    """-> list of (name, addr_int, body_text)"""
    out = []
    ms = list(HDR.finditer(text))
    for i, m in enumerate(ms):
        start = m.end()
        end = ms[i+1].start() if i+1 < len(ms) else len(text)
        out.append((m.group(1), int(m.group(2), 16), text[start:end]))
    return out

# ---- cast detection -----------------------------------------------------
BASE = r'''(?:
   _BOOL[1248] | _BYTE | _WORD | _DWORD | _QWORD | _OWORD | _TBYTE | _UNKNOWN
 | __int8 | __int16 | __int32 | __int64 | __int128
 | unsigned\s+__int8 | unsigned\s+__int16 | unsigned\s+__int32 | unsigned\s+__int64 | unsigned\s+__int128
 | unsigned\s+long\s+long | unsigned\s+long\s+int | unsigned\s+long | unsigned\s+short | unsigned\s+char | unsigned\s+int | unsigned
 | signed\s+long\s+long | signed\s+long | signed\s+short | signed\s+char | signed\s+int | signed
 | long\s+long\s+int | long\s+long | long\s+double | long\s+int | long
 | short\s+int | short | char | int | bool | void | float | double | size_t | ssize_t
 | uint8_t | uint16_t | uint32_t | uint64_t | int8_t | int16_t | int32_t | int64_t
 | uint1 | uint2 | uint4 | uint8 | int1 | int2 | int4 | int8 | undefined[1248]? | code | byte | word | dword | qword | ulong | uint | ushort | uchar
 | FILE | va_list | __va_list_tag | jmp_buf | wchar_t | time_t | mode_t | pid_t | off_t | intptr_t | uintptr_t | ptrdiff_t
 | struct\s+\w+ | union\s+\w+ | enum\s+\w+
 | struct_\w+ | union_\w+ | astruct\w* | \w+_t | [A-Z][A-Za-z0-9_]*
)'''

CAST = re.compile(r'\(\s*(?:const\s+|volatile\s+)*(' + BASE + r')((?:\s*\*)*)\s*(?:\[\s*\d*\s*\])?\s*\)\s*(?=[A-Za-z_0-9(&*~!\-+."\']|\+\+|--)', re.X)

# things that look like a cast but are a call or a parenthesised expr
NOTCAST_NEXT = re.compile(r'^(?:\s*\))')

KEYWORDS = {'if','while','for','switch','return','sizeof','else','do','case','goto','break','continue'}

def find_casts(body, known_types=None):
    """Return list of (span_start, type_text, following_snippet)."""
    res = []
    for m in CAST.finditer(body):
        ty = m.group(1)
        stars = m.group(2).replace(' ', '')
        # reject when the "type" is really a variable/function name used in
        # a parenthesised expression, e.g. (v1)->x   or  (x) * y
        if ty in KEYWORDS:
            continue
        if not stars:
            # a bare (Name)foo where Name is not a builtin: require it in known_types
            if known_types is not None and ty not in known_types and not _builtin(ty):
                continue
            if _looks_like_local(ty):
                continue
        else:
            if known_types is not None and not _builtin(ty) and ty not in known_types:
                # (v1 *) is impossible; require plausibility
                if _looks_like_local(ty):
                    continue
        # the char right before '(' must not be an identifier char (that'd be a call)
        i = m.start()
        j = i - 1
        while j >= 0 and body[j] in ' \t':
            j -= 1
        if j >= 0 and (body[j].isalnum() or body[j] == '_'):
            # e.g.  foo (int)  -- unlikely; but  sizeof(int) handled below
            prev_word = re.search(r'([A-Za-z_]\w*)\s*$', body[:i])
            if prev_word and prev_word.group(1) not in ('return','sizeof','case'):
                continue
            if prev_word and prev_word.group(1) == 'sizeof':
                continue
        res.append((m.start(), (ty + ' ' + stars).strip(), body[m.end():m.end()+70]))
    return res

_BUILTIN_RE = re.compile(r'^(?:const\s+)?(?:unsigned|signed|long|short|char|int|void|bool|float|double|_BOOL[1248]|_BYTE|_WORD|_DWORD|_QWORD|_OWORD|_TBYTE|__int\d+|size_t|ssize_t|u?int\d+_t|u?int[1248]|undefined\d?|byte|word|dword|qword|ulong|uint|ushort|uchar|code|FILE|wchar_t|time_t|mode_t|pid_t|off_t|u?intptr_t|ptrdiff_t|va_list|__va_list_tag)\b', re.X)

def _builtin(ty):
    return bool(_BUILTIN_RE.match(ty))

_LOCAL_RE = re.compile(r'^(?:v\d+|a\d+|s\d+|i|j|k|n|x|y|p|q|result|dest|src|s|d)$')
def _looks_like_local(ty):
    return bool(_LOCAL_RE.match(ty))

def declared_types(text):
    """type names that appear as declarations in the file (struct/typedef/vars)"""
    s = set()
    for m in re.finditer(r'\b(?:struct|union|enum)\s+(\w+)', text):
        s.add('struct ' + m.group(1)); s.add('union ' + m.group(1)); s.add('enum ' + m.group(1)); s.add(m.group(1))
    for m in re.finditer(r'^\s*typedef\s+.*?\b(\w+)\s*;', text, re.M):
        s.add(m.group(1))
    for m in re.finditer(r'\b(struct_\w+|astruct\w*|\w+_t)\b', text):
        s.add(m.group(1))
    return s

def load(opt, proj, dec, binname):
    p = os.path.join(R, opt, proj, "decompiled", f"{dec}_{binname}.c")
    if not os.path.exists(p):
        return None
    return open(p, errors='replace').read()
