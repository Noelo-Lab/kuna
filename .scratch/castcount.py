#!/usr/bin/env python3
"""castcount.py -- count C casts in decompiler output by SHAPE, not by regex.

WHY NOT A REGEX
    A regex over `\\([a-z_ *]+\\)` miscounts four different things as casts:
    a parenthesized variable `(a1)`, a parameter declaration `(long long a1)`,
    a `sizeof(int)` operand, and the `(int)` half of a function-pointer
    declarator `void (*fp)(int)`.  And it misses `(void (*)(long))p`.  This
    counter tokenizes and applies the C cast grammar instead.

WHAT COUNTS AS A CAST
    A `(` ... `)` span counts as exactly one cast when ALL THREE hold:

    (1) CONTENT parses as a C *type-name*: a specifier-qualifier-list
        (`void char short int long float double signed unsigned _Bool bool
        const volatile _Complex`, or `struct|union|enum TAG`, or a name in the
        harvested typedef vocabulary -- see below) followed by an OPTIONAL
        ABSTRACT declarator: `*`s with optional qualifiers, `[N]` array
        suffixes, and the function-pointer form `(*)(...)` / `(**)(...)`.
        An abstract declarator carries NO identifier, which is what rejects
        `(long long a1)` and `(int argc, char **argv)`.

    (2) The token BEFORE the `(` permits a cast to start there.  A cast may
        not follow an identifier, a constant, a string, `)`, `]`, `++` or `--`
        (in those positions `(` opens a call or is a syntax error), and may
        not follow `sizeof`, `alignof`, `_Alignof`, `__alignof__`,
        `offsetof`, `typeof`, `__typeof__`, `va_arg`, `__builtin_va_arg`.

    (3) The token AFTER the `)` begins a unary-expression: an identifier, a
        constant, a string, `(`, or one of `* & - + ~ ! ++ --`.  A `{` after
        the `)` means a COMPOUND LITERAL `(struct S){...}`, not a cast, and is
        excluded here.  `;` `,` `)` `]` after it mean the parens held a
        declarator or a stray group.

    Nested casts each count once: `(char *)(long)x` is 2.

TYPEDEF VOCABULARY (the C typedef/identifier ambiguity)
    `(FILE *)p` is a cast only if FILE is a type.  Each tool has its own
    vocabulary (IDA `_QWORD/__int64`, Ghidra `undefined8/ulong/code`, kuna
    `uint/ulong/code`), so the vocabulary is HARVESTED PER FILE, tool-agnostic,
    from four declaration positions in that same file:
      - `typedef ... NAME;` and `typedef ... (*NAME)(...);`
      - `struct|union|enum TAG {` tags
      - local/global declarations `TYPE [*...] name ;` at statement start
      - parameter declarations `TYPE [*...] name` inside a function signature
    plus a small fixed seed set of types every one of these tools emits.
    Harvesting means an identifier only becomes a type because THIS FILE used
    it as one, so `(v5)` and `(ptr)` stay non-casts.

OPERAND SHAPE
    Each counted cast also records the shape of the operand it applies to,
    for the shape census: <call>, <const>, <str>, <deref> (`*`), <addr> (`&`),
    <var>, <field> (var followed by `.`/`->`), <paren>, <neg>, <expr>.

TYPE NORMALIZATION
    So an IDA `(_QWORD *)` and a kuna `(unsigned long *)` compare as the same
    shape, every type spelling is mapped to a canonical name:
    u8/i8/u16/i16/u32/i32/u64/i64/f32/f64/void/bool/char/uchar/code/undef/
    struct:<tag>/named:<name>, with the pointer depth kept as a `*` suffix.

STATEMENTS
    Statements are counted as `;` tokens that are NOT inside a `for (...)`
    header, plus `{`-less... no: plus nothing.  Declarations count as
    statements (every tool emits them), which is the same rule for all tools.

USAGE
    castcount.py FILE...              per-file JSON summary
    castcount.py --shapes FILE...     add the cast-shape histogram
    castcount.py --per-function F     per-function rows (needs `// Function: n @ 0xA`)
    castcount.py --dump FILE          print every counted cast with its line
    castcount.py --audit FILE N       print every `( ... )` span in the first N
                                      lines with the verdict and the reason,
                                      for hand-checking the miscount rate
"""
import sys, re, json, collections

# ---------------------------------------------------------------- tokenizer

TOKEN_RE = re.compile(r"""
    (?P<ws>\s+)
  | (?P<lcomment>//[^\n]*)
  | (?P<bcomment>/\*.*?\*/)
  | (?P<str>"(?:\\.|[^"\\])*")
  | (?P<chr>'(?:\\.|[^'\\])*')
  | (?P<num>(?:0[xX][0-9a-fA-F]+|(?:\d+\.\d*|\.\d+|\d+)(?:[eE][-+]?\d+)?)[uUlLfF]*)
  | (?P<id>[A-Za-z_$][A-Za-z_0-9$]*)
  | (?P<punct><<=|>>=|\.\.\.|->|\+\+|--|<<|>>|<=|>=|==|!=|&&|\|\||[-+*/%&|^]=|.)
""", re.VERBOSE | re.DOTALL)


class Tok:
    __slots__ = ("kind", "text", "line")

    def __init__(self, kind, text, line):
        self.kind, self.text, self.line = kind, text, line

    def __repr__(self):
        return f"{self.kind}:{self.text}"


def tokenize(src):
    toks, line = [], 1
    for m in TOKEN_RE.finditer(src):
        kind = m.lastgroup
        text = m.group()
        if kind in ("ws", "lcomment", "bcomment"):
            line += text.count("\n")
            continue
        toks.append(Tok(kind, text, line))
        line += text.count("\n")
    return toks


# ---------------------------------------------------------------- vocabulary

BASE = {"void", "char", "short", "int", "long", "float", "double", "signed",
        "unsigned", "_Bool", "bool", "_Complex"}
QUAL = {"const", "volatile", "restrict", "__restrict", "_Atomic", "register"}
TAGKW = {"struct", "union", "enum"}
# Types every one of ida/ghidra/kuna/angr emits, seeded so a file that never
# declares a local of that type still recognizes a cast to it.
SEED = {
    # ida
    "_BYTE", "_WORD", "_DWORD", "_QWORD", "_OWORD", "_TBYTE", "_UNKNOWN",
    "__int8", "__int16", "__int32", "__int64", "__int128", "_LONGLONG",
    "_ULONGLONG", "__m128", "__m128i", "__m128d",
    # ghidra
    "undefined", "undefined1", "undefined2", "undefined3", "undefined4",
    "undefined5", "undefined6", "undefined7", "undefined8", "undefined16",
    "byte", "word", "dword", "qword", "uint", "ushort", "ulong", "ulonglong",
    "longlong", "code", "uint3", "uint5", "uint6", "uint7", "int3", "int5",
    "int6", "int7", "unkbyte9", "unkuint9",
    # kuna console/CLI spellings
    "int1", "int2", "int4", "int8", "int16", "uint1", "uint2", "uint4",
    "uint8", "uint16", "float4", "float8", "float10", "bool1",
    # common libc / angr
    "FILE", "size_t", "ssize_t", "ptrdiff_t", "wchar_t", "intptr_t",
    "uintptr_t", "int8_t", "int16_t", "int32_t", "int64_t", "uint8_t",
    "uint16_t", "uint32_t", "uint64_t", "va_list", "__va_list_tag",
    "time_t", "off_t", "mode_t", "pid_t", "uid_t", "gid_t", "dev_t", "ino_t",
    "nlink_t", "blksize_t", "blkcnt_t", "socklen_t", "sig_atomic_t",
    "DIR", "jmp_buf", "sigjmp_buf", "__jmp_buf_tag",
}

DECL_START_RE = re.compile(r"^[A-Za-z_]")


def harvest_types(toks, src):
    """Collect the identifiers this file uses as type names."""
    vocab = set(SEED)

    # struct/union/enum tags -> also used bare in some outputs
    for i, t in enumerate(toks):
        if t.kind == "id" and t.text in TAGKW and i + 1 < len(toks) \
                and toks[i + 1].kind == "id":
            vocab.add(toks[i + 1].text)

    # typedef NAME;  /  typedef ... (*NAME)(...);
    i = 0
    while i < len(toks):
        if toks[i].kind == "id" and toks[i].text == "typedef":
            depth, j, last_id = 0, i + 1, None
            while j < len(toks):
                tt = toks[j].text
                if tt in "([":
                    depth += 1
                elif tt in ")]":
                    depth -= 1
                elif tt == ";" and depth <= 0:
                    break
                elif tt == "{":
                    d2 = 1
                    j += 1
                    while j < len(toks) and d2:
                        if toks[j].text == "{":
                            d2 += 1
                        elif toks[j].text == "}":
                            d2 -= 1
                        j += 1
                    continue
                if toks[j].kind == "id" and toks[j].text not in BASE \
                        and toks[j].text not in QUAL and toks[j].text not in TAGKW:
                    last_id = toks[j].text
                j += 1
            if last_id:
                vocab.add(last_id)
            i = j
        i += 1

    # Declaration position: at a statement start (after ; { } or file start),
    # `IDENT [const] *... IDENT [ [..] ] (; | = | , )`  -> first IDENT is a type.
    # Also the parameter form inside a signature: after ( or , at paren depth 1.
    n = len(toks)
    for i in range(n):
        t = toks[i]
        if t.kind != "id" or t.text in BASE or t.text in QUAL or t.text in TAGKW:
            continue
        if i > 0:
            p = toks[i - 1].text
            if p not in (";", "{", "}", "(", ",", ")"):
                continue
        j = i + 1
        while j < n and (toks[j].text == "*" or toks[j].text in QUAL):
            j += 1
        if j >= n or toks[j].kind != "id":
            continue
        if toks[j].text in BASE or toks[j].text in QUAL:
            continue
        k = j + 1
        while k < n and toks[k].text == "[":
            d = 1
            k += 1
            while k < n and d:
                if toks[k].text == "[":
                    d += 1
                elif toks[k].text == "]":
                    d -= 1
                k += 1
        if k < n and toks[k].text in (";", "=", ",", ")"):
            # `foo (bar)` would be a call; require a * or a following ; = , )
            vocab.add(t.text)
    return vocab


# ------------------------------------------------------- type-name grammar

def parse_type_name(toks, lo, hi, vocab):
    """toks[lo:hi] is the inside of a (...).  Return normalized type or None."""
    i = lo
    specs, tag = [], None
    saw_spec = False
    while i < hi:
        t = toks[i]
        if t.kind != "id":
            break
        if t.text in QUAL:
            i += 1
            continue
        if t.text in TAGKW:
            if i + 1 < hi and toks[i + 1].kind == "id":
                tag = (t.text, toks[i + 1].text)
                i += 2
                saw_spec = True
                continue
            return None
        if t.text in BASE:
            specs.append(t.text)
            saw_spec = True
            i += 1
            continue
        if not saw_spec and t.text in vocab:
            specs.append(t.text)
            saw_spec = True
            i += 1
            # a typedef name is a complete specifier; another bare id after it
            # means a declarator name -> not an abstract declarator
            continue
        break
    if not saw_spec:
        return None

    # abstract declarator
    stars = 0
    while i < hi:
        t = toks[i]
        if t.text == "*":
            stars += 1
            i += 1
        elif t.kind == "id" and t.text in QUAL:
            i += 1
        else:
            break

    fnptr = False
    if i < hi and toks[i].text == "(":
        # function-pointer abstract declarator: ( * [*...] ) ( params )
        d, j = 1, i + 1
        inner_ok = True
        k = i + 1
        while k < hi and toks[k].text == "*":
            k += 1
        if k >= hi or toks[k].text != ")":
            inner_ok = False
        if inner_ok:
            j = k + 1
            if j < hi and toks[j].text == "(":
                d = 1
                j += 1
                while j < hi and d:
                    if toks[j].text == "(":
                        d += 1
                    elif toks[j].text == ")":
                        d -= 1
                    j += 1
                fnptr = True
                i = j
            else:
                return None
        else:
            return None

    while i < hi and toks[i].text == "[":
        d = 1
        i += 1
        while i < hi and d:
            if toks[i].text == "[":
                d += 1
            elif toks[i].text == "]":
                d -= 1
            i += 1
        stars += 1  # array-of decays to pointer for shape purposes

    if i != hi:
        return None
    return normalize(specs, tag, stars, fnptr)


CANON = {
    ("void",): "void", ("char",): "char", ("signed", "char"): "i8",
    ("unsigned", "char"): "uchar", ("short",): "i16",
    ("signed", "short"): "i16", ("short", "int"): "i16",
    ("unsigned", "short"): "u16", ("unsigned", "short", "int"): "u16",
    ("int",): "i32", ("signed",): "i32", ("signed", "int"): "i32",
    ("unsigned",): "u32", ("unsigned", "int"): "u32",
    ("long",): "i64", ("long", "int"): "i64", ("signed", "long"): "i64",
    ("unsigned", "long"): "u64", ("unsigned", "long", "int"): "u64",
    ("long", "long"): "i64", ("long", "long", "int"): "i64",
    ("signed", "long", "long"): "i64",
    ("unsigned", "long", "long"): "u64",
    ("unsigned", "long", "long", "int"): "u64",
    ("float",): "f32", ("double",): "f64", ("long", "double"): "f80",
    ("_Bool",): "bool", ("bool",): "bool",
}
TYPEDEF_CANON = {
    "_BYTE": "u8", "byte": "u8", "undefined1": "u8", "uint1": "u8",
    "uint8_t": "u8", "__int8": "i8", "int1": "i8", "int8_t": "i8",
    "bool1": "bool",
    "_WORD": "u16", "word": "u16", "undefined2": "u16", "uint2": "u16",
    "uint16_t": "u16", "ushort": "u16", "__int16": "i16", "int2": "i16",
    "int16_t": "i16", "short": "i16",
    "_DWORD": "u32", "dword": "u32", "undefined4": "u32", "uint4": "u32",
    "uint32_t": "u32", "uint": "u32", "__int32": "i32", "int4": "i32",
    "int32_t": "i32",
    "_QWORD": "u64", "qword": "u64", "undefined8": "u64", "uint8": "u64",
    "uint64_t": "u64", "ulong": "u64", "ulonglong": "u64", "size_t": "u64",
    "__int64": "i64", "int8": "i64", "int64_t": "i64", "longlong": "i64",
    "ssize_t": "i64", "intptr_t": "i64", "uintptr_t": "u64", "off_t": "i64",
    "ptrdiff_t": "i64",
    "_OWORD": "u128", "__int128": "i128", "undefined16": "u128",
    "uint16": "u128", "int16": "i128",
    "float4": "f32", "float8": "f64", "float10": "f80",
    "undefined": "undef", "_UNKNOWN": "undef",
    "code": "code",
}


def normalize(specs, tag, stars, fnptr):
    if tag:
        base = f"{tag[0]}:{tag[1]}"
    elif specs:
        key = tuple(specs)
        if key in CANON:
            base = CANON[key]
        elif len(specs) == 1 and specs[0] in TYPEDEF_CANON:
            base = TYPEDEF_CANON[specs[0]]
        elif len(specs) == 1:
            base = "named:" + specs[0]
        else:
            base = "named:" + " ".join(specs)
    else:
        return None
    if fnptr:
        return "fnptr" + "*" * max(stars, 0)
    return base + ("*" * stars)


# ------------------------------------------------------------ cast scanner

NO_CAST_BEFORE_KIND = {"num", "str", "chr"}
NO_CAST_BEFORE_TEXT = {"]", "++", "--"}
TYPE_OPERATORS = {"sizeof", "alignof", "_Alignof", "__alignof__", "__alignof",
                  "offsetof", "__offsetof", "typeof", "__typeof__", "__typeof",
                  "va_arg", "__builtin_va_arg", "__builtin_offsetof",
                  "_Generic", "__builtin_types_compatible_p"}
# The only identifiers that may precede a cast: keywords that introduce an
# expression.  Any other identifier before `(` opens a CALL.
CAST_OK_AFTER_ID = {"return", "case", "else", "do", "if", "while", "for",
                    "switch", "goto"}
UNARY_START_PUNCT = {"*", "&", "-", "+", "~", "!", "(", "++", "--"}


def cast_allowed_before(toks, i, cast_close):
    """May a cast start at toks[i] == '('?  cast_close: `)` indices that closed
    a cast already counted (so `(long)(int)x` counts twice)."""
    if i == 0:
        return True
    p = toks[i - 1]
    if p.kind in NO_CAST_BEFORE_KIND:
        return False
    if p.kind == "id":
        # a type keyword before `(` means a declarator: `long (*fp)(void)`
        if p.text in BASE or p.text in QUAL or p.text in TAGKW:
            return False
        if p.text in TYPE_OPERATORS:
            return False
        return p.text in CAST_OK_AFTER_ID
    if p.text == ")":
        return (i - 1) in cast_close
    if p.text in NO_CAST_BEFORE_TEXT:
        return False
    return True


def operand_shape(toks, k, n):
    """Shape of the expression starting at toks[k] (just past the `)`)."""
    if k >= n:
        return "<none>"
    t = toks[k]
    if t.text == "*":
        return "<deref>"
    if t.text == "&":
        return "<addr>"
    if t.text in ("-", "+", "~", "!"):
        return "<neg>"
    if t.text == "(":
        return "<paren>"
    if t.kind == "num":
        return "<const>"
    if t.kind in ("str", "chr"):
        return "<str>"
    if t.kind == "id":
        if k + 1 < n:
            nx = toks[k + 1].text
            if nx == "(":
                return "<call>"
            if nx in (".", "->"):
                return "<field>"
            if nx == "[":
                return "<index>"
            # a binary operator right after means the cast binds the var only
        return "<var>"
    return "<expr>"


def matching(toks):
    """index of the matching ) for each ( (and vice versa)."""
    stack, match = [], {}
    for i, t in enumerate(toks):
        if t.text == "(":
            stack.append(i)
        elif t.text == ")" and stack:
            j = stack.pop()
            match[j] = i
    return match


def find_casts(toks, vocab):
    n = len(toks)
    match = matching(toks)
    out, cast_close = [], set()
    for i in range(n):  # ascending open index: an outer cast is seen first
        if toks[i].text != "(":
            continue
        j = match.get(i)
        if j is None or j == i + 1:
            continue
        if not cast_allowed_before(toks, i, cast_close):
            continue
        # (3) successor
        k = j + 1
        if k >= n:
            continue
        nt = toks[k]
        if not (nt.kind in ("id", "num", "str", "chr")
                or nt.text in UNARY_START_PUNCT):
            continue
        # (1) content
        ty = parse_type_name(toks, i + 1, j, vocab)
        if ty is None:
            continue
        cast_close.add(j)
        out.append((ty, operand_shape(toks, k, n), toks[i].line,
                    "".join(t.text for t in toks[i:j + 1])))
    return out


def count_statements(toks):
    """`;` outside a for(...) header."""
    n, stmts = len(toks), 0
    match = matching(toks)
    skip = set()
    for i, t in enumerate(toks):
        if t.kind == "id" and t.text == "for" and i + 1 < n and toks[i + 1].text == "(":
            j = match.get(i + 1)
            if j:
                for m in range(i + 1, j):
                    skip.add(m)
    for i, t in enumerate(toks):
        if t.text == ";" and i not in skip:
            stmts += 1
    return stmts


FUNC_HDR = re.compile(r"^//\s*Function:\s*(\S+)\s*@\s*(0x[0-9a-fA-F]+)", re.M)


def split_functions(src):
    """[(name, addr, text)] using the decbench `// Function: n @ 0xA` banner."""
    hits = list(FUNC_HDR.finditer(src))
    if not hits:
        return []
    out = []
    for a, m in enumerate(hits):
        end = hits[a + 1].start() if a + 1 < len(hits) else len(src)
        out.append((m.group(1), int(m.group(2), 16), src[m.start():end]))
    return out


def analyze(src):
    toks = tokenize(src)
    vocab = harvest_types(toks, src)
    casts = find_casts(toks, vocab)
    lines = src.count("\n") + 1
    # code lines: non-blank, non-comment-only
    code = sum(1 for L in src.splitlines()
               if L.strip() and not L.strip().startswith("//"))
    return {
        "lines": lines, "code_lines": code, "statements": count_statements(toks),
        "tokens": len(toks), "casts": len(casts), "cast_list": casts,
        "vocab_size": len(vocab),
    }


def summarize(path, shapes=False):
    src = open(path, errors="replace").read()
    a = analyze(src)
    d = {"file": path, "lines": a["lines"], "code_lines": a["code_lines"],
         "statements": a["statements"], "casts": a["casts"],
         "casts_per_kloc": round(1000.0 * a["casts"] / max(a["code_lines"], 1), 2),
         "casts_per_100stmt": round(100.0 * a["casts"] / max(a["statements"], 1), 2)}
    if shapes:
        h = collections.Counter((t, s) for t, s, _, _ in a["cast_list"])
        d["shapes"] = [{"type": t, "operand": s, "n": n}
                       for (t, s), n in h.most_common()]
    return d


def per_function(path):
    src = open(path, errors="replace").read()
    rows = []
    for name, addr, text in split_functions(src):
        a = analyze(text)
        rows.append({"name": name, "addr": addr, "code_lines": a["code_lines"],
                     "statements": a["statements"], "casts": a["casts"],
                     "shapes": collections.Counter(
                         (t, s) for t, s, _, _ in a["cast_list"])})
    return rows


def audit(path, nlines, lo=None, hi=None):
    """Every `(...)` span with a verdict + reason.  With lo/hi the WHOLE file is
    tokenized (correct vocabulary and paren matching) but only spans opening in
    [lo,hi] are printed -- that is the hand-check window."""
    if lo is not None:
        src = open(path, errors="replace").read()
    else:
        src = "\n".join(open(path, errors="replace").read().splitlines()[:nlines])
    toks = tokenize(src)
    vocab = harvest_types(toks, src)
    match = matching(toks)
    n = len(toks)
    cast_close = set()
    for i in range(n):
        if toks[i].text != "(":
            continue
        j = match.get(i)
        if j is None:
            continue
        if lo is not None and not (lo <= toks[i].line <= hi):
            if parse_type_name(toks, i + 1, j, vocab) is not None and \
                    cast_allowed_before(toks, i, cast_close) and j + 1 < n and \
                    (toks[j+1].kind in ("id","num","str","chr") or toks[j+1].text in UNARY_START_PUNCT):
                cast_close.add(j)
            continue
        inner = "".join(t.text for t in toks[i:j + 1])
        if len(inner) > 60:
            inner = inner[:57] + "..."
        reasons = []
        if not cast_allowed_before(toks, i, cast_close):
            reasons.append(f"pred={toks[i-1].text!r}" if i else "pred=BOF")
        k = j + 1
        if k < n:
            nt = toks[k]
            if not (nt.kind in ("id", "num", "str", "chr")
                    or nt.text in UNARY_START_PUNCT):
                reasons.append(f"succ={nt.text!r}")
        else:
            reasons.append("succ=EOF")
        ty = parse_type_name(toks, i + 1, j, vocab)
        if ty is None:
            reasons.append("not-a-type-name")
        verdict = "CAST" if not reasons else "no"
        if verdict == "CAST":
            cast_close.add(j)
        nx = toks[k].text if k < n else "EOF"
        print(f"L{toks[i].line:<5} {verdict:<4} {inner:<62} next={nx!r:<12} "
              f"{';'.join(reasons)}")


if __name__ == "__main__":
    args = sys.argv[1:]
    if args and args[0] == "--audit":
        audit(args[1], int(args[2]) if len(args) > 2 else 100)
    elif args and args[0] == "--auditrange":
        audit(args[1], 0, int(args[2]), int(args[3]))
    elif args and args[0] == "--dump":
        src = open(args[1], errors="replace").read()
        toks = tokenize(src)
        vocab = harvest_types(toks, src)
        for ty, sh, ln, txt in find_casts(toks, vocab):
            print(f"L{ln:<6} {ty:<18} {sh:<10} {txt}")
    elif args and args[0] == "--per-function":
        for r in per_function(args[1]):
            r["shapes"] = [[f"({t}){s}", n] for (t, s), n in r["shapes"].most_common()]
            print(json.dumps(r))
    else:
        shapes = "--shapes" in args
        files = [a for a in args if not a.startswith("--")]
        print(json.dumps([summarize(f, shapes) for f in files], indent=1))
