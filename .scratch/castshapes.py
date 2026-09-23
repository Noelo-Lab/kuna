#!/usr/bin/env python3
"""Count cast tokens in decompiler C output and classify their shape.

A cast is `(T)` or `(T *)` immediately followed by an operand start.
Deliberately conservative: the type word must look like a type.
"""
import re, sys, os, collections, json

BASE = r"""(?:
  (?:const\s+|volatile\s+|struct\s+|union\s+|enum\s+|unsigned\s+|signed\s+)*
  (?:long\s+long|long\s+double|unsigned\s+char|unsigned\s+short|unsigned\s+int|unsigned\s+long|
     signed\s+char|short\s+int|long\s+int|long\s+unsigned\s+int|
     [A-Za-z_][A-Za-z0-9_]*)
  (?:\s*\*+)?
  (?:\s*\(\s*\*\s*\)\s*\([^()]*\))?
)"""
CAST_RE = re.compile(r"\((" + BASE + r")\)\s*(?=[A-Za-z_0-9(&*~!\-+])", re.X)

# words that are never types (keywords / very common identifiers)
NOT_TYPE = {
    'if','while','for','return','sizeof','switch','case','else','do','goto',
    'break','continue','default','static','extern','typedef','void_','NULL',
}
TYPEISH = re.compile(r"^(?:const |volatile |struct |union |enum |unsigned |signed )*"
                     r"(?:void|char|short|int|long|float|double|_Bool|bool|"
                     r"undefined\d*|code|int\d+|uint\d+|byte|word|dword|qword|ushort|uint|ulong|"
                     r"struct_\d+|[A-Za-z_][A-Za-z0-9_]*_t|"
                     r"__int\d+|_[A-Z]+|[A-Z][A-Za-z0-9_]*)"
                     r"(?: ?\*+)?$")

def norm(t):
    t = re.sub(r"\s+", " ", t.strip())
    t = re.sub(r"\s*\*", " *", t)
    t = re.sub(r"\* \*", "**", t)
    return t

def casts(text):
    out = []
    for m in CAST_RE.finditer(text):
        raw = norm(m.group(1))
        head = raw.split()[0].rstrip('*')
        if head in NOT_TYPE:
            continue
        if not TYPEISH.match(raw):
            continue
        # skip `(x)(...)` call-through-pointer: handled by requiring operand
        out.append((raw, m.start()))
    return out

def ctx_line(text, pos):
    s = text.rfind('\n', 0, pos) + 1
    e = text.find('\n', pos)
    return text[s:e if e > 0 else len(text)].strip()

def main():
    files = sys.argv[1:]
    per_tool = collections.defaultdict(lambda: {'casts': 0, 'lines': 0,
                                                'types': collections.Counter(),
                                                'shapes': collections.Counter()})
    examples = collections.defaultdict(list)
    for f in files:
        base = os.path.basename(f)
        tool = base.split('_')[0]
        text = open(f, errors='replace').read()
        nl = text.count('\n') + 1
        cs = casts(text)
        d = per_tool[tool]
        d['casts'] += len(cs)
        d['lines'] += nl
        for t, p in cs:
            d['types'][t] += 1
            line = ctx_line(text, p)
            sh = shape(t, line, p, text)
            d['shapes'][sh] += 1
            if len(examples[(tool, sh)]) < 6:
                examples[(tool, sh)].append((base, line))
    res = {}
    for tool, d in sorted(per_tool.items()):
        res[tool] = {
            'casts': d['casts'], 'lines': d['lines'],
            'per_kloc': round(1000.0 * d['casts'] / max(d['lines'], 1), 1),
            'top_types': d['types'].most_common(25),
            'top_shapes': d['shapes'].most_common(20),
        }
    print(json.dumps(res, indent=1))
    if os.environ.get('SHOW_EX'):
        for k in sorted(examples):
            if k[0] != os.environ.get('SHOW_EX'):
                continue
            print('\n##', k[1])
            for b, l in examples[k]:
                print('   ', b, '|', l[:170])

def shape(t, line, pos, text):
    """Classify the syntactic context of the cast."""
    ptr = '*' in t
    after = text[pos:pos+400]
    # what immediately follows the cast
    m = CAST_RE.match(after)
    rest = after[m.end():] if m else ''
    tag = []
    if ptr:
        tag.append('ptr')
        if re.match(r"^&", rest):
            tag.append('addr-of')
        elif re.match(r"^\(?[A-Za-z_]\w*\s*\+", rest):
            tag.append('base+off')
        else:
            tag.append('plain')
        # is it dereferenced right away: *(T *)x
        if pos > 0 and text[pos-1] == '*':
            tag = ['deref-ptr'] + tag[1:]
    else:
        tag.append('scalar:' + t)
        if re.match(r"^0x[0-9a-fA-F]+|^\d", rest):
            tag.append('const')
        elif re.match(r"^\(", rest):
            tag.append('paren-expr')
        else:
            tag.append('var')
    return ' '.join(tag)

if __name__ == '__main__':
    main()
