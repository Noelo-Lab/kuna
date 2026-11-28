"""What each recompiled method refers to, and in what order: wrong code at the scale of the benchmark projects.

The benchmark scores a decompiler's text against the project's source, which does not see a call that resolves to
another overload or a lambda that reads another variable: the text looks like the source. This takes a harness run
(``run --full``), whose scoring already recompiled each decompiled top-level file against the original jar with the
javac, ``--release`` and debug flags that reproduce the project's class files, and compares every method of every
class of each file that compiled as written (``classes.json`` ``raw_compiles``), nested classes included, with the
original, as the sequences of what the code refers to:

* invoked members (kind, owner, name, descriptor); ``invokedynamic`` by bootstrap method, name, descriptor and
  static arguments; a lambda by its descriptor and interface, its body compared with the body of the lambda at the
  same place in the recompiled caller, not by its ``lambda$..$N`` name;
* field reads and writes; the types of ``new``, ``checkcast``, ``instanceof`` and array creation; constants
  (``ldc``, pushes, ``iinc`` amounts); arithmetic, conversion and comparison operators, array element accesses,
  ``athrow`` and monitors; the keys of each switch, grouped by target;
* the exception table as (handler type, the references the range covers), compared once the sequences are equal.

Local slots, jumps, stack operations and returns are left out, so a method whose sequences are equal is taken as
round-tripped whatever its jump and slot layout. Two references name the same member when, after the recompiled
anonymous and local classes are paired with the originals (``bytecode.match_classes``), their names and descriptors
are equal and their owners resolve to the same declaration (JVMS 5.4.3, through the project's classpath and the
JDK's ``ct.sym``): a static call, ``invokespecial`` or field by the class that declares it, a virtual or interface
call by the topmost supertype of its owner that declares it, since dispatch on the same object reaches the same
method whichever of its static types names it (``ArrayList.add`` for ``List.add``).

Per differing method one line, ``DIFF project categories class#method detail``, then a summary per project. From
the repo root::

    python3 -m scripts.javabench.refs RUN_OR_PROJECT_DIR...      [--jobs 8] [--out FILE.jsonl] [--quiet]
    python3 -m scripts.javabench.refs --classes ORIGINAL_DIR RECOMPILED_DIR      # e.g. the probe's --keep a/ b/
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import zipfile
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
from difflib import SequenceMatcher
from pathlib import Path

from .bytecode import match_classes
from .classfile import ACC_BRIDGE, ACC_PRIVATE, ACC_STATIC, ACC_SYNTHETIC, NEWARRAY_TYPES, ClassFile, decode_code, parse_class

CALL = {"invokevirtual": "virtual", "invokeinterface": "virtual", "invokespecial": "special", "invokestatic": "static"}
FIELD = {"getfield": ("get", False), "putfield": ("put", False), "getstatic": ("get", True), "putstatic": ("put", True)}
TYPED = ("new", "checkcast", "instanceof", "anewarray")
SHORT_CONST = {"aconst_null": ("null", None), "iconst_m1": ("I", -1), "lconst_0": ("J", 0), "lconst_1": ("J", 1),
               "fconst_0": ("F", "0.0"), "fconst_1": ("F", "1.0"), "fconst_2": ("F", "2.0"), "dconst_0": ("D", "0.0"),
               "dconst_1": ("D", "1.0"), **{f"iconst_{k}": ("I", k) for k in range(6)}}
CONVERSIONS = {"i2l", "i2f", "i2d", "l2i", "l2f", "l2d", "f2i", "f2l", "f2d", "d2i", "d2l", "d2f", "i2b", "i2c", "i2s"}
NULL_CHECKS = {("java/lang/Object", "getClass", "()Ljava/lang/Class;"),
               ("java/util/Objects", "requireNonNull", "(Ljava/lang/Object;)Ljava/lang/Object;")}
ENDS = {"ireturn", "lreturn", "freturn", "dreturn", "areturn", "return", "athrow", "ret"}
VALUEOF = {"I", "J", "F", "D", "Z", "C", "Ljava/lang/Object;"}
OPS = {"iadd", "ladd", "fadd", "dadd", "isub", "lsub", "fsub", "dsub", "imul", "lmul", "fmul", "dmul", "idiv", "ldiv",
       "fdiv", "ddiv", "irem", "lrem", "frem", "drem", "ineg", "lneg", "fneg", "dneg", "ishl", "lshl", "ishr", "lshr",
       "iushr", "lushr", "iand", "land", "ior", "lor", "ixor", "lxor", "i2l", "i2f", "i2d", "l2i", "l2f", "l2d", "f2i",
       "f2l", "f2d", "d2i", "d2l", "d2f", "i2b", "i2c", "i2s", "lcmp", "fcmpl", "fcmpg", "dcmpl", "dcmpg", "iaload",
       "laload", "faload", "daload", "aaload", "baload", "caload", "saload", "iastore", "lastore", "fastore",
       "dastore", "aastore", "bastore", "castore", "sastore", "arraylength", "athrow", "monitorenter", "monitorexit"}
BUILDERS = ("java/lang/StringBuilder", "java/lang/StringBuffer")
LAMBDA_FACTORY = "java/lang/invoke/LambdaMetafactory"
CONCAT_FACTORY = "java/lang/invoke/StringConcatFactory"
_ACCESSOR = re.compile(r"access\$\d+")
_OUTER_THIS = re.compile(r"this\$\d+")
_DESC_CLASS = re.compile(r"L([^;<]+);")
_REF_TYPE = re.compile(r"\[*L[^;]+;|\[+[ZBCSIJFD]")

# Most dangerous first: a method is listed under its first category in the summary's "first" column.
CATEGORIES = [
    "other overload", "other method called", "other owner", "call missing", "call added",
    "field write missing", "field write added", "other field", "field read/write differs", "field read missing",
    "field read added", "constant differs", "constant missing", "constant added", "lambda capture differs",
    "lambda type differs", "lambda instantiated type differs", "lambda missing", "lambda added", "lambda body missing", "string concatenation differs",
    "invokedynamic differs", "cast missing", "cast added", "cast type differs", "instanceof differs", "new differs",
    "operator differs", "operator missing", "operator added", "conversion differs", "conversion missing",
    "conversion added", "switch keys differ", "exception range differs",
    "string concatenation form", "try-with-resources form", "boolean materialization", "references moved", "same references, other order", "code duplicated", "code merged", "same blocks, other order",
    "method missing", "class missing", "class header differs", "unreadable",
]


class Hierarchy:
    """Supertypes and members of the classes of a classpath and of the JDK (``ct.sym``), read on demand."""

    def __init__(self, jars: list[str], extra: dict[str, ClassFile] | None = None):
        self.where: dict[str, tuple[str, str]] = {}
        for jar in jars:
            try:
                names = zipfile.ZipFile(jar).namelist()
            except (OSError, zipfile.BadZipFile):
                continue
            for n in names:
                if n.endswith(".class") and not n.startswith("META-INF/"):
                    self.where.setdefault(n[:-6], (jar, n))
        ct = _ct_sym()
        if ct:
            best: dict[str, tuple[str, str]] = {}
            for n in zipfile.ZipFile(ct).namelist():
                if n.endswith(".sig"):
                    rel, mod_cls = n.split("/", 1)
                    cls = mod_cls.split("/", 1)[1][:-4] if "/" in mod_cls else mod_cls[:-4]
                    if cls not in best or max(rel) > max(best[cls][0]):
                        best[cls] = (rel, n)
            for cls, (_rel, n) in best.items():
                self.where.setdefault(cls, (ct, n))
        self.extra = extra or {}
        self.cache: dict[str, ClassFile | None] = {}
        self.zips: dict[str, zipfile.ZipFile] = {}
        self.memo: dict[tuple, str] = {}

    def info(self, name: str) -> ClassFile | None:
        if name in self.extra:
            return self.extra[name]
        if name not in self.cache:
            cf = None
            loc = self.where.get(name)
            if loc:
                z = self.zips.get(loc[0]) or self.zips.setdefault(loc[0], zipfile.ZipFile(loc[0]))
                try:
                    cf = parse_class(z.read(loc[1]))
                except Exception:
                    cf = None
            self.cache[name] = cf
        return self.cache[name]

    def supertypes(self, name: str) -> list[str] | None:
        """``name`` and its supertypes, nearest first; None when a class on the way is unknown."""
        out, todo = [], [name]
        while todo:
            n = todo.pop(0)
            if n in out:
                continue
            cf = self.info(n)
            if cf is None and n == "java/lang/Object":
                out.append(n)
                continue
            if cf is None:
                return None
            out.append(n)
            todo += ([cf.super_name] if cf.super_name else []) + list(cf.interfaces)
        return out

    def declares(self, cls: str, name: str, desc: str) -> int | None:
        cf = self.info(cls)
        m = cf and next((m for m in cf.methods if m.name == name and m.descriptor == desc), None)
        return m.access if m else None

    def canonical(self, kind: str, owner: str, name: str, desc: str) -> str:
        """The owner that names the declaration this reference reaches (``owner`` when it cannot be resolved)."""
        key = (kind, owner, name, desc)
        if key not in self.memo:
            self.memo[key] = self._canonical(kind, owner, name, desc) or owner
        return self.memo[key]

    def _canonical(self, kind: str, owner: str, name: str, desc: str) -> str | None:
        if owner.startswith("[") or name == "<init>":
            return owner
        if kind == "field":
            return self._resolve_field(owner, name, set())
        chain = []
        c = owner
        while c:
            cf = self.info(c)
            chain.append(c)
            if cf is None:
                if c != "java/lang/Object":
                    return None
                break
            c = cf.super_name
        if kind in ("static", "special"):
            hit = next((c for c in chain if self.declares(c, name, desc) is not None), None)
            if hit or kind == "special":
                return hit
            sup = self.supertypes(owner) or []
            return next((c for c in sup if self.declares(c, name, desc) is not None), None)
        acc = self.declares(owner, name, desc)
        if acc is not None and acc & (ACC_PRIVATE | ACC_STATIC):
            return owner
        sup = self.supertypes(owner)
        if sup is None:
            return None
        decl = [c for c in sup if (a := self.declares(c, name, desc)) is not None and not a & (ACC_PRIVATE | ACC_STATIC)]
        roots = [c for c in decl if not any(d != c and d in (self.supertypes(c) or []) for d in decl)]
        return min(roots) if roots else None

    def _resolve_field(self, owner: str, name: str, seen: set) -> str | None:
        if owner in seen:
            return None
        seen.add(owner)
        cf = self.info(owner)
        if cf is None:
            return None
        if name in cf.field_names:
            return owner
        for i in cf.interfaces:
            hit = self._resolve_field(i, name, seen)
            if hit:
                return hit
        return self._resolve_field(cf.super_name, name, seen) if cf.super_name else None


def _ct_sym() -> str | None:
    javac = os.environ.get("JAVA_HOME")
    homes = [javac] if javac else []
    try:
        homes.append(str(Path(subprocess.run(["which", "javac"], capture_output=True, text=True).stdout.strip())
                         .resolve().parents[1]))
    except (OSError, IndexError):
        pass
    return next((str(Path(h) / "lib/ct.sym") for h in homes if h and (Path(h) / "lib/ct.sym").exists()), None)


class Side:
    """One side of a comparison: its classes, the renaming into the original's names, and the hierarchy."""

    def __init__(self, classes: dict[str, ClassFile], tr, hier: Hierarchy):
        self.classes, self.tr, self.hier = classes, tr, hier
        self.maps: dict[tuple[str, str], dict[int, str]] = {}

    def switch_map(self, owner: str, name: str) -> dict[int, str]:
        """The case key -> enum constant table javac's ``$SwitchMap$`` array ``owner.name`` holds."""
        if (owner, name) not in self.maps:
            table: dict[int, str] = {}
            cf = self.classes.get(owner)
            ecj = name.startswith("$SWITCH_TABLE$")
            m = cf and (_method(cf, name, "()[I") if ecj else _method(cf, "<clinit>", "()V"))
            if m and m.code:
                array = const = key = None
                for ins in decode_code(m.code.code):
                    if ins.op == "getstatic":
                        _o, n, d = cf.cp.member(ins.args[0])
                        array, const, key = (n, const, None) if d == "[I" else (array, n, None)
                    elif ins.op in SHORT_CONST or ins.op in ("bipush", "sipush"):
                        key = SHORT_CONST[ins.op][1] if ins.op in SHORT_CONST else ins.args[0]
                    elif ins.op == "iastore" and (ecj or array == name) and const and key is not None:
                        table[key] = const
            self.maps[(owner, name)] = table
        return self.maps[(owner, name)]

    def capture_index(self, owner: str, name: str) -> str:
        cf = self.classes.get(owner)
        caps = [f for f in cf.field_names if f.startswith("val$")] if cf else []
        return f"val${caps.index(name)}" if name in caps else name


class Refs:
    """One method's reference sequence, its exception table and the bodies of its lambdas (by token index)."""

    def __init__(self, cf: ClassFile, m, side: Side):
        self.tokens: list[tuple] = []
        self.lambdas: dict[int, tuple[str, str]] = {}
        self.cf, self.side, self.switch_map, self.ctor = cf, side, None, m.name == "<init>"
        insns = decode_code(m.code.code)
        self.at = {ins.pc: ins for ins in insns}
        leaders = {0} | {e.handler_pc for e in m.code.exceptions}
        for k, ins in enumerate(insns):
            if ins.targets or ins.op in ENDS:
                leaders |= set(ins.targets) | ({insns[k + 1].pc} if k + 1 < len(insns) else set())
        pcs, block_of, block = [], [], -1
        for k, ins in enumerate(insns):
            block += ins.pc in leaders
            if k + 1 < len(insns) and insns[k + 1].op == "pop" and ins.op in ("invokevirtual", "invokestatic") \
                    and cf.cp.member(ins.args[0]) in NULL_CHECKS:
                pcs.append(ins.pc)
                block_of.append(block)
                self.tokens.append(("nullcheck",))
                continue
            for t in self._tokens(cf, ins):
                pcs.append(ins.pc)
                block_of.append(block)
                self.tokens.append(t)
        drop = set()
        for i in range(len(self.tokens) - 1):
            a, b = self.tokens[i], self.tokens[i + 1]
            if a == ("const", "null", None) and b[0] == "checkcast":
                drop.add(i + 1)
            if a[0] == "call" and a[2:4] == ("java/lang/String", "valueOf") and a[4][1:-19] in VALUEOF:
                if b[0] == "call" and b[3] == "append" and b[4].startswith("(Ljava/lang/String;)"):
                    self.tokens[i + 1] = b[:4] + ("(" + a[4][1:-19] + b[4][19:],)
                    drop.add(i)
                elif a[4] == "(Ljava/lang/Object;)Ljava/lang/String;" and any(
                        t[0] == "concat" and block_of[k] == block_of[i] for k, t in enumerate(self.tokens[i + 1:], i + 1)):
                    drop.add(i)
        if drop:
            keep = [i for i in range(len(self.tokens)) if i not in drop]
            new = {old: n for n, old in enumerate(keep)}
            self.lambdas = {new[i]: v for i, v in self.lambdas.items()}
            self.tokens, pcs, block_of = ([seq[i] for i in keep] for seq in (self.tokens, pcs, block_of))
        blocks: dict[int, list[tuple]] = {}
        for t, bl in zip(self.tokens, block_of):
            blocks.setdefault(bl, []).append(t)
        self.blocks = Counter(tuple(v) for v in blocks.values())
        ranges = ((side.tr(e.catch_type) if e.catch_type else "any",
                   tuple(i for i, pc in enumerate(pcs) if e.start_pc <= pc < e.end_pc)) for e in m.code.exceptions)
        self.exceptions = sorted(r for r in ranges if r[1])

    def _tokens(self, cf, ins) -> list[tuple]:
        t = self._token(cf, cf.cp, ins)
        if t is None:
            return []
        if t[0] == "call" and t[1] == "static" and t[3] == "$values" and t[2] == self.side.tr(cf.name):
            m = next((m for m in cf.methods if m.name == "$values" and m.access & ACC_SYNTHETIC and m.code), None)
            if m is not None:
                return [u for i in decode_code(m.code.code) for u in self._tokens(cf, i)]
        if t[0] == "iinc":
            return [("const", "I", abs(t[1])), ("op", "iadd" if t[1] >= 0 else "isub")]
        return [t]

    def _resolve(self, pc: int) -> int:
        for _ in range(16):
            ins = self.at.get(pc)
            if ins is None or ins.op not in ("goto", "goto_w"):
                break
            pc = ins.targets[0]
        return pc

    def _token(self, cf, cp, ins):
        op, tr, hier = ins.op, self.side.tr, self.side.hier
        if op in CALL:
            owner, name, desc = cp.member(ins.args[0])
            if name.startswith("$SWITCH_TABLE$") and op == "invokestatic":
                self.switch_map = self.side.switch_map(owner, name)
                return ("switchmap",)
            owner, desc = tr(owner), tr(desc)
            name = _ACCESSOR.sub("access$#", name)
            return ("call", CALL[op], hier.canonical(CALL[op], owner, name, desc), name, desc)
        if op in FIELD:
            raw, name, desc = cp.member(ins.args[0])
            if name.startswith("$SwitchMap$"):
                self.switch_map = self.side.switch_map(raw, name)
                return ("switchmap",)
            if self.ctor and op == "getfield" and raw == cf.name and _OUTER_THIS.fullmatch(name):
                return None
            if name.startswith("val$"):
                name = self.side.capture_index(raw, name)
            return ("field", *FIELD[op], hier.canonical("field", tr(raw), name, desc), name, tr(desc))
        if op in TYPED:
            return (op, tr(cp.class_name(ins.args[0])))
        if op == "multianewarray":
            return (op, tr(cp.class_name(ins.args[0])), ins.args[1])
        if op == "newarray":
            return (op, NEWARRAY_TYPES.get(ins.args[0], ins.args[0]))
        if op in SHORT_CONST:
            return ("const", *SHORT_CONST[op])
        if op in ("bipush", "sipush"):
            return ("const", "I", ins.args[0])
        if op in ("ldc", "ldc_w", "ldc2_w"):
            e = cp.entries[ins.args[0]]
            if e[0] in (3, 4, 5, 6):
                return ("const", {3: "I", 4: "F", 5: "J", 6: "D"}[e[0]], e[1] if e[0] in (3, 5) else repr(e[1]))
            if e[0] == 7:
                return ("const", "class", tr(cp.class_name(ins.args[0])))
            return ("const", "ldc", tr(cp.describe(ins.args[0])))
        if op == "iinc":
            return ("iinc", ins.args[1])
        if op in CONVERSIONS:
            return ("conv", op)
        if op in OPS:
            return ("op", op)
        if op in ("tableswitch", "lookupswitch"):
            default, targets = self._resolve(ins.targets[0]), [self._resolve(t) for t in ins.targets[1:]]
            keys = range(ins.args[0], ins.args[1] + 1) if op == "tableswitch" else ins.args
            names = self.switch_map or {}
            groups: dict[int, list] = {}
            for k, t in zip(keys, targets):
                if t != default:
                    groups.setdefault(t, []).append(names.get(k, k))
            self.switch_map = None
            return ("switch", tuple(sorted(tuple(sorted(g, key=str)) for g in groups.values())))
        if op == "invokedynamic":
            return self._indy(cf, cp, ins.args[0], tr)
        return None

    def _indy(self, cf, cp, idx, tr):
        e = cp.entries[idx]
        name, desc = cp.name_and_type(e[2])
        ref, args = cf.bootstrap[e[1]]
        bowner, bname, _bdesc = cp.member(cp.entries[ref][2])
        if bowner == LAMBDA_FACTORY and len(args) >= 3:
            kind, impl = cp.entries[args[1]][1], cp.entries[args[1]][2]
            iowner, iname, idesc = cp.member(impl)
            sam, inst = cp.describe(args[0]), tr(cp.describe(args[2]))
            if iowner == cf.name and iname.startswith("lambda$"):
                self.lambdas[len(self.tokens)] = (iname, idesc)
                return ("lambda", name, tr(desc), sam, inst, kind)
            return ("lambda", name, tr(desc), sam, inst, kind, tr(iowner), iname, tr(idesc))
        if bowner == CONCAT_FACTORY:
            return ("concat", _REF_TYPE.sub("A", tr(desc)).replace("B", "I").replace("S", "I"),
                    tuple(cp.describe(a) for a in args))
        return ("indy", f"{bowner}.{bname}", name, tr(desc), tuple(tr(cp.describe(a)) for a in args))


def _family(t: tuple) -> str:
    return {"call": "call", "field": "field", "const": "constant", "iinc": "constant", "op": "operator", "conv": "conversion",
            "checkcast": "cast", "instanceof": "instanceof", "lambda": "lambda", "concat": "concat",
            "indy": "indy", "switch": "switch", "nullcheck": "nullcheck", "switchmap": "switch"}.get(t[0], "new")


def _pair_category(a: tuple, b: tuple) -> str:
    fam = _family(a)
    if fam == "call":
        if a[3] != b[3]:
            return "other method called"
        return "other owner" if a[4] == b[4] else "other overload"
    if fam == "field":
        return "field read/write differs" if a[1] != b[1] else "other field"
    if fam == "lambda":
        if a[2] != b[2]:
            return "lambda capture differs"
        return "lambda instantiated type differs" if a[:4] + a[5:] == b[:4] + b[5:] else "lambda type differs"
    return {"constant": "constant differs", "operator": "operator differs", "conversion": "conversion differs",
            "cast": "cast type differs",
            "instanceof": "instanceof differs", "concat": "string concatenation differs",
            "indy": "invokedynamic differs", "switch": "switch keys differ"}.get(fam, "new differs")


def _lone_category(t: tuple, side: str) -> str:
    fam = _family(t)
    if fam == "field":
        return f"field {'read' if t[1] == 'get' else 'write'} {side}"
    if fam in ("call", "constant", "operator", "conversion", "cast", "lambda"):
        return f"{fam} {side}"
    if fam == "nullcheck":
        return f"call {side}"
    return {"instanceof": "instanceof differs", "concat": "string concatenation differs",
            "indy": "invokedynamic differs", "switch": "switch keys differ"}.get(fam, "new differs")


def show(t: tuple) -> str:
    if t[0] == "call":
        return f"{t[1]} {t[2]}.{t[3]}{t[4]}"
    if t[0] == "field":
        return f"{t[1]}{'static' if t[2] else 'field'} {t[3]}.{t[4]}"
    return " ".join(str(x) for x in t)


def diff_refs(a: Refs, b: Refs) -> tuple[list[str], list[str], list[tuple[int, int]]]:
    """(categories, details, aligned lambda pairs as token indexes) for two methods' references."""
    if a.tokens == b.tokens:
        pairs = [(i, i) for i in a.lambdas if i in b.lambdas]
        if a.exceptions != b.exceptions:
            only_a, only_b = Counter(a.exceptions) - Counter(b.exceptions), Counter(b.exceptions) - Counter(a.exceptions)
            return ["exception range differs"], [f"catch {sorted(only_a)[:3]} => {sorted(only_b)[:3]}"], pairs
        return [], [], pairs
    if Counter(a.tokens) == Counter(b.tokens):
        order = "same blocks, other order" if a.blocks == b.blocks else "same references, other order"
        return [order], [], _pairs_by_value(a, b, sorted(a.lambdas), sorted(b.lambdas))
    cats, details, pairs = [], [], []
    regions = []
    for tag, i1, i2, j1, j2 in SequenceMatcher(None, a.tokens, b.tokens, autojunk=False).get_opcodes():
        if tag == "equal":
            pairs += [(i1 + k, j1 + k) for k in range(i2 - i1) if i1 + k in a.lambdas and j1 + k in b.lambdas]
        elif i1 == i2 and _occurs(b.tokens, b.tokens[j1:j2], j1):
            cats.append("code duplicated")
            details.append(f"+ {'; '.join(show(t) for t in b.tokens[j1:j2][:3])}")
        elif j1 == j2 and _occurs(a.tokens, a.tokens[i1:i2], i1):
            cats.append("code merged")
            details.append(f"- {'; '.join(show(t) for t in a.tokens[i1:i2][:3])}")
        else:
            regions.append((i1, i2, j1, j2))
    moved = (Counter(a.tokens[i] for i1, i2, _j1, _j2 in regions for i in range(i1, i2))
             & Counter(b.tokens[j] for _i1, _i2, j1, j2 in regions for j in range(j1, j2)))
    if moved:
        cats.append("references moved")
    ma, mb, moved_a, moved_b = moved.copy(), moved.copy(), [], []
    for i1, i2, j1, j2 in regions:
        left, right = [], []
        for i in range(i1, i2):
            if ma[a.tokens[i]] > 0:
                ma[a.tokens[i]] -= 1
                moved_a.append(i)
            else:
                left.append(i)
        for j in range(j1, j2):
            if mb[b.tokens[j]] > 0:
                mb[b.tokens[j]] -= 1
                moved_b.append(j)
            else:
                right.append(j)
        if not left and not right:
            continue
        if _concat_form(a.tokens, left, b.tokens, right):
            cats.append("string concatenation form")
            continue
        la, rb = [a.tokens[i] for i in left], [b.tokens[j] for j in right]
        if not (la and rb) and _booleans(la or rb):
            cats.append("boolean materialization")
            continue
        if all(map(_twr, la + rb)) and any(t[3] == "addSuppressed" for t in a.tokens + b.tokens if t[0] == "call"):
            cats.append("try-with-resources form")
            continue
        for fam in dict.fromkeys(_family(a.tokens[i]) for i in left):
            ls = [i for i in left if _family(a.tokens[i]) == fam]
            rs = [j for j in right if _family(b.tokens[j]) == fam]
            for i, j in zip(ls, rs):
                cats.append(_pair_category(a.tokens[i], b.tokens[j]))
                if i in a.lambdas and j in b.lambdas:
                    pairs.append((i, j))
            left = [i for i in left if i not in ls[: len(rs)]]
            right = [j for j in right if j not in rs[: len(ls)]]
        cats += [_lone_category(a.tokens[i], "missing") for i in left]
        cats += [_lone_category(b.tokens[j], "added") for j in right]
        details.append(f"{'; '.join(show(t) for t in a.tokens[i1:i2][:3]) or '-'} => "
                       f"{'; '.join(show(t) for t in b.tokens[j1:j2][:3]) or '-'}")
    pairs += _pairs_by_value(a, b, [i for i in moved_a if i in a.lambdas], [j for j in moved_b if j in b.lambdas])
    return cats, details, pairs


def _booleans(ts: list[tuple]) -> bool:
    """Whether ``ts`` are pairs of ``1`` and ``0``: a condition javac materialized as a value on one side only."""
    return ts.count(("const", "I", 1)) == ts.count(("const", "I", 0)) == len(ts) / 2


def _twr(t: tuple) -> bool:
    """A reference javac 8 and javac 9+ desugar ``try``-with-resources into in different numbers."""
    return (t in (("op", "athrow"), ("const", "null", None))
            or t[0] == "call" and (t[3:] == ("close", "()V") or t[2:4] == ("java/lang/Throwable", "addSuppressed")))


def _concatish(t: tuple) -> bool:
    return (t[0] in ("concat", "const") or t[0] == "new" and t[1] in BUILDERS
            or t[0] == "call" and (t[2] in BUILDERS or t[2:4] == ("java/lang/String", "valueOf")
                                   or t[2:5] == ("java/lang/Object", "toString", "()Ljava/lang/String;")))


def _concat_form(at: list[tuple], left: list[int], bt: list[tuple], right: list[int]) -> bool:
    """Whether a region only trades a ``StringBuilder`` chain for an ``invokedynamic`` concatenation (or back)."""
    la, rb = [at[i] for i in left], [bt[j] for j in right]
    if not all(map(_concatish, la + rb)):
        return False
    form = lambda ts: ({t[1] for t in ts if t[0] == "new"} | {t[2] for t in ts if t[0] == "call" and t[2] in BUILDERS}
                       | {"indy" for t in ts if t[0] == "concat"})
    if bool(form(la) or form(rb)) and form(la) != form(rb):
        return True
    valueof = lambda t: t[0] == "call" and t[2:4] == ("java/lang/String", "valueOf")
    folded = {t[4][1:-19] for t in la + rb if valueof(t)}
    if not folded:
        return False
    loose = lambda t: (t[0], re.sub(r"[ZCIJFD]", lambda m: "A" if m.group(0) in folded else m.group(0), t[1][:t[1].index(")")]), t[2])
    return [loose(t) for t in la if not valueof(t)] == [loose(t) for t in rb if not valueof(t)]


def _occurs(seq: list[tuple], run: list[tuple], at: int) -> bool:
    """Whether ``run`` (taken from ``seq`` at ``at``) also occurs in ``seq`` somewhere else."""
    n = len(run)
    return any(k != at and seq[k:k + n] == run for k in range(len(seq) - n + 1))


def _pairs_by_value(a: Refs, b: Refs, ia: list[int], jb: list[int]) -> list[tuple[int, int]]:
    free: dict[tuple, list[int]] = {}
    for j in jb:
        free.setdefault(b.tokens[j], []).append(j)
    return [(i, free[a.tokens[i]].pop(0)) for i in ia if free.get(a.tokens[i])]


def compare_method(ocf, om, rcf, rm, os: Side, rs: Side, depth=0) -> tuple[list[str], list[str]]:
    a, b = Refs(ocf, om, os), Refs(rcf, rm, rs)
    cats, details, pairs = diff_refs(a, b)
    if depth < 8:
        for i, j in pairs:
            olm = _method(ocf, *a.lambdas[i])
            rlm = _method(rcf, *b.lambdas[j])
            if olm is None or rlm is None or olm.code is None or rlm.code is None:
                cats.append("lambda body missing")
                continue
            c, d = compare_method(ocf, olm, rcf, rlm, os, rs, depth + 1)
            cats += c
            details += [f"in {a.lambdas[i][0]}: {x}" for x in d]
    return cats, details


def _method(cf: ClassFile, name: str, desc: str):
    return next((m for m in cf.methods if m.name == name and m.descriptor == desc), None)


def _translator(names: dict[str, str]):
    """A function renaming the recompiled classes in ``names`` (recompiled -> original) inside a name or descriptor."""
    moved = {k: v for k, v in names.items() if k != v}
    if not moved:
        return lambda s: s
    return lambda s: moved.get(s) or (_DESC_CLASS.sub(lambda m: f"L{moved.get(m.group(1), m.group(1))};", s)
                                      if "L" in s else s)


def _old_synthetic(m) -> bool:
    """A member javac before 5 marked synthetic only by attribute: an accessor, a class-literal helper, or the
    constructor a private nested class gets for its outer class, whose last parameter is an anonymous class."""
    return bool(_ACCESSOR.fullmatch(m.name) or m.name == "class$"
                or m.name == "<init>" and re.search(r"\$\d+;\)V$", m.descriptor))


def pair_classes(original: dict[str, ClassFile], recompiled: dict[str, ClassFile]) -> dict[str, str]:
    """Recompiled -> original class names: ``bytecode.match_classes``, after pairing the anonymous and local classes
    of each enclosing method in javac's order, so that a class numbered differently is still paired."""
    out = {n: n for n in recompiled if n in original and not (recompiled[n].is_anonymous or recompiled[n].is_local)}
    groups: dict[tuple, tuple[list[str], list[str]]] = {}
    for side, classes in ((0, original), (1, recompiled)):
        for n, cf in sorted(classes.items(), key=lambda kv: _index_order(kv[0])):
            if (cf.is_anonymous or cf.is_local) and cf.enclosing_method and cf.enclosing_method[1]:
                own = cf.own_inner_info()
                key = (*cf.enclosing_method, cf.super_name, tuple(cf.interfaces), own.simple_name if own else None)
                groups.setdefault(key, ([], []))[side].append(n)
    changed = True
    while changed:
        changed = False
        for (outer, mname, mdesc, sup, ifs, simple), (onames, rnames) in groups.items():
            if outer in out.values() or outer in original:
                for r, o in zip(rnames, onames):
                    if r not in out and o not in out.values():
                        out[r] = o
                        changed = True
    rest = match_classes({n: c for n, c in original.items() if n not in out.values()},
                         {n: c for n, c in recompiled.items() if n not in out})
    return {**rest, **out}


def _index_order(name: str) -> list:
    return [int(p) if p.isdigit() else p for p in re.split(r"(\d+)", name)]


def compare_classes(original: dict[str, ClassFile], recompiled: dict[str, ClassFile], hier: Hierarchy) -> dict:
    """``{"compared": n, "equal": n, "methods": [{class, method, categories, details}]}`` for one file's classes."""
    rmap = pair_classes(original, recompiled)
    inv = {o: r for r, o in rmap.items()}
    rtr = _translator(rmap)
    os, rs = Side(original, lambda s: s, hier), Side(recompiled, rtr, hier)
    out = {"compared": 0, "equal": 0, "methods": []}
    for name, ocf in sorted(original.items()):
        if ocf.is_synthetic:
            continue
        rcf = recompiled.get(inv.get(name, ""))
        if rcf is not None and (rtr(rcf.super_name or "") != (ocf.super_name or "")
                                or sorted(map(rtr, rcf.interfaces)) != sorted(ocf.interfaces)):
            out["methods"].append({"class": name, "method": "<header>", "categories": ["class header differs"],
                                   "details": [f"{ocf.super_name} {ocf.interfaces} => {rcf.super_name} {rcf.interfaces}"]})
        for om in ocf.methods:
            if om.code is None or om.access & (ACC_SYNTHETIC | ACC_BRIDGE) or _old_synthetic(om):
                continue
            out["compared"] += 1
            key = om.name + om.descriptor
            rm = None if rcf is None else next(
                (m for m in rcf.methods if m.name == om.name and rtr(m.descriptor) == om.descriptor
                 and not m.access & (ACC_SYNTHETIC | ACC_BRIDGE)), None)
            if rm is None or rm.code is None:
                cat = "class missing" if rcf is None else "method missing"
                out["methods"].append({"class": name, "method": key, "categories": [cat], "details": []})
                continue
            try:
                cats, details = compare_method(ocf, om, rcf, rm, os, rs)
            except Exception as e:  # a class file the reader rejects
                cats, details = ["unreadable"], [repr(e)[:200]]
            if cats:
                out["methods"].append({"class": name, "method": key, "categories": sorted(set(cats), key=_rank),
                                       "details": details[:4]})
            else:
                out["equal"] += 1
    return out


def _rank(c: str) -> int:
    return CATEGORIES.index(c) if c in CATEGORIES else len(CATEGORIES)


# --- harness runs ----------------------------------------------------------------

_HIER: dict[tuple, Hierarchy] = {}
_JARS: dict[str, tuple[zipfile.ZipFile, dict[str, list[str]]]] = {}


def _file_task(args) -> tuple[str, dict]:
    jar, cp, outdir, tops, rel = args
    hier = _HIER.get(tuple(cp)) or _HIER.setdefault(tuple(cp), Hierarchy(cp))
    if jar not in _JARS:
        z = zipfile.ZipFile(jar)
        index: dict[str, list[str]] = {}
        for n in z.namelist():
            if n.endswith(".class"):
                index.setdefault(n[:-6].split("$")[0], []).append(n)
        _JARS[jar] = (z, index)
    z, index = _JARS[jar]
    original = {}
    for t in tops:
        for n in index.get(t.replace(".", "/"), []):
            cf = parse_class(z.read(n))
            if cf.name == t.replace(".", "/") or cf.own_inner_info() or cf.enclosing_method:
                original[cf.name] = cf
    recompiled = {}
    for p in Path(outdir).rglob("*.class"):
        cf = parse_class(p.read_bytes())
        recompiled[cf.name] = cf
    return rel, compare_classes(original, recompiled, hier)


def project_tasks(pdir: Path) -> tuple[list[tuple], int]:
    """The comparison tasks of one project of a harness run and the count of its files that did not compile."""
    info = json.loads((pdir / "run.json").read_text())
    files = json.loads((pdir / "classes.json").read_text())
    dec = pdir / "decompiled"
    tasks, cp, not_compiled = [], [], 0
    for shard in sorted((pdir / "recompile" / "jobs-r0").glob("shard*.tsv")):
        if not shard.name[5:-4].isdigit():
            continue
        for line in shard.read_text().splitlines():
            f = line.split("\t")
            if f[0] == "#CP":
                cp = [p for p in f[1].split(os.pathsep) if p.endswith(".jar")]
                continue
            rel = str(Path(f[1]).relative_to(dec))
            rec = files.get(rel)
            if rec is None:
                continue
            if not rec.get("raw_compiles"):
                not_compiled += 1
                continue
            tasks.append((info["input_jar"], cp, f[2], rec["tops"], rel))
    return tasks, not_compiled


def project_dirs(paths: list[str]) -> list[Path]:
    out = []
    for p in map(Path, paths):
        if (p / "classes.json").exists():
            out.append(p)
        elif p.is_dir():
            out += [s for s in sorted(p.iterdir()) if (s / "classes.json").exists()]
    return out


def summarize(name: str, results: list[dict], not_compiled: int = 0) -> dict:
    s = {"project": name, "compared": 0, "equal": 0, "differing": 0, "files_not_compiled": not_compiled,
         "first": Counter(), "any": Counter()}
    for r in results:
        s["compared"] += r["compared"]
        s["equal"] += r["equal"]
        for m in r["methods"]:
            if m["method"] == "<header>":
                s["any"]["class header differs"] += 1
                continue
            s["differing"] += 1
            s["first"][m["categories"][0]] += 1
            s["any"].update(m["categories"])
    return s


def print_summary(rows: list[dict]) -> None:
    print(f"{'project':18s} {'compared':>9s} {'equal':>9s} {'differ':>7s}  categories (first / any)")
    for s in rows:
        cats = ", ".join(f"{c} {s['first'].get(c, 0)}/{n}" for c, n in sorted(s["any"].items(), key=lambda kv: _rank(kv[0])))
        print(f"{s['project']:18s} {s['compared']:9d} {s['equal']:9d} {s['differing']:7d}  {cats}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="python3 -m scripts.javabench.refs", description=__doc__.split("\n\n")[0])
    ap.add_argument("runs", nargs="*", help="harness run directories, or one project's directory of a run")
    ap.add_argument("--classes", nargs=2, metavar=("ORIGINAL", "RECOMPILED"), help="compare two class trees")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--out", type=Path, help="write every differing method as JSON lines here")
    ap.add_argument("--quiet", action="store_true", help="print the summary only")
    args = ap.parse_args(argv)
    out = args.out.open("w") if args.out else None
    rows = []

    def emit(project: str, rel: str, res: dict) -> None:
        for m in res["methods"]:
            if out:
                out.write(json.dumps({"project": project, "file": rel, **m}) + "\n")
            if not args.quiet:
                print(f"DIFF\t{project}\t{','.join(m['categories'])}\t{m['class']}#{m['method']}\t"
                      f"{' | '.join(m['details'])[:300]}")

    if args.classes:
        load = lambda d: {cf.name: cf for cf in (parse_class(p.read_bytes()) for p in Path(d).rglob("*.class"))}
        original, recompiled = load(args.classes[0]), load(args.classes[1])
        res = compare_classes(original, recompiled, Hierarchy([], extra=original))
        emit("classes", "", res)
        rows.append(summarize("classes", [res]))
    with ProcessPoolExecutor(max_workers=args.jobs) as ex:
        for pdir in project_dirs(args.runs):
            tasks, not_compiled = project_tasks(pdir)
            results = []
            for rel, res in ex.map(_file_task, tasks, chunksize=8):
                emit(pdir.name, rel, res)
                results.append(res)
            rows.append(summarize(pdir.name, results, not_compiled))
            print(f"[refs] {pdir}: {len(tasks)} files compared, {not_compiled} did not compile", file=sys.stderr, flush=True)
    if out:
        out.close()
    print_summary(rows)
    return 0


if __name__ == "__main__":
    sys.exit(main())
