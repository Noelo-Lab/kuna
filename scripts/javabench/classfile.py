"""Minimal, dependency-free JVM class-file reader.

Reads exactly what javabench needs: the constant pool, class/method headers,
the Code attribute (instructions + exception table), LineNumberTable,
LocalVariableTable / LocalVariableTypeTable, Signature, InnerClasses,
EnclosingMethod, SourceFile and BootstrapMethods. Parsing is strict; a
malformed file raises ``ClassFormatError``.
"""

from __future__ import annotations

import struct
from dataclasses import dataclass, field

ACC_PUBLIC = 0x0001
ACC_PRIVATE = 0x0002
ACC_PROTECTED = 0x0004
ACC_STATIC = 0x0008
ACC_FINAL = 0x0010
ACC_SYNCHRONIZED = 0x0020
ACC_BRIDGE = 0x0040
ACC_VARARGS = 0x0080
ACC_NATIVE = 0x0100
ACC_INTERFACE = 0x0200
ACC_ABSTRACT = 0x0400
ACC_SYNTHETIC = 0x1000
ACC_ANNOTATION = 0x2000
ACC_ENUM = 0x4000
ACC_MODULE = 0x8000


class ClassFormatError(Exception):
    pass


def decode_mutf8(data: bytes) -> str:
    """Decode JVM 'modified UTF-8' (C0 80 NULs, CESU-8 surrogate pairs)."""
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        pass
    data = data.replace(b"\xc0\x80", b"\x00")
    text = data.decode("utf-8", errors="surrogatepass")
    try:
        return text.encode("utf-16", "surrogatepass").decode("utf-16")
    except UnicodeDecodeError:
        return text.encode("utf-16", "surrogatepass").decode("utf-16", errors="replace")


# --- opcode table -----------------------------------------------------------
# operand kinds: '' none, 'b' s1, 's' s2, 'c1' cp u1, 'c2' cp u2, 'l' local u1,
# 'br' s2 branch, 'bw' s4 branch, 'iinc', 'tsw', 'lsw', 'ii' invokeinterface,
# 'indy', 'na' newarray, 'mna' multianewarray, 'wide'
_OPS: dict[int, tuple[str, str]] = {}


def _op(code: int, name: str, kind: str = "") -> None:
    _OPS[code] = (name, kind)


for _i, _n in enumerate(
    ["nop", "aconst_null", "iconst_m1", "iconst_0", "iconst_1", "iconst_2", "iconst_3",
     "iconst_4", "iconst_5", "lconst_0", "lconst_1", "fconst_0", "fconst_1", "fconst_2",
     "dconst_0", "dconst_1"]
):
    _op(_i, _n)
_op(16, "bipush", "b")
_op(17, "sipush", "s")
_op(18, "ldc", "c1")
_op(19, "ldc_w", "c2")
_op(20, "ldc2_w", "c2")
for _i, _p in enumerate("ilfda"):
    _op(21 + _i, f"{_p}load", "l")
    for _k in range(4):
        _op(26 + _i * 4 + _k, f"{_p}load_{_k}")
for _i, _n in enumerate(["iaload", "laload", "faload", "daload", "aaload", "baload", "caload", "saload"]):
    _op(46 + _i, _n)
for _i, _p in enumerate("ilfda"):
    _op(54 + _i, f"{_p}store", "l")
    for _k in range(4):
        _op(59 + _i * 4 + _k, f"{_p}store_{_k}")
for _i, _n in enumerate(
    ["iastore", "lastore", "fastore", "dastore", "aastore", "bastore", "castore", "sastore",
     "pop", "pop2", "dup", "dup_x1", "dup_x2", "dup2", "dup2_x1", "dup2_x2", "swap",
     "iadd", "ladd", "fadd", "dadd", "isub", "lsub", "fsub", "dsub", "imul", "lmul", "fmul",
     "dmul", "idiv", "ldiv", "fdiv", "ddiv", "irem", "lrem", "frem", "drem", "ineg", "lneg",
     "fneg", "dneg", "ishl", "lshl", "ishr", "lshr", "iushr", "lushr", "iand", "land", "ior",
     "lor", "ixor", "lxor"]
):
    _op(79 + _i, _n)
_op(132, "iinc", "iinc")
for _i, _n in enumerate(
    ["i2l", "i2f", "i2d", "l2i", "l2f", "l2d", "f2i", "f2l", "f2d", "d2i", "d2l", "d2f",
     "i2b", "i2c", "i2s", "lcmp", "fcmpl", "fcmpg", "dcmpl", "dcmpg"]
):
    _op(133 + _i, _n)
for _i, _n in enumerate(
    ["ifeq", "ifne", "iflt", "ifge", "ifgt", "ifle", "if_icmpeq", "if_icmpne", "if_icmplt",
     "if_icmpge", "if_icmpgt", "if_icmple", "if_acmpeq", "if_acmpne", "goto", "jsr"]
):
    _op(153 + _i, _n, "br")
_op(169, "ret", "l")
_op(170, "tableswitch", "tsw")
_op(171, "lookupswitch", "lsw")
for _i, _n in enumerate(["ireturn", "lreturn", "freturn", "dreturn", "areturn", "return"]):
    _op(172 + _i, _n)
for _i, _n in enumerate(
    ["getstatic", "putstatic", "getfield", "putfield", "invokevirtual", "invokespecial",
     "invokestatic"]
):
    _op(178 + _i, _n, "c2")
_op(185, "invokeinterface", "ii")
_op(186, "invokedynamic", "indy")
_op(187, "new", "c2")
_op(188, "newarray", "na")
_op(189, "anewarray", "c2")
_op(190, "arraylength")
_op(191, "athrow")
_op(192, "checkcast", "c2")
_op(193, "instanceof", "c2")
_op(194, "monitorenter")
_op(195, "monitorexit")
_op(196, "wide", "wide")
_op(197, "multianewarray", "mna")
_op(198, "ifnull", "br")
_op(199, "ifnonnull", "br")
_op(200, "goto_w", "bw")
_op(201, "jsr_w", "bw")

OPCODES = _OPS
NEWARRAY_TYPES = {4: "boolean", 5: "char", 6: "float", 7: "double", 8: "byte", 9: "short", 10: "int", 11: "long"}


@dataclass
class Insn:
    pc: int
    op: str
    args: tuple = ()
    targets: tuple = ()  # absolute branch-target pcs (in order)


@dataclass
class LocalVar:
    start_pc: int
    length: int
    name: str
    descriptor: str
    index: int
    signature: str | None = None


@dataclass
class ExceptionEntry:
    start_pc: int
    end_pc: int
    handler_pc: int
    catch_type: str | None


@dataclass
class Code:
    max_stack: int
    max_locals: int
    code: bytes
    exceptions: list[ExceptionEntry]
    lines: list[tuple[int, int]]  # (start_pc, line)
    locals: list[LocalVar]

    def line_numbers(self) -> list[int]:
        return sorted({ln for _, ln in self.lines})


@dataclass
class Method:
    access: int
    name: str
    descriptor: str
    signature: str | None = None
    code: Code | None = None

    @property
    def is_synthetic(self) -> bool:
        return bool(self.access & ACC_SYNTHETIC)

    @property
    def is_bridge(self) -> bool:
        return bool(self.access & ACC_BRIDGE)

    @property
    def is_static(self) -> bool:
        return bool(self.access & ACC_STATIC)


@dataclass
class InnerClassInfo:
    inner: str
    outer: str | None
    simple_name: str | None
    access: int


@dataclass
class ClassFile:
    major: int
    minor: int
    access: int
    name: str  # internal name, e.g. com/foo/Bar$Baz
    super_name: str | None
    interfaces: list[str]
    methods: list[Method]
    field_names: list[str] = field(default_factory=list)
    source_file: str | None = None
    signature: str | None = None
    inner_classes: list[InnerClassInfo] = field(default_factory=list)
    enclosing_method: tuple[str, str | None, str | None] | None = None  # (class, name, desc)
    nest_host: str | None = None
    cp: "ConstantPool | None" = None
    bootstrap: list[tuple[int, list[int]]] = field(default_factory=list)

    @property
    def is_synthetic(self) -> bool:
        return bool(self.access & ACC_SYNTHETIC)

    def own_inner_info(self) -> InnerClassInfo | None:
        for ic in self.inner_classes:
            if ic.inner == self.name:
                return ic
        return None

    @property
    def is_anonymous(self) -> bool:
        ic = self.own_inner_info()
        return ic is not None and ic.simple_name is None

    @property
    def is_local(self) -> bool:
        ic = self.own_inner_info()
        return ic is not None and ic.simple_name is not None and ic.outer is None

    @property
    def dotted(self) -> str:
        return self.name.replace("/", ".")


class ConstantPool:
    __slots__ = ("entries",)

    def __init__(self, entries: list):
        self.entries = entries

    def utf8(self, idx: int) -> str:
        e = self.entries[idx]
        if e is None or e[0] != 1:
            raise ClassFormatError(f"cp#{idx} is not Utf8")
        return e[1]

    def class_name(self, idx: int) -> str:
        e = self.entries[idx]
        if e is None or e[0] != 7:
            raise ClassFormatError(f"cp#{idx} is not Class")
        return self.utf8(e[1])

    def name_and_type(self, idx: int) -> tuple[str, str]:
        e = self.entries[idx]
        return self.utf8(e[1]), self.utf8(e[2])

    def member(self, idx: int) -> tuple[str, str, str]:
        e = self.entries[idx]
        owner = self.class_name(e[1])
        name, desc = self.name_and_type(e[2])
        return owner, name, desc

    def tag(self, idx: int) -> int:
        e = self.entries[idx]
        return e[0] if e else 0

    def describe(self, idx: int) -> str:
        """Layout-independent symbolic rendering of a constant."""
        e = self.entries[idx]
        if e is None:
            return "?"
        tag = e[0]
        if tag == 1:
            return "utf8:" + e[1]
        if tag in (3, 4, 5, 6):
            kind = {3: "int", 4: "float", 5: "long", 6: "double"}[tag]
            return f"{kind}:{e[1]!r}"
        if tag == 7:
            return "class:" + self.utf8(e[1])
        if tag == 8:
            return "str:" + repr(self.utf8(e[1]))
        if tag in (9, 10, 11):
            o, n, d = self.member(idx)
            return f"{o}.{n}:{d}"
        if tag == 12:
            n, d = self.name_and_type(idx)
            return f"{n}:{d}"
        if tag == 15:
            return f"handle{e[1]}:{self.describe(e[2])}"
        if tag == 16:
            return "mtype:" + self.utf8(e[1])
        if tag in (17, 18):
            n, d = self.name_and_type(e[2])
            return f"dyn:{n}:{d}"
        if tag in (19, 20):
            return "modpkg:" + self.utf8(e[1])
        return "?"


def _parse_cp(buf: bytes, off: int) -> tuple[ConstantPool, int]:
    (count,) = struct.unpack_from(">H", buf, off)
    off += 2
    entries: list = [None] * count
    i = 1
    while i < count:
        tag = buf[off]
        off += 1
        if tag == 1:
            (ln,) = struct.unpack_from(">H", buf, off)
            off += 2
            entries[i] = (1, decode_mutf8(buf[off : off + ln]))
            off += ln
        elif tag == 3:
            entries[i] = (3, struct.unpack_from(">i", buf, off)[0])
            off += 4
        elif tag == 4:
            entries[i] = (4, struct.unpack_from(">f", buf, off)[0])
            off += 4
        elif tag == 5:
            entries[i] = (5, struct.unpack_from(">q", buf, off)[0])
            off += 8
            i += 1
        elif tag == 6:
            entries[i] = (6, struct.unpack_from(">d", buf, off)[0])
            off += 8
            i += 1
        elif tag in (7, 8, 16, 19, 20):
            entries[i] = (tag, struct.unpack_from(">H", buf, off)[0])
            off += 2
        elif tag in (9, 10, 11, 12, 17, 18):
            a, b = struct.unpack_from(">HH", buf, off)
            entries[i] = (tag, a, b)
            off += 4
        elif tag == 15:
            kind = buf[off]
            (ref,) = struct.unpack_from(">H", buf, off + 1)
            entries[i] = (15, kind, ref)
            off += 3
        else:
            raise ClassFormatError(f"bad cp tag {tag} at #{i}")
        i += 1
    return ConstantPool(entries), off


def _attrs(buf: bytes, off: int, cp: ConstantPool) -> tuple[list[tuple[str, int, int]], int]:
    (n,) = struct.unpack_from(">H", buf, off)
    off += 2
    out = []
    for _ in range(n):
        ni, ln = struct.unpack_from(">HI", buf, off)
        off += 6
        out.append((cp.utf8(ni), off, ln))
        off += ln
    return out, off


def _parse_code(buf: bytes, off: int, cp: ConstantPool) -> Code:
    max_stack, max_locals, code_len = struct.unpack_from(">HHI", buf, off)
    off += 8
    code = bytes(buf[off : off + code_len])
    off += code_len
    (ne,) = struct.unpack_from(">H", buf, off)
    off += 2
    excs = []
    for _ in range(ne):
        s, e, h, t = struct.unpack_from(">HHHH", buf, off)
        off += 8
        excs.append(ExceptionEntry(s, e, h, cp.class_name(t) if t else None))
    attrs, off = _attrs(buf, off, cp)
    lines: list[tuple[int, int]] = []
    lvars: dict[tuple[int, int, int], LocalVar] = {}
    lvtt: dict[tuple[int, int, int], str] = {}
    for name, aoff, _ln in attrs:
        if name == "LineNumberTable":
            (n,) = struct.unpack_from(">H", buf, aoff)
            for k in range(n):
                pc, line = struct.unpack_from(">HH", buf, aoff + 2 + 4 * k)
                lines.append((pc, line))
        elif name in ("LocalVariableTable", "LocalVariableTypeTable"):
            (n,) = struct.unpack_from(">H", buf, aoff)
            for k in range(n):
                s, ln_, ni, di, idx = struct.unpack_from(">HHHHH", buf, aoff + 2 + 10 * k)
                key = (s, ln_, idx)
                if name == "LocalVariableTable":
                    lvars[key] = LocalVar(s, ln_, cp.utf8(ni), cp.utf8(di), idx)
                else:
                    lvtt[key] = cp.utf8(di)
    for key, sig in lvtt.items():
        if key in lvars:
            lvars[key].signature = sig
    locs = sorted(lvars.values(), key=lambda v: (v.start_pc, v.index))
    return Code(max_stack, max_locals, code, excs, sorted(lines), locs)


def parse_class(buf: bytes) -> ClassFile:
    try:
        return _parse_class(buf)
    except (struct.error, IndexError, KeyError, TypeError) as e:
        raise ClassFormatError(str(e)) from e


def _parse_class(buf: bytes) -> ClassFile:
    if buf[:4] != b"\xca\xfe\xba\xbe":
        raise ClassFormatError("bad magic")
    minor, major = struct.unpack_from(">HH", buf, 4)
    cp, off = _parse_cp(buf, 8)
    access, this_i, super_i, nif = struct.unpack_from(">HHHH", buf, off)
    off += 8
    interfaces = []
    for k in range(nif):
        interfaces.append(cp.class_name(struct.unpack_from(">H", buf, off + 2 * k)[0]))
    off += 2 * nif
    (nfields,) = struct.unpack_from(">H", buf, off)
    off += 2
    field_names = []
    for _ in range(nfields):
        _acc, ni, _di = struct.unpack_from(">HHH", buf, off)
        field_names.append(cp.utf8(ni))
        _a, off = _attrs(buf, off + 6, cp)
    (nmethods,) = struct.unpack_from(">H", buf, off)
    off += 2
    methods = []
    for _ in range(nmethods):
        macc, ni, di = struct.unpack_from(">HHH", buf, off)
        attrs, off = _attrs(buf, off + 6, cp)
        m = Method(macc, cp.utf8(ni), cp.utf8(di))
        for name, aoff, _ln in attrs:
            if name == "Code":
                m.code = _parse_code(buf, aoff, cp)
            elif name == "Signature":
                m.signature = cp.utf8(struct.unpack_from(">H", buf, aoff)[0])
        methods.append(m)
    cattrs, off = _attrs(buf, off, cp)
    cf = ClassFile(
        major=major,
        minor=minor,
        access=access,
        name=cp.class_name(this_i),
        super_name=cp.class_name(super_i) if super_i else None,
        interfaces=interfaces,
        methods=methods,
        field_names=field_names,
        cp=cp,
    )
    for name, aoff, _ln in cattrs:
        if name == "SourceFile":
            cf.source_file = cp.utf8(struct.unpack_from(">H", buf, aoff)[0])
        elif name == "Signature":
            cf.signature = cp.utf8(struct.unpack_from(">H", buf, aoff)[0])
        elif name == "InnerClasses":
            (n,) = struct.unpack_from(">H", buf, aoff)
            for k in range(n):
                ii, oi, ni, acc = struct.unpack_from(">HHHH", buf, aoff + 2 + 8 * k)
                cf.inner_classes.append(
                    InnerClassInfo(
                        cp.class_name(ii),
                        cp.class_name(oi) if oi else None,
                        cp.utf8(ni) if ni else None,
                        acc,
                    )
                )
        elif name == "EnclosingMethod":
            ci, mi = struct.unpack_from(">HH", buf, aoff)
            if mi:
                mn, md = cp.name_and_type(mi)
            else:
                mn = md = None
            cf.enclosing_method = (cp.class_name(ci), mn, md)
        elif name == "NestHost":
            cf.nest_host = cp.class_name(struct.unpack_from(">H", buf, aoff)[0])
        elif name == "BootstrapMethods":
            (n,) = struct.unpack_from(">H", buf, aoff)
            p = aoff + 2
            for _ in range(n):
                ref, na = struct.unpack_from(">HH", buf, p)
                p += 4
                args = list(struct.unpack_from(f">{na}H", buf, p))
                p += 2 * na
                cf.bootstrap.append((ref, args))
    return cf


def decode_code(code: bytes) -> list[Insn]:
    """Decode raw bytecode into instructions with absolute branch targets."""
    out: list[Insn] = []
    pc = 0
    n = len(code)
    while pc < n:
        opc = code[pc]
        if opc not in _OPS:
            raise ClassFormatError(f"bad opcode {opc} at pc {pc}")
        name, kind = _OPS[opc]
        start = pc
        if kind == "":
            out.append(Insn(start, name))
            pc += 1
        elif kind == "b":
            out.append(Insn(start, name, (struct.unpack_from(">b", code, pc + 1)[0],)))
            pc += 2
        elif kind == "s":
            out.append(Insn(start, name, (struct.unpack_from(">h", code, pc + 1)[0],)))
            pc += 3
        elif kind == "c1":
            out.append(Insn(start, name, (code[pc + 1],)))
            pc += 2
        elif kind == "c2":
            out.append(Insn(start, name, (struct.unpack_from(">H", code, pc + 1)[0],)))
            pc += 3
        elif kind == "l":
            out.append(Insn(start, name, (code[pc + 1],)))
            pc += 2
        elif kind == "br":
            off = struct.unpack_from(">h", code, pc + 1)[0]
            out.append(Insn(start, name, (), (start + off,)))
            pc += 3
        elif kind == "bw":
            off = struct.unpack_from(">i", code, pc + 1)[0]
            out.append(Insn(start, name, (), (start + off,)))
            pc += 5
        elif kind == "iinc":
            out.append(Insn(start, name, (code[pc + 1], struct.unpack_from(">b", code, pc + 2)[0])))
            pc += 3
        elif kind == "ii":
            out.append(Insn(start, name, (struct.unpack_from(">H", code, pc + 1)[0],)))
            pc += 5
        elif kind == "indy":
            out.append(Insn(start, name, (struct.unpack_from(">H", code, pc + 1)[0],)))
            pc += 5
        elif kind == "na":
            out.append(Insn(start, name, (code[pc + 1],)))
            pc += 2
        elif kind == "mna":
            out.append(Insn(start, name, (struct.unpack_from(">H", code, pc + 1)[0], code[pc + 3])))
            pc += 4
        elif kind == "tsw":
            p = (pc + 4) & ~3
            default, low, high = struct.unpack_from(">iii", code, p)
            cnt = high - low + 1
            offs = struct.unpack_from(f">{cnt}i", code, p + 12)
            out.append(Insn(start, name, (low, high), tuple([start + default] + [start + o for o in offs])))
            pc = p + 12 + 4 * cnt
        elif kind == "lsw":
            p = (pc + 4) & ~3
            default, npairs = struct.unpack_from(">ii", code, p)
            pairs = struct.unpack_from(f">{2 * npairs}i", code, p + 8)
            keys = tuple(pairs[0::2])
            tg = [start + default] + [start + o for o in pairs[1::2]]
            out.append(Insn(start, name, keys, tuple(tg)))
            pc = p + 8 + 8 * npairs
        elif kind == "wide":
            sub = code[pc + 1]
            sname, _sk = _OPS[sub]
            if sub == 132:
                idx, const = struct.unpack_from(">Hh", code, pc + 2)
                out.append(Insn(start, "iinc", (idx, const)))
                pc += 6
            else:
                idx = struct.unpack_from(">H", code, pc + 2)[0]
                out.append(Insn(start, sname, (idx,)))
                pc += 4
        else:  # pragma: no cover
            raise ClassFormatError(kind)
    return out
