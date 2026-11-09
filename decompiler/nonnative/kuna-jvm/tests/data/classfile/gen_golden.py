#!/usr/bin/env python3
"""Golden vectors for the class-file reader, with `javap` as the oracle.

    gen_golden.py compile            # javac the corpus under every config into classes/
    gen_golden.py golden [DIR [OUT]] # javap -v -p -c every .class under DIR, write .golden files

`golden` writes `<name>.golden` next to each class (or under OUT, mirroring DIR).
The golden text is what `tests/struct_classfile.rs` renders from the parsed
`StructClass`, in Vineflower's normalized instruction form: iconst/xload_n
fold into bipush/xload n, goto_w/jsr_w into goto/jsr, wide forms keep their
base mnemonic plus `wide`, jump and switch operands are relative offsets
followed by the absolute targets the parsed destinations resolve to.
Only the attributes Vineflower reads are listed; a method's Code
sub-attributes are merged into its own map exactly as `StructMethod.create`
does (a Code-level attribute replaces a method-level one of the same name).
"""

import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "src")
CLASSES = os.path.join(HERE, "classes")

CONFIGS = [
    ("r8g", ["--release", "8", "-g"], ["all", "wide"]),
    ("r11n", ["--release", "11", "-g:none"], ["all", "mod11"]),
    ("r17p", ["--release", "17", "-g", "-parameters"], ["all"]),
    ("r21g", ["--release", "21", "-g"], ["all", "j21", "wide"]),
    ("r21np", ["--release", "21", "-g:none", "-parameters"], ["j21"]),
]

KNOWN = {
    "Code", "InnerClasses", "Signature", "AnnotationDefault", "Exceptions", "EnclosingMethod",
    "RuntimeVisibleAnnotations", "RuntimeInvisibleAnnotations",
    "RuntimeVisibleParameterAnnotations", "RuntimeInvisibleParameterAnnotations",
    "RuntimeVisibleTypeAnnotations", "RuntimeInvisibleTypeAnnotations",
    "LocalVariableTable", "LocalVariableTypeTable", "ConstantValue", "BootstrapMethods",
    "Synthetic", "Deprecated", "LineNumberTable", "MethodParameters", "Module", "Record",
    "PermittedSubclasses", "SourceFile", "NestHost",
}
ALL_ATTRS = KNOWN | {
    "StackMapTable", "NestMembers", "ModulePackages", "ModuleMainClass", "ModuleTarget",
    "ModuleHashes", "ModuleResolution", "SourceDebugExtension", "SourceID", "CompilationID",
    "CharacterRangeTable", "StackMap",
}

FOLD = {}
for i, v in enumerate(["m1", "0", "1", "2", "3", "4", "5"]):
    FOLD["iconst_" + v] = ("bipush", [i - 1])
for p in "ilfda":
    for n in range(4):
        FOLD[f"{p}load_{n}"] = (f"{p}load", [n])
        FOLD[f"{p}store_{n}"] = (f"{p}store", [n])
JUMPS = {"ifeq", "ifne", "iflt", "ifge", "ifgt", "ifle", "if_icmpeq", "if_icmpne", "if_icmplt",
         "if_icmpge", "if_icmpgt", "if_icmple", "if_acmpeq", "if_acmpne", "goto", "jsr",
         "ifnull", "ifnonnull", "goto_w", "jsr_w"}
CP_REF = {"getstatic", "putstatic", "getfield", "putfield", "invokevirtual", "invokespecial",
          "invokestatic", "new", "anewarray", "checkcast", "instanceof", "ldc", "ldc_w", "ldc2_w"}
ATYPE = {"boolean": 4, "char": 5, "float": 6, "double": 7, "byte": 8, "short": 9, "int": 10, "long": 11}
TARGETS = {
    "CLASS_TYPE_PARAMETER": 0x00, "METHOD_TYPE_PARAMETER": 0x01, "CLASS_EXTENDS": 0x10,
    "CLASS_TYPE_PARAMETER_BOUND": 0x11, "METHOD_TYPE_PARAMETER_BOUND": 0x12, "FIELD": 0x13,
    "METHOD_RETURN": 0x14, "METHOD_RECEIVER": 0x15, "METHOD_FORMAL_PARAMETER": 0x16,
    "THROWS": 0x17, "LOCAL_VARIABLE": 0x40, "RESOURCE_VARIABLE": 0x41,
    "EXCEPTION_PARAMETER": 0x42, "INSTANCEOF": 0x43, "NEW": 0x44,
    "CONSTRUCTOR_REFERENCE": 0x45, "METHOD_REFERENCE": 0x46, "CAST": 0x47,
    "CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT": 0x48, "METHOD_INVOCATION_TYPE_ARGUMENT": 0x49,
    "CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT": 0x4A, "METHOD_REFERENCE_TYPE_ARGUMENT": 0x4B,
}
MODIFIERS = {"public": 0x1, "private": 0x2, "protected": 0x4, "static": 0x8, "final": 0x10,
             "synchronized": 0x20, "volatile": 0x40, "transient": 0x80, "native": 0x100,
             "abstract": 0x400, "strictfp": 0x800, "synthetic": 0x1000, "mandated": 0x8000,
             "default": 0, "sealed": 0, "non-sealed": 0}


def run_javac():
    for name, opts, dirs in CONFIGS:
        out = os.path.join(CLASSES, name)
        os.makedirs(out, exist_ok=True)
        for d in dirs:
            base = os.path.join(SRC, d)
            files = []
            for root, _, fs in os.walk(base):
                files += [os.path.join(root, f) for f in fs if f.endswith(".java")]
            target = out if d != "mod11" else os.path.join(out, "mod11")
            os.makedirs(target, exist_ok=True)
            subprocess.run(["javac", "-nowarn", "-XDsuppressNotes", *opts, "-d", target, *sorted(files)],
                           check=True, cwd=base)


def split_classes(text):
    blocks = re.split(r"^Classfile (.*)\n", text, flags=re.M)
    for i in range(1, len(blocks), 2):
        yield blocks[i].strip(), blocks[i + 1]


class Pool:
    def __init__(self, lines):
        self.e = {}
        for line in lines:
            m = re.match(r"\s*#(\d+) = (\w+)\s+(.*?)\s*$", line)
            if not m:
                continue
            idx, kind, rest = int(m.group(1)), m.group(2), m.group(3)
            comment = None
            if "//" in rest and kind != "Utf8":
                rest, comment = rest.split("//", 1)
                rest, comment = rest.strip(), comment.strip()
            self.e[idx] = (kind, rest, comment)

    def utf(self, i):
        return self.e[i][1]

    def cls(self, i):
        kind, rest, _ = self.e[i]
        name = self.utf(int(rest[1:]))
        return name

    def named(self, i):
        kind, rest, _ = self.e[i]
        return self.utf(int(rest[1:]))

    def nat(self, i):
        _, rest, _ = self.e[i]
        a, b = rest.split(":")
        return self.utf(int(a[1:])), self.utf(int(b[1:]))

    def ref(self, i):
        kind, rest, _ = self.e[i]
        if kind in ("Fieldref", "Methodref", "InterfaceMethodref"):
            a, b = rest.split(".")
            n, d = self.nat(int(b[1:]))
            return f"{self.cls(int(a[1:]))}.{n}:{d}"
        if kind in ("InvokeDynamic", "Dynamic"):
            a, b = rest.split(":")
            n, d = self.nat(int(b[1:]))
            return f"#{a[1:]}:{n}:{d}"
        if kind == "Class":
            return self.cls(i)
        if kind == "MethodHandle":
            k, r = rest.split(":")
            return f"{k}:{self.ref(int(r[1:]))}"
        return None


def strip_generics(s):
    out, depth = [], 0
    i = 0
    while i < len(s):
        c = s[i]
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
            if depth == 0 and i + 1 < len(s) and s[i + 1] == ".":
                out.append("$")
                i += 2
                continue
        elif depth == 0:
            out.append(c)
        i += 1
    return "".join(out)


def split_top(s, sep=","):
    parts, depth, cur = [], 0, ""
    for c in s:
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
        if c == sep and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += c
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts]


def interfaces_of(decl):
    s = decl.strip()
    is_iface = re.search(r"\binterface\b", s) is not None
    m = re.search(r"\b(class|interface|enum|record)\b\s+", s)
    if not m:
        return []
    rest = s[m.end():]
    depth, i = 0, 0
    while i < len(rest):
        c = rest[i]
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
        elif c == " " and depth == 0:
            break
        i += 1
    tail = rest[i:]
    key = " extends " if is_iface else " implements "
    idx = find_top(tail, key)
    if idx < 0:
        return []
    clause = tail[idx + len(key):]
    names = [strip_generics(p).strip().replace(".", "/") for p in split_top(clause)]
    return names


def find_top(s, key):
    depth = 0
    for i in range(len(s)):
        c = s[i]
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
        elif depth == 0 and s.startswith(key, i):
            return i
    return -1


def member_name(decl, this_class):
    d = decl.strip().rstrip(";")
    if d == "static {}":
        return "<clinit>"
    if "(" not in strip_generics(d):
        tokens = strip_generics(d).split()
        return tokens[-1]
    flat = strip_generics(d)
    head = flat[:flat.index("(")].split()
    name = head[-1]
    rest = [t for t in head[:-1] if t not in MODIFIERS]
    if not rest:
        return "<init>"
    return name


def indent_of(line):
    return len(line) - len(line.lstrip(" "))


BODY_LINE = re.compile(r"\s*(-?\d+|default): |\s*}\s*$")
INSTR = re.compile(r"\s*(\d+): ([a-z][a-z0-9_]*)\s*(.*)$")


def sections(lines, indent):
    """Split `lines` into (header, body) groups whose header sits at `indent`."""
    out = []
    for line in lines:
        if line.strip() == "":
            if out:
                out[-1][1].append(line)
            continue
        if indent_of(line) == indent and not BODY_LINE.match(line):
            out.append((line, []))
        elif out:
            out[-1][1].append(line)
    return out


def attr_name(header):
    m = re.match(r"\s*([A-Za-z]+):", header)
    if m and m.group(1) in ALL_ATTRS:
        return m.group(1)
    return None


def anno_lines(name, body, prefix):
    out = []
    for line in body:
        m = re.match(r"\s*(\d+): (#\d+\(.*?\))\s*$", line)
        if m and indent_of(line) == indent_of(body[0]):
            out.append(f"{prefix}anno {name} {m.group(2)}")
    return out


def tanno_lines(name, body, prefix):
    out = []
    base = indent_of(body[0]) if body else 0
    for line in body:
        if indent_of(line) != base:
            continue
        m = re.match(r"\s*\d+: (#\d+\(.*?\)): (.*)$", line)
        if not m:
            continue
        anno, rest = m.group(1), m.group(2)
        loc = re.search(r"location=\[(.*?)\]", rest)
        path = []
        if loc:
            for p in [x.strip() for x in loc.group(1).split(",")]:
                if p == "ARRAY":
                    path.append("0:0")
                elif p == "INNER_TYPE":
                    path.append("1:0")
                elif p == "WILDCARD":
                    path.append("2:0")
                else:
                    a = re.match(r"TYPE_ARGUMENT\((\d+)\)", p)
                    path.append(f"3:{a.group(1)}")
            rest = rest[:loc.start()].rstrip(", ")
        kind = rest.split(",")[0].strip()
        t = TARGETS[kind]
        target = t << 24
        kv = dict(re.findall(r"(\w+)=(\d+)", rest))
        if kind in ("CLASS_TYPE_PARAMETER", "METHOD_TYPE_PARAMETER", "METHOD_FORMAL_PARAMETER"):
            target |= int(kv["param_index"])
        elif kind in ("CLASS_TYPE_PARAMETER_BOUND", "METHOD_TYPE_PARAMETER_BOUND"):
            target |= (int(kv["param_index"]) << 8) | int(kv["bound_index"])
        elif kind in ("CLASS_EXTENDS", "THROWS"):
            target |= int(kv["type_index"])
        elif kind == "EXCEPTION_PARAMETER":
            target |= int(kv["exception_index"])
        elif kind in ("INSTANCEOF", "NEW", "CONSTRUCTOR_REFERENCE", "METHOD_REFERENCE"):
            target |= int(kv["offset"])
        out.append(f"{prefix}tanno {name} {target & 0xffffffff:08x} [{' '.join(path)}] {anno}")
    return out


def panno_lines(name, body, prefix):
    out = []
    param = -1
    for line in body:
        m = re.match(r"\s*parameter (\d+):", line)
        if m:
            param = int(m.group(1))
            continue
        m = re.match(r"\s*(\d+): (#\d+\(.*?\))\s*$", line)
        if m and param >= 0 and indent_of(line) == indent_of(body[0]) + 2:
            out.append(f"{prefix}panno {name} {param} {m.group(2)}")
    return out


def code_lines(body, pool, this_class, prefix):
    out = []
    locals_ = None
    instrs = []
    handlers = []
    lines_tab = []
    lvt, lvtt = [], []
    attrs = {}
    i = 0
    base = indent_of(body[0]) if body else 6
    in_attrs = False
    while i < len(body):
        line = body[i]
        s = line.strip()
        if s.startswith("stack="):
            locals_ = int(re.search(r"locals=(\d+)", s).group(1))
            i += 1
            continue
        m = INSTR.match(line)
        if m and not in_attrs:
            off, op, rest = int(m.group(1)), m.group(2), m.group(3)
            if op in ("tableswitch", "lookupswitch"):
                cases = []
                i += 1
                while body[i].strip() != "}":
                    k, v = body[i].strip().split(":")
                    cases.append((k.strip(), int(v)))
                    i += 1
                instrs.append((off, op, rest, cases))
            else:
                instrs.append((off, op, rest, None))
            i += 1
            continue
        if indent_of(line) == base and s == "Exception table:":
            i += 2
            while i < len(body) and re.match(r"\s+\d+\s+\d+\s+\d+\s+", body[i]):
                f = body[i].split()
                typ = "any" if f[3] == "any" else f[4]
                handlers.append((int(f[0]), int(f[1]), int(f[2]), typ))
                i += 1
            continue
        if indent_of(line) == base and attr_name(line):
            in_attrs = True
            name = attr_name(line)
            j = i + 1
            sub = []
            while j < len(body) and (body[j].strip() == "" or indent_of(body[j]) > base):
                sub.append(body[j])
                j += 1
            sub = [x for x in sub if x.strip()]
            attrs[name] = sub
            if name == "LineNumberTable":
                for x in sub:
                    lm = re.match(r"\s*line (\d+): (\d+)", x)
                    lines_tab.append((int(lm.group(2)), int(lm.group(1))))
            elif name in ("LocalVariableTable", "LocalVariableTypeTable"):
                rows = []
                for x in sub[1:]:
                    f = x.split()
                    rows.append((int(f[2]), int(f[0]), int(f[1]), f[3], f[4]))
                (lvt if name == "LocalVariableTable" else lvtt).extend(rows)
            i = j
            continue
        i += 1
    return locals_, instrs, handlers, lines_tab, lvt, lvtt, attrs


def render_instrs(instrs, pool, prefix):
    out = []
    for off, op, full, cases in instrs:
        rest = full.split("//")[0].strip()
        wide = False
        name = op
        ops = []
        extra = []
        if op in FOLD:
            name, ops = FOLD[op]
        elif op.endswith("_w") and op not in ("ldc_w", "ldc2_w", "goto_w", "jsr_w"):
            wide = True
            name = op[:-2]
        if op in ("goto_w", "jsr_w"):
            name = op[:-2]
        if name in ("bipush", "sipush") and not ops:
            ops = [int(rest)]
        elif name in JUMPS or op in JUMPS:
            tgt = int(rest)
            ops = [tgt - off]
            extra.append(f"-> {tgt}")
        elif name in CP_REF:
            idx = int(rest.lstrip("#"))
            ops = [idx]
            r = pool.ref(idx)
            if r is not None and name not in ("ldc", "ldc_w", "ldc2_w"):
                extra.append(r)
        elif name == "invokeinterface":
            a, b = [x.strip() for x in rest.split(",")]
            idx = int(a.lstrip("#"))
            ops = [idx, int(b)]
            extra.append(pool.ref(idx))
        elif name == "invokedynamic":
            a = rest.split(",")[0].strip()
            idx = int(a.lstrip("#"))
            ops = [idx]
            extra.append(pool.ref(idx))
        elif name == "multianewarray":
            a, b = [x.strip() for x in rest.split(",")]
            idx = int(a.lstrip("#"))
            ops = [idx, int(b)]
            extra.append(pool.ref(idx))
        elif name == "newarray":
            ops = [ATYPE[rest.strip()]]
        elif name == "iinc":
            a, b = [x.strip() for x in rest.split(",")]
            ops = [int(a), int(b)]
        elif name in ("tableswitch", "lookupswitch"):
            default = [v for k, v in cases if k == "default"][0]
            body = [(k, v) for k, v in cases if k != "default"]
            if name == "tableswitch":
                m = re.search(r"//\s*(-?\d+) to (-?\d+)", full)
                low = int(body[0][0]) if body else None
                high = int(body[-1][0]) if body else None
                if m:
                    low, high = int(m.group(1)), int(m.group(2))
                ops = [default - off, low, high] + [v - off for _, v in body]
            else:
                ops = [default - off, len(body)]
                for k, v in body:
                    ops += [int(k), v - off]
            extra.append(f"-> {default} [" + " ".join(f"{k}:{v}" for k, v in body) + "]")
        elif rest:
            try:
                ops = [int(x.strip()) for x in rest.split(",")]
            except ValueError:
                ops = [rest]
        line = f"{prefix}{off}: {name}"
        if wide:
            line += " wide"
        if ops:
            line += " " + " ".join(str(x) for x in ops)
        if extra:
            line += " | " + " | ".join(extra)
        out.append(line)
    return out


def golden_of(path, text):
    lines = text.split("\n")
    out = []
    try:
        pool_start = lines.index("Constant pool:")
    except ValueError:
        pool_start = None
    brace = next(i for i, l in enumerate(lines) if l == "{")
    end = max(i for i, l in enumerate(lines) if l == "}")
    head = lines[:pool_start if pool_start is not None else brace]
    pool = Pool(lines[pool_start + 1:brace] if pool_start is not None else [])
    decl = next(l for l in head if not l.startswith(" ") and l.strip())
    hv = {}
    for l in head:
        m = re.match(r"\s*(minor version|major version|flags|this_class|super_class|interfaces): (.*)$", l)
        if m:
            hv[m.group(1)] = m.group(2)
    this_class = pool.cls(int(re.match(r"#(\d+)", hv["this_class"]).group(1)))
    sup = int(re.match(r"#(\d+)", hv["super_class"]).group(1))
    super_name = pool.cls(sup) if sup else "-"
    n_ifaces = int(re.match(r"(\d+)", hv["interfaces"]).group(1))
    flags = int(re.search(r"\(0x([0-9a-f]+)\)", hv["flags"]).group(1), 16)
    out.append(f"class {this_class}")
    out.append(f"version {hv['major version']}.{hv['minor version']}")
    out.append(f"flags 0x{flags:04x}")
    out.append(f"super {super_name}")
    ifaces = interfaces_of(decl) if n_ifaces else []
    if len(ifaces) != n_ifaces:
        ifaces = ["?"] * n_ifaces
    out.append(f"interfaces {n_ifaces}" + "".join(" " + x for x in ifaces))

    members = sections(lines[brace + 1:end], 2)
    tail = sections(lines[end + 1:], 0)
    out += attribute_lines(tail, pool, this_class, "", is_class=True)

    for header, body in members:
        body = [b for b in body]
        props = {}
        for b in body:
            m = re.match(r"\s{4}(descriptor|flags): (.*)$", b)
            if m:
                props[m.group(1)] = m.group(2)
        name = member_name(header, this_class)
        desc = props["descriptor"]
        is_method = desc.startswith("(")
        mflags = int(re.search(r"\(0x([0-9a-f]+)\)", props["flags"]).group(1), 16)
        out.append(f"{'method' if is_method else 'field'} {name} {desc}")
        out.append(f" flags 0x{mflags:04x}")
        secs = sections(body, 4)
        out += attribute_lines(secs, pool, this_class, " ", is_class=False)
    return "\n".join(out) + "\n"


def attribute_lines(secs, pool, this_class, prefix, is_class):
    present = {}
    order = []
    for header, body in secs:
        name = attr_name(header)
        if name is None:
            continue
        present[name] = (header, [b for b in body if b.strip()])
        order.append(name)
    effective = {k: v for k, v in present.items() if k in KNOWN and k != "Code"}
    code = None
    if "Code" in present:
        h, body = present["Code"]
        code = code_lines(body, pool, this_class, prefix)
        for k, v in code[6].items():
            if k in KNOWN:
                effective[k] = (k + ":", v)
    out = [f"{prefix}attrs " + ",".join(sorted(effective))]
    for name in sorted(effective):
        header, body = effective[name]
        if name == "Signature" and present.get("Signature") is effective.get("Signature"):
            m = re.search(r"#(\d+)", header)
            out.append(f"{prefix}signature {pool.utf(int(m.group(1)))}")
        elif name == "SourceFile":
            out.append(f"{prefix}sourcefile {header.split(':', 1)[1].strip().strip(chr(34))}")
        elif name == "InnerClasses":
            for b in body:
                m = re.match(r"\s*((?:[a-z]+ )*)(?:#(\d+)= )?#(\d+)(?: of #(\d+))?;", b)
                mods = sum(MODIFIERS.get(w, 0) for w in m.group(1).split())
                simple = pool.utf(int(m.group(2))) if m.group(2) else "-"
                inner = pool.cls(int(m.group(3)))
                outer = pool.cls(int(m.group(4))) if m.group(4) else "-"
                out.append(f"{prefix}innerclass {inner} {outer} {simple} 0x{mods:04x}")
        elif name == "EnclosingMethod":
            m = re.search(r"#(\d+)\.#(\d+)", header)
            cls = pool.cls(int(m.group(1)))
            meth = pool.nat(int(m.group(2)))[0] if int(m.group(2)) else "-"
            out.append(f"{prefix}enclosing {cls} {meth}")
        elif name == "NestHost":
            out.append(f"{prefix}nesthost {header.split('class', 1)[1].strip()}")
        elif name == "PermittedSubclasses":
            out.append(f"{prefix}permitted " + " ".join(b.strip() for b in body))
        elif name == "Exceptions":
            names = []
            for b in body:
                names += [x.strip().replace(".", "/") for x in b.strip()[len("throws "):].split(",")]
            out.append(f"{prefix}exceptions " + " ".join(names))
        elif name == "MethodParameters":
            items = []
            for b in body[1:]:
                row = b.strip()
                if row.startswith("<no name>"):
                    pname, words = "-", row[len("<no name>"):].split()
                else:
                    pname, words = row.split()[0], row.split()[1:]
                items.append(f"{pname}:0x{sum(MODIFIERS[w] for w in words):04x}")
            out.append(f"{prefix}params " + " ".join(items))
        elif name == "AnnotationDefault":
            m = re.match(r"\s*default_value: (.*)$", body[0])
            out.append(f"{prefix}default {m.group(1).strip()}")
        elif name in ("RuntimeVisibleAnnotations", "RuntimeInvisibleAnnotations"):
            out += anno_lines(name, body, prefix)
        elif name in ("RuntimeVisibleParameterAnnotations", "RuntimeInvisibleParameterAnnotations"):
            out += panno_lines(name, body, prefix)
        elif name in ("RuntimeVisibleTypeAnnotations", "RuntimeInvisibleTypeAnnotations"):
            out += tanno_lines(name, body, prefix)
        elif name == "BootstrapMethods":
            cur = None
            for b in body:
                m = re.match(r"\s*(\d+): #(\d+) ", b)
                if m and indent_of(b) == indent_of(body[0]):
                    if cur:
                        out.append(cur[0] + f" args {cur[1]}")
                    cur = [f"{prefix}bootstrap {m.group(1)} {pool.ref(int(m.group(2)))}", 0]
                elif re.match(r"\s*#\d+", b) and cur:
                    cur[1] += 1
            if cur:
                out.append(cur[0] + f" args {cur[1]}")
        elif name == "Record":
            comps = sections(body, indent_of(body[0])) if body else []
            for h, cb in comps:
                cname = h.strip().rstrip(";").split()[-1]
                cdesc = next(x for x in cb if x.strip().startswith("descriptor:")).split(":", 1)[1].strip()
                csecs = sections(cb, indent_of(h) + 2)
                cattrs = sorted(n for n in (attr_name(x[0]) for x in csecs) if n in KNOWN)
                out.append(f"{prefix}record {cname} {cdesc} attrs {','.join(cattrs)}")
        elif name == "Module":
            out += module_lines(body, pool, prefix)
    if code is not None:
        locals_, instrs, handlers, lines_tab, lvt, lvtt, _ = code
        out.append(f"{prefix}code locals={locals_}")
        p2 = prefix + " "
        out += render_instrs(instrs, pool, p2)
        for f, t, h, typ in handlers:
            out.append(f"{p2}handler {f} {t} {h} {typ}")
        for pc, ln in lines_tab:
            out.append(f"{p2}line {pc} {ln}")
        rows = sorted(lvt, key=lambda r: (r[0], r[1], r[2]))
        sigs = [None] * len(rows)
        for t in sorted(lvtt, key=lambda r: (r[0], r[1], r[2])):
            for k, r in enumerate(rows):
                if r[:3] == t[:3]:
                    sigs[k] = t[4]
                    break
        for r, s in zip(rows, sigs):
            out.append(f"{p2}lvt {r[0]} {r[1]} {r[2]} {r[3]} {r[4]}" + (f" {s}" if s else ""))
    return out


def module_lines(body, pool, prefix):
    rows = [(indent_of(b), b.split("//")[0].strip()) for b in body if b.strip()]
    name_i, flags = rows[0][1].lstrip("#").split(",")
    ver = int(rows[1][1].lstrip("#"))
    head = f"{prefix}module {pool.named(int(name_i))} 0x{int(flags, 16):04x} {pool.utf(ver) if ver else '-'}"
    return module_targets(rows, pool, prefix, head)


def module_targets(rows, pool, prefix, head):
    out = [head]
    k = 2
    n = int(rows[k][1]); k += 1
    for _ in range(n):
        i, f = rows[k][1].lstrip("#").split(","); k += 1
        v = int(rows[k][1].lstrip("#")); k += 1
        out.append(f"{prefix} requires {pool.named(int(i))} 0x{int(f, 16):04x} {pool.utf(v) if v else '-'}")
    for kind in ("exports", "opens"):
        n = int(rows[k][1]); k += 1
        for _ in range(n):
            ind = rows[k][0]
            i, f = rows[k][1].lstrip("#").split(","); k += 1
            tos = []
            while k < len(rows) and rows[k][0] > ind:
                tos.append(pool.named(int(rows[k][1].lstrip("#")))); k += 1
            out.append(f"{prefix} {kind} {pool.named(int(i))} 0x{int(f, 16):04x}" + "".join(" " + t for t in tos))
    n = int(rows[k][1]); k += 1
    for _ in range(n):
        out.append(f"{prefix} uses {pool.cls(int(rows[k][1].lstrip('#')))}"); k += 1
    n = int(rows[k][1]); k += 1
    for _ in range(n):
        ind = rows[k][0]
        iface = pool.cls(int(rows[k][1].lstrip("#"))); k += 1
        impls = []
        while k < len(rows) and rows[k][0] > ind:
            impls.append(pool.cls(int(rows[k][1].lstrip("#")))); k += 1
        out.append(f"{prefix} provides {iface} with" + "".join(" " + x for x in impls))
    return out


def javap_batch(paths):
    res = subprocess.run(["javap", "-v", "-p", "-c", *paths], capture_output=True, text=True)
    if res.returncode != 0 and not res.stdout:
        raise RuntimeError(res.stderr)
    return res.stdout


def golden_dir(root, out_root):
    classes = []
    for dirpath, _, files in os.walk(root):
        classes += [os.path.join(dirpath, f) for f in files if f.endswith(".class")]
    classes.sort()
    done, failed = 0, 0
    for i in range(0, len(classes), 200):
        chunk = classes[i:i + 200]
        text = javap_batch(chunk)
        by_path = dict(split_classes(text))
        for c in chunk:
            block = by_path.get(c)
            target = os.path.join(out_root, os.path.relpath(c, root))[:-len(".class")] + ".golden"
            os.makedirs(os.path.dirname(target), exist_ok=True)
            try:
                g = golden_of(c, block)
                with open(target, "w") as fh:
                    fh.write(g)
                done += 1
            except Exception as e:  # noqa: BLE001
                failed += 1
                print(f"skip {c}: {type(e).__name__}: {e}", file=sys.stderr)
    print(f"golden: {done} written, {failed} skipped", file=sys.stderr)


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "all"
    if cmd in ("compile", "all"):
        run_javac()
    if cmd in ("golden", "all"):
        root = sys.argv[2] if len(sys.argv) > 2 else CLASSES
        out = sys.argv[3] if len(sys.argv) > 3 else root
        golden_dir(root, out)


if __name__ == "__main__":
    main()
