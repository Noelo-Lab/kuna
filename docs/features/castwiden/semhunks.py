"""Classify every castwiden hunk by what the C means, not by its text.

For each function whose text differs between two decompile-all trees (castwiden
off, then the default), every changed line is parsed as C (casts found with
castcount.py's cast grammar and the file's own type vocabulary) and the two trees
are walked side by side:

- GROUPING: the trees differ once casts are set aside (and literal suffixes
  ignored), so a removed cast changed which operands an operator takes.
- BOTH_LOST: a binary operator both of whose operands carried a cast before and
  neither does now (one widened value read twice, printed as a 32-bit op).
- SCOPE: a removed cast that is not a cast to an 8-byte integer.
- FORBIDDEN: a cast removed from a shift, a comparison or a unary operand.
- NARROW: the operand that lost its cast now meets an operand, or a destination,
  whose C type (from the function's own declarations) is narrower than 8 bytes or
  does not convert it to the cast's type: the value would change.
- ADDED: the new text carries a cast the old did not.
- LINECOUNT / UNPARSED: the function's line structure changed, or a changed line
  did not parse; listed for reading.

Every other removed cast is counted by context: the operator it sat under
(`arith:+`), `assign`, `store`, `call-arg`, `return`, `index` or `other`, and each
literal that gained a suffix as `suffix`.  Each is also `verified` when the C type
of what it meets is known from the declarations (a declared variable, a
dereference of a declared or cast pointer, a literal, a call to a function the
same file defines) and C's conversion then yields the cast's type, or
`unverified` when that type is not known (a structure field, an undeclared
global, a library call); the unverified ones are listed per context for reading.

  python3 semhunks.py OLD_DIR NEW_DIR [--json OUT]
"""
import collections, json, sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castcount as CC  # noqa: E402

WIDE = {"i64", "u64"}
ARITH = {"+", "-", "*", "/", "%", "&", "|", "^"}
SHIFT_CMP = {"<<", ">>", "<", ">", "<=", ">=", "==", "!="}
ASSIGN = {"=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|="}
BIN = {"||": 4, "&&": 5, "|": 6, "^": 7, "&": 8, "==": 9, "!=": 9, "<": 10, ">": 10,
       "<=": 10, ">=": 10, "<<": 11, ">>": 11, "+": 12, "-": 12, "*": 13, "/": 13, "%": 13}
PREFIX = {"-", "+", "!", "~", "*", "&", "++", "--"}


class ParseError(Exception):
    pass


class P:
    def __init__(self, toks, vocab):
        self.t, self.i, self.v = toks, 0, vocab
        self.match = CC.matching(toks)

    def peek(self, k=0):
        j = self.i + k
        return self.t[j] if j < len(self.t) else None

    def eat(self, text=None):
        tok = self.peek()
        if tok is None or (text is not None and tok.text != text):
            raise ParseError(f"want {text} got {tok}")
        self.i += 1
        return tok

    def expr(self, minp=0):
        left = self.unary()
        while True:
            tok = self.peek()
            if tok is None:
                return left
            op = tok.text
            if op in ASSIGN and minp <= 2:
                self.i += 1
                left = ("asg", op, left, self.expr(2))
                continue
            if op == "?" and minp <= 3:
                self.i += 1
                a = self.expr(0)
                self.eat(":")
                left = ("tern", left, a, self.expr(3))
                continue
            p = BIN.get(op)
            if p is None or p < minp or tok.kind != "punct":
                return left
            self.i += 1
            left = ("bin", op, left, self.expr(p + 1))

    def cast_at(self):
        tok = self.peek()
        if tok is None or tok.text != "(":
            return None
        j = self.match.get(self.i)
        if j is None or j == self.i + 1 or j + 1 >= len(self.t):
            return None
        nt = self.t[j + 1]
        if not (nt.kind in ("id", "num", "str", "chr") or nt.text in CC.UNARY_START_PUNCT):
            return None
        ty = CC.parse_type_name(self.t, self.i + 1, j, self.v)
        if ty is None:
            ty = array_pointer(self.t[self.i + 1:j], self.v)
        if ty is None:
            return None
        return ty, j

    def unary(self):
        tok = self.peek()
        if tok is None:
            raise ParseError("eof")
        c = self.cast_at()
        if c is not None:
            ty, j = c
            self.i = j + 1
            return ("cast", ty, self.unary())
        if tok.kind == "punct" and tok.text in PREFIX:
            self.i += 1
            return ("un", tok.text, self.unary())
        if tok.kind == "id" and tok.text == "sizeof":
            self.i += 1
            if self.peek() and self.peek().text == "(":
                j = self.match[self.i]
                txt = " ".join(x.text for x in self.t[self.i:j + 1])
                self.i = j + 1
                return ("sizeof", txt)
            return ("un", "sizeof", self.unary())
        return self.postfix(self.primary())

    def primary(self):
        tok = self.eat()
        if tok.kind == "id" and tok.text in ("L", "u", "U") and self.peek() is not None \
                and self.peek().kind in ("chr", "str"):
            tok = self.eat()
        if tok.kind in ("id", "num", "str", "chr"):
            if tok.kind == "str":
                s = tok.text
                while self.peek() is not None and self.peek().kind == "str":
                    s += self.eat().text
                return ("str", s)
            if tok.kind == "num":
                return ("num", tok.text)
            if tok.kind == "chr":
                return ("chr", tok.text)
            return ("id", tok.text)
        if tok.text == "(":
            e = self.expr(0)
            while self.peek() is not None and self.peek().text == ",":
                self.i += 1
                e = ("comma", e, self.expr(0))
            self.eat(")")
            return e
        raise ParseError(f"primary {tok}")

    def postfix(self, e):
        while True:
            tok = self.peek()
            if tok is None:
                return e
            if tok.text == "(":
                self.i += 1
                args = []
                if self.peek().text != ")":
                    args.append(self.expr(0))
                    while self.peek().text == ",":
                        self.i += 1
                        args.append(self.expr(0))
                self.eat(")")
                e = ("call", e, tuple(args))
            elif tok.text == "[":
                self.i += 1
                ix = self.expr(0)
                self.eat("]")
                e = ("idx", e, ix)
            elif tok.text in (".", "->"):
                self.i += 1
                e = ("mem", tok.text, e, self.eat().text)
            elif tok.text in ("++", "--"):
                self.i += 1
                e = ("post", tok.text, e)
            else:
                return e


def array_pointer(toks, vocab):
    """`T (*)[N]`, a pointer to an array, which castcount's grammar leaves out."""
    txt = [t.text for t in toks]
    try:
        k = txt.index("(")
    except ValueError:
        return None
    if txt[k:k + 3] != ["(", "*", ")"] or len(txt) < k + 6 or txt[k + 3] != "[" or txt[-1] != "]":
        return None
    base = CC.parse_type_name(toks, 0, k, vocab)
    return None if base is None else base + "(*)[]"


def comma_expr(p):
    e = p.expr(0)
    while p.peek() is not None and p.peek().text == ",":
        p.i += 1
        e = ("comma", e, p.expr(0))
    return e


def statement_exprs(toks, vocab):
    """The expressions of one printed line, as ASTs (None for a line with none)."""
    p = P(toks, vocab)
    out = []
    while p.peek() is not None and p.peek().text in ("}", "else", "do"):
        p.i += 1
    tok = p.peek()
    if tok is None or tok.text == "{":
        return out
    if tok.kind == "id" and tok.text in ("if", "while", "switch"):
        p.i += 1
        p.eat("(")
        out.append(comma_expr(p))
        p.eat(")")
        rest = toks[p.i:]
        if rest and rest[0].text not in ("{", ";"):
            out += statement_exprs(rest, vocab)
        return out
    if tok.kind == "id" and tok.text == "for":
        p.i += 1
        p.eat("(")
        for stop in (";", ";", ")"):
            if p.peek().text != stop:
                out.append(p.expr(0))
                while p.peek().text == ",":
                    p.i += 1
                    out.append(p.expr(0))
            p.eat(stop)
        return out
    if tok.kind == "id" and tok.text in ("goto", "break", "continue"):
        return out
    if tok.kind == "id" and tok.text == "case":
        p.i += 1
        out.append(p.expr(0))
        return out
    if tok.kind == "id" and tok.text == "return":
        p.i += 1
        if p.peek() is not None and p.peek().text != ";":
            out.append(("return", p.expr(0)))
        return out
    if len(toks) == 2 and toks[1].text == ":":
        return out
    e = p.expr(0)
    while p.peek() is not None and p.peek().text == ",":
        p.i += 1
        e = ("comma", e, p.expr(0))
    if p.peek() is None or p.peek().text != ";":
        raise ParseError(f"trailing {p.peek()}")
    out.append(e)
    return out


def strip(e):
    """(cast chain, node) with the casts directly over the node peeled off."""
    chain = []
    while isinstance(e, tuple) and e[0] == "cast":
        chain.append(e[1])
        e = e[2]
    return chain, e


def lit_value(text):
    t = text.lower().rstrip("ul")
    try:
        return int(t, 16) if t.startswith("0x") else int(t, 8) if len(t) > 1 and t.startswith("0") else int(t)
    except ValueError:
        return text


def canon(e):
    """The tree with every cast peeled and every literal read as its value."""
    if isinstance(e, tuple) and e and isinstance(e[0], str):
        _, e = strip(e)
        if e[0] == "num":
            return ("num", lit_value(e[1]))
    if isinstance(e, tuple):
        return tuple(canon(x) for x in e)
    return e


RANGE = {"i32": (-2**31, 2**31 - 1), "u32": (0, 2**32 - 1), "i64": (-2**63, 2**63 - 1), "u64": (0, 2**64 - 1)}
SMALL = {"char", "i8", "u8", "uchar", "bool", "i16", "u16"}


def promote(t):
    if t in SMALL:
        return "i32"
    return t if t in RANGE else None


def usual(a, b):
    """C's usual arithmetic conversions on two canonical integer types."""
    a, b = promote(a), promote(b)
    if a is None or b is None:
        return None
    if a == b:
        return a
    wa, wb = a in ("i64", "u64"), b in ("i64", "u64")
    if wa != wb:
        return a if wa else b
    return "u64" if wa else "u32"


def lit_type(text):
    t = text.lower()
    suf = ""
    while t and t[-1] in "ul":
        suf, t = t[-1] + suf, t[:-1]
    v = lit_value(text)
    if not isinstance(v, int):
        return None
    based = t.startswith("0x") or (len(t) > 1 and t.startswith("0"))
    u, lng = "u" in suf, "l" in suf
    order = [c for c in ("i32", "u32", "i64", "u64")
             if (not lng or c in ("i64", "u64")) and (not u or c.startswith("u"))
             and (u or based or c.startswith("i") or c == "u64")]
    return next((c for c in order if RANGE[c][0] <= v <= RANGE[c][1]), None)


def pointee(t):
    if t and t.endswith("*"):
        return t[:-1]
    return None


class Env:
    def __init__(self, vars_, funcs, ret, structs):
        self.vars, self.funcs, self.ret, self.structs = vars_, funcs, ret, structs


# Library prototypes kuna's call-argument rule trusts, as canonical types, for the
# functions the corpora call with a widened argument.
LIBC = {
    "memchr": ("void*", ["void*", "i32", "u64"]), "memcpy": ("void*", ["void*", "void*", "u64"]),
    "memmove": ("void*", ["void*", "void*", "u64"]), "memset": ("void*", ["void*", "i32", "u64"]),
    "memcmp": ("i32", ["void*", "void*", "u64"]), "strncmp": ("i32", ["char*", "char*", "u64"]),
    "strncpy": ("char*", ["char*", "char*", "u64"]), "strncat": ("char*", ["char*", "char*", "u64"]),
    "strndup": ("char*", ["char*", "u64"]), "strnlen": ("u64", ["char*", "u64"]),
    "malloc": ("void*", ["u64"]), "calloc": ("void*", ["u64", "u64"]), "realloc": ("void*", ["void*", "u64"]),
    "reallocarray": ("void*", ["void*", "u64", "u64"]), "xmalloc": ("void*", ["u64"]),
    "xcalloc": ("void*", ["u64", "u64"]), "xrealloc": ("void*", ["void*", "u64"]),
    "xcharalloc": ("char*", ["u64"]), "xnmalloc": ("void*", ["u64", "u64"]),
    "fwrite": ("u64", ["void*", "u64", "u64", "void*"]), "fread": ("u64", ["void*", "u64", "u64", "void*"]),
    "read": ("i64", ["i32", "void*", "u64"]), "write": ("i64", ["i32", "void*", "u64"]),
    "send": ("i64", ["i32", "void*", "u64", "i32"]), "recv": ("i64", ["i32", "void*", "u64", "i32"]),
    "lseek": ("i64", ["i32", "i64", "i32"]), "fseeko": ("i32", ["void*", "i64", "i32"]),
    "fseek": ("i32", ["void*", "i64", "i32"]), "qsort": (None, ["void*", "u64", "u64", "code*"]),
    "snprintf": ("i32", ["char*", "u64"]), "strtol": ("i64", ["char*", "char**", "i32"]),
    "mmap": ("void*", ["void*", "u64", "i32", "i32", "i32", "i64"]), "munmap": ("i32", ["void*", "u64"]),
    "getline": ("i64", ["char**", "u64*", "void*"]), "fgets": ("char*", ["char*", "i32", "void*"]),
    "posix_fadvise": ("i32", ["i32", "i64", "i64", "i32"]), "ftruncate": ("i32", ["i32", "i64"]),
    "pread": ("i64", ["i32", "void*", "u64", "i64"]), "pwrite": ("i64", ["i32", "void*", "u64", "i64"]),
}


def typeof(e, env):
    """The canonical C type of the printed expression e, when the text says."""
    k = e[0]
    if k == "id":
        return env.vars.get(e[1])
    if k == "num":
        return lit_type(e[1])
    if k == "chr":
        return "i32"
    if k == "str":
        return "char*"
    if k == "cast":
        return e[1]
    if k == "un":
        op, t = e[1], typeof(e[2], env)
        if op == "*":
            return pointee(t)
        if op == "&":
            return t + "*" if t else None
        if op in ("-", "+", "~"):
            return promote(t)
        if op == "!":
            return "i32"
        return t
    if k == "idx":
        return pointee(typeof(e[1], env))
    if k == "mem":
        base = typeof(e[2], env)
        base = pointee(base) if e[1] == "->" else base
        if base and base.startswith("named:"):
            return env.structs.get(base[6:], {}).get(e[3])
        return None
    if k == "bin":
        op = e[1]
        if op in SHIFT_CMP - {"<<", ">>"} or op in ("&&", "||"):
            return "i32"
        a, b = typeof(e[2], env), typeof(e[3], env)
        if op in ("<<", ">>"):
            return promote(a)
        if (a and a.endswith("*")) or (b and b.endswith("*")):
            if op == "-" and a and b and a.endswith("*") and b.endswith("*"):
                return "i64"
            return a if a and a.endswith("*") else (b if op == "+" else None)
        return usual(a, b)
    if k in ("asg", "post"):
        return typeof(e[2], env)
    if k == "tern":
        a, b = typeof(e[2], env), typeof(e[3], env)
        return usual(a, b) if promote(a) and promote(b) else (a if a == b else None)
    if k == "call":
        return (env.funcs.get(e[1][1]) or LIBC.get(e[1][1]) or (None,))[0] if e[1][0] == "id" else None
    if k == "comma":
        return typeof(e[2], env)
    if k == "sizeof":
        return "u64"
    return None


KEYWORDS = {"return", "goto", "case", "else", "do", "break", "continue", "sizeof", "if", "while", "for", "switch"}


def decl_type(toks, vocab):
    """(name, canonical type) of a declaration `T name`, `T *name` or `T name[N]`."""
    txt = [t.text for t in toks]
    end = txt.index("[") if "[" in txt else len(txt)
    if end < 2 or toks[end - 1].kind != "id" or txt[0] in KEYWORDS:
        return None
    ty = CC.parse_type_name(toks, 0, end - 1, vocab)
    if ty is None:
        return None
    return txt[end - 1], ty + ("*" if end < len(txt) else "")


def signature(text, vocab):
    """(name, return type, [param types], {param: type}) of a function's text."""
    lines = text.splitlines()
    if "{" not in lines or lines.index("{") == 0:
        return None
    toks = CC.tokenize(lines[lines.index("{") - 1])
    txt = [t.text for t in toks]
    if "(" not in txt:
        return None
    k = txt.index("(")
    if k < 2:
        return None
    ret = CC.parse_type_name(toks, 0, k - 1, vocab)
    m = CC.matching(toks).get(k)
    params, env = [], {}
    if m is not None and txt[k + 1:m] != ["void"]:
        start = k + 1
        depth = 0
        for j in range(k + 1, m + 1):
            if txt[j] in ("(", "["):
                depth += 1
            elif txt[j] in (")", "]") and j != m:
                depth -= 1
            if (txt[j] == "," and depth == 0) or j == m:
                d = decl_type(toks[start:j], vocab)
                params.append(d[1] if d else None)
                if d:
                    env[d[0]] = d[1]
                start = j + 1
    return txt[k - 1], ret, params, env


def structs_of(text, vocab):
    """{struct name: {field: type}} from the definitions `structdefs` prints."""
    out, cur = {}, None
    for L in text.splitlines():
        s = L.split("//")[0].strip()
        if s.startswith("struct ") and s.endswith("{"):
            cur = out.setdefault(s.split()[1], {})
        elif s.startswith("}"):
            cur = None
        elif cur is not None and s.endswith(";"):
            d = decl_type(CC.tokenize(s[:-1]), vocab)
            if d:
                cur[d[0]] = d[1]
    return out


def locals_of(text, vocab):
    env = {}
    lines = text.splitlines()
    for L in lines[lines.index("{") + 1 if "{" in lines else 0:]:
        s = L.split("//")[0].strip()
        if not s:
            break
        if not s.endswith(";"):
            continue
        d = decl_type(CC.tokenize(s[:-1]), vocab)
        if d:
            env[d[0]] = d[1]
    return env


def children(e):
    """(slot, child) for every expression child of node e (casts peeled)."""
    k = e[0]
    if k == "bin":
        return [(("bin", e[1], 0), e[2]), (("bin", e[1], 1), e[3])]
    if k == "asg":
        return [(("asg", e[1], 0), e[2]), (("asg", e[1], 1), e[3])]
    if k == "un":
        return [(("un", e[1], 0), e[2])]
    if k == "tern":
        return [(("tern", 0), e[1]), (("tern", 1), e[2]), (("tern", 2), e[3])]
    if k == "call":
        return [(("callee",), e[1])] + [(("call-arg", i), a) for i, a in enumerate(e[2])]
    if k == "idx":
        return [(("idx-base",), e[1]), (("index",), e[2])]
    if k == "mem":
        return [(("mem",), e[2])]
    if k == "post":
        return [(("post", e[1]), e[2])]
    if k == "comma":
        return [(("comma", 0), e[1]), (("comma", 1), e[2])]
    if k == "return":
        return [(("return",), e[1])]
    return []


def context(slot, parent):
    k = slot[0]
    if k == "bin":
        return ("arith:" if slot[1] in ARITH else "FORBIDDEN:") + slot[1]
    if k == "asg":
        if slot[2] == 0:
            return "lhs"
        lhs = strip(parent[2])[1]
        store = lhs[0] == "idx" or (lhs[0] == "un" and lhs[1] == "*") or lhs[0] == "mem"
        return ("store" if store else "assign") + ("" if slot[1] == "=" else ":" + slot[1])
    if k == "un":
        return "FORBIDDEN:unary" + slot[1]
    if k in ("call-arg", "return", "index"):
        return k
    return "other:" + k


def bare_type(e, env):
    chain, node = strip(e)
    return chain[0] if chain else typeof(node, env)


RING = {"+", "-", "*", "&", "|", "^"}


def ring_top(node, anc):
    """The top of the chain of + - * & | ^ (and unary - ~) that node sits in: the
    bits of every op in it depend only on the bits of its operands."""
    top = node
    for a in reversed(anc):
        if (a[0] == "bin" and a[1] in RING) or (a[0] == "un" and a[1] in ("-", "~")):
            top = a
        else:
            break
    return top


def verdict(ctx, t, f, parent, slot, env, olds):
    """'verified', 'unverified' or 'NARROW' for a cast to t removed from an operand
    now of type f, under parent (the new tree) at slot.  olds is (old parent,
    old ancestors, new ancestors) for the ring-chain rule."""
    if ctx.startswith("arith:"):
        y = bare_type(parent[3 - slot[2]], env)
        if f is None or y is None or promote(y) is None:
            return "unverified"
        if usual(t, y) == usual(f, y):
            return "verified"
        # At 64 bits an op of the ring gives the same bits whatever the signedness;
        # only the type the whole chain ends with can matter to what reads it.
        if slot[1] in RING and usual(t, y) in ("i64", "u64") and usual(f, y) in ("i64", "u64"):
            oparent, oanc, nanc = olds
            if typeof(ring_top(oparent, oanc), env) == typeof(ring_top(parent, nanc), env) is not None:
                return "verified"
        return "NARROW"
    if ctx.startswith("assign") or ctx.startswith("store"):
        y = typeof(parent[2], env)
    elif ctx == "return":
        y = env.ret
    elif ctx == "call-arg":
        callee = parent[1]
        sig = (env.funcs.get(callee[1]) or LIBC.get(callee[1])) if callee[0] == "id" else None
        y = sig[1][slot[1]] if sig and slot[1] < len(sig[1]) else None
    else:
        return "unverified"
    if y is not None and y == t:
        return "verified"
    if y is None or promote(y) is None or f is None or promote(f) is None:
        return "unverified"
    return "verified" if y in ("i64", "u64") else "NARROW"


def walk(old, new, stats, flags, where, env, parent=None, slot=None, oparent=None, oanc=(), nanc=()):
    oc, on = strip(old)
    nc, nn = strip(new)
    it = iter(oc)
    if not all(c in it for c in nc):
        flags.append(("ADDED", where, oc, nc))
        return
    removed = list(oc)
    for c in nc:
        removed.remove(c)
    if removed:
        ctx = context(slot, parent) if slot else "top"
        for c in removed:
            if c not in WIDE:
                flags.append(("SCOPE", where, c, ctx))
            elif ctx.startswith("FORBIDDEN"):
                flags.append(("FORBIDDEN", where, c, ctx))
            elif oc and c != oc[0]:
                stats["removed inner " + ctx] += 1
                flags.append(("INNER", where, oc, nc, ctx))
            else:
                stats["removed " + ctx] += 1
                f = nc[0] if nc else typeof(nn, env)
                v = verdict(ctx, c, f, parent, slot, env, (oparent, oanc[:-1], nanc[:-1]))
                stats[v + " " + ctx.split(":")[0]] += 1
                if v != "verified":
                    flags.append((v.upper() if v == "NARROW" else "UNVERIFIED", where, c, f, ctx))
    if on[0] == "num" and nn[0] == "num" and on[1] != nn[1]:
        stats["suffix"] += 1
    if on[0] == "bin":
        (_, ol), (_, orr) = children(on)
        (_, nl), (_, nr) = children(nn)
        if strip(ol)[0] and strip(orr)[0] and not strip(nl)[0] and not strip(nr)[0]:
            flags.append(("BOTH_LOST", where, on[1], None))
        if canon(ol) == canon(orr) and (len(strip(nl)[0]) < len(strip(ol)[0]) or len(strip(nr)[0]) < len(strip(orr)[0])):
            stats["same operand text, one cast kept"] += 1
    for (s_, oc2), (_, nc2) in zip(children(on), children(nn)):
        walk(oc2, nc2, stats, flags, where, env, nn, s_, on, oanc + (on,), nanc + (nn,))


def classify(old_dir, new_dir):
    stats, flags = collections.Counter(), []
    for f in sorted(Path(old_dir).rglob("*.c")):
        g = Path(new_dir) / f.relative_to(old_dir)
        if not g.exists():
            continue
        osrc, nsrc = f.read_text(errors="replace"), g.read_text(errors="replace")
        ov = CC.harvest_types(CC.tokenize(osrc), osrc)
        nv = CC.harvest_types(CC.tokenize(nsrc), nsrc)
        a = {addr: t for _, addr, t in CC.split_functions(osrc)}
        b = {addr: t for _, addr, t in CC.split_functions(nsrc)}
        sigs = {}
        for t in b.values():
            sg = signature(t, nv)
            if sg:
                sigs.setdefault(sg[0], (sg[1], sg[2]))
        for addr, ot in a.items():
            nt = b.get(addr)
            if nt is None or nt == ot:
                stats["functions same"] += nt is not None
                continue
            stats["functions changed"] += 1
            where = f"{f.relative_to(old_dir)}@{addr:#x}"
            sg = signature(nt, nv)
            env = Env({**(sg[3] if sg else {}), **locals_of(nt, nv)}, sigs, sg[1] if sg else None, structs_of(nt, nv))
            ol, nl = ot.splitlines(), nt.splitlines()
            if len(ol) != len(nl):
                flags.append(("LINECOUNT", where, len(ol), len(nl)))
                continue
            for x, y in zip(ol, nl):
                if x == y:
                    continue
                stats["lines changed"] += 1
                try:
                    ox = statement_exprs(CC.tokenize(x), ov)
                    ny = statement_exprs(CC.tokenize(y), nv)
                except (ParseError, KeyError, AttributeError, IndexError) as err:
                    flags.append(("UNPARSED", where, x.strip(), y.strip(), str(err)))
                    continue
                if len(ox) != len(ny) or any(canon(p) != canon(q) for p, q in zip(ox, ny)):
                    flags.append(("GROUPING", where, x.strip(), y.strip()))
                    continue
                n0 = len(flags)
                for p, q in zip(ox, ny):
                    walk(p, q, stats, flags, where, env)
                for k in range(n0, len(flags)):
                    flags[k] = flags[k] + (x.strip(), y.strip())
    return stats, flags


if __name__ == "__main__":
    stats, flags = classify(sys.argv[1], sys.argv[2])
    kinds = collections.Counter(fl[0] for fl in flags)
    print(json.dumps({"stats": dict(stats), "flags": dict(kinds)}, indent=1))
    shown = collections.Counter()
    for fl in flags:
        key = fl[0] + (" " + str(fl[4]).split(":")[0] if fl[0] == "UNVERIFIED" else "")
        shown[key] += 1
        if shown[key] <= 25:
            print(*fl, sep=" | ")
    if "--json" in sys.argv:
        out = sys.argv[sys.argv.index("--json") + 1]
        Path(out).write_text(json.dumps({"stats": dict(stats), "flags": dict(kinds),
                                         "listed": [list(map(str, fl)) for fl in flags]}, indent=1))
