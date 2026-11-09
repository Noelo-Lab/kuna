"""Tree-sitter-java indexing shared by the source side and every decompiler's output.

``index_source(text)`` returns every type declaration (named, local, anonymous,
enum-constant bodies) and every method/constructor declaration, with binary names
for named types, javac-style numbering for anonymous/local types, type-parameter
scopes and line spans. The same function is applied to original source and to
decompiled output, so neither side gets a different parser.
"""

from __future__ import annotations

import re
import warnings
from dataclasses import dataclass, field

with warnings.catch_warnings():
    warnings.simplefilter("ignore")
    from tree_sitter_languages import get_parser

_PARSER = None


def parser():
    global _PARSER
    if _PARSER is None:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            _PARSER = get_parser("java")
    return _PARSER


TYPE_DECLS = {
    "class_declaration": "class",
    "interface_declaration": "interface",
    "enum_declaration": "enum",
    "record_declaration": "record",
    "annotation_type_declaration": "annotation",
}
METHOD_DECLS = {"method_declaration", "constructor_declaration", "compact_constructor_declaration"}


@dataclass
class Param:
    type_text: str
    name: str
    varargs: bool = False
    dims: int = 0  # C-style dims on the name: ``int a[]``


@dataclass
class TypeDecl:
    kind: str  # class/interface/enum/record/annotation/anon/local
    name: str | None
    parent: "TypeDecl | None"
    node: object
    body: object
    start_line: int
    end_line: int
    supertype: str | None = None  # simple name of the created/extended type
    type_params: dict[str, str] = field(default_factory=dict)
    binary: str | None = None
    static_ctx: bool = False  # declared in a static context (no outer instance)
    is_static: bool = False  # member type declared static (or implicitly static)
    anon_index: int = 0
    local_index: int = 0
    enclosing_method: "MethodDecl | None" = None
    record_params: list[Param] | None = None
    methods: list["MethodDecl"] = field(default_factory=list)
    has_error: bool = False

    def scope_type_params(self) -> dict[str, str]:
        out: dict[str, str] = {}
        chain = []
        t: TypeDecl | None = self
        while t is not None:
            chain.append(t)
            t = t.parent
        for t in reversed(chain):
            if t.enclosing_method is not None:
                out.update(t.enclosing_method.type_params)
            out.update(t.type_params)
        return out


@dataclass
class MethodDecl:
    name: str  # "<init>" for constructors
    kind: str  # method / ctor / compact
    owner: TypeDecl
    node: object
    body: object | None
    params: list[Param]
    ret_type: str | None
    type_params: dict[str, str]
    start_line: int
    end_line: int
    has_error: bool
    is_static: bool = False

    def scope_type_params(self) -> dict[str, str]:
        out = self.owner.scope_type_params()
        out.update(self.type_params)
        return out


@dataclass
class SourceIndex:
    package: str
    imports: list[str]
    types: list[TypeDecl]
    methods: list[MethodDecl]
    has_error: bool
    tree: object
    text: bytes


def node_text(n) -> str:
    return n.text.decode("utf-8", errors="replace")


def _line(point) -> int:
    return point[0] + 1


_ANNOT = re.compile(r"@[\w.$]+(\s*\((?:[^()]|\([^()]*\))*\))?")


def _modifiers(n) -> set[str]:
    for c in n.children:
        if c.type == "modifiers":
            return {node_text(x) for x in c.children if x.type not in ("marker_annotation", "annotation")}
    return set()


def type_params_of(n) -> dict[str, str]:
    """``<T extends A & B, U>`` -> {T: 'A', U: 'Object'} (first bound, unparsed text)."""
    tp = n.child_by_field_name("type_parameters")
    out: dict[str, str] = {}
    if tp is None:
        return out
    for c in tp.named_children:
        if c.type != "type_parameter":
            continue
        name = None
        bound = "Object"
        for x in c.named_children:
            if x.type in ("type_identifier", "identifier") and name is None:
                name = node_text(x)
            elif x.type == "type_bound":
                bs = [y for y in x.named_children]
                if bs:
                    bound = node_text(bs[0])
        if name:
            out[name] = bound
    return out


def _params(plist) -> list[Param]:
    out: list[Param] = []
    if plist is None:
        return out
    for c in plist.named_children:
        if c.type == "formal_parameter":
            t = c.child_by_field_name("type")
            nm = c.child_by_field_name("name")
            dims = c.child_by_field_name("dimensions")
            out.append(
                Param(
                    node_text(t) if t is not None else "?",
                    node_text(nm) if nm is not None else "?",
                    False,
                    node_text(dims).count("[") if dims is not None else 0,
                )
            )
        elif c.type == "spread_parameter":
            t = None
            name = "?"
            for x in c.named_children:
                if x.type == "variable_declarator":
                    nn = x.child_by_field_name("name")
                    name = node_text(nn) if nn is not None else node_text(x)
                elif x.type not in ("modifiers",) and t is None:
                    t = x
            out.append(Param(node_text(t) if t is not None else "?", name, True))
    return out


def _strip_generic_args(t: str) -> str:
    out = []
    depth = 0
    for ch in t:
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
        elif depth == 0:
            out.append(ch)
    return "".join(out)


def _simple_type_name(n) -> str | None:
    if n is None:
        return None
    t = _strip_generic_args(_ANNOT.sub("", node_text(n))).strip()
    return t.split(".")[-1].strip() or None


_VARARG_ANNOT = re.compile(rb"(?:@[\w.$]+(?:\s*\((?:[^()]|\([^()]*\))*\))?\s*)+(?=\.\.\.)")


def _blank(m: "re.Match[bytes]") -> bytes:
    return bytes(b if b in (10, 13) else 32 for b in m.group(0))


def sanitize(text: bytes) -> bytes:
    """Offset/line-preserving rewrites for syntax tree-sitter-java cannot parse.

    Type-use annotations directly before a varargs ellipsis
    (``Object @Nullable ... xs``) are blanked (annotations never influence the
    CFG or the normalized types), and a constant-less enum missing its ``;``
    gets one. The same rewrites run on every side; recompilation always uses
    the raw text.
    """
    if b"..." in text and b"@" in text:
        text = _VARARG_ANNOT.sub(_blank, text)
    if b"enum" in text:
        text = _EMPTY_ENUM.sub(_semicolon, text)
    return text


# ``enum X { static int f() {..} }`` (no constants, missing ``;``): javac rejects
# it, and tree-sitter would bury every member in an ERROR node. A blank inside
# the whitespace after ``{`` becomes ``;`` (offsets and lines unchanged); valid
# Java never matches because a modifier keyword cannot be an enum constant.
_EMPTY_ENUM = re.compile(
    rb"(\benum\s+[\w$]+(?:\s*<[^{]*>)?(?:\s+implements\s+[^{;]+)?\s*\{)([ \t\r\n]+)"
    rb"(?=(?:public|private|protected|static|final|abstract|synchronized|native|strictfp|"
    rb"class|interface|enum|record|void|sealed|non-sealed)\b)")


def _semicolon(m: "re.Match[bytes]") -> bytes:
    ws = m.group(2)
    for i, b in enumerate(ws):
        if b in (32, 9):
            return m.group(1) + ws[:i] + b";" + ws[i + 1 :]
    return m.group(0)


def index_source(text: bytes | str) -> SourceIndex:
    if isinstance(text, str):
        text = text.encode("utf-8")
    text = sanitize(text)
    tree = parser().parse(text)
    root = tree.root_node
    package = ""
    imports: list[str] = []
    for c in root.named_children:
        if c.type == "package_declaration":
            for x in c.named_children:
                if x.type in ("scoped_identifier", "identifier"):
                    package = node_text(x)
        elif c.type == "import_declaration":
            imports.append(re.sub(r"\s+", "", node_text(c))[len("import"):].rstrip(";"))
    idx = SourceIndex(package, imports, [], [], root.has_error, tree, text)
    _Walker(idx).walk_program(root)
    return idx


class _Walker:
    def __init__(self, idx: SourceIndex):
        self.idx = idx
        self.anon_counter: dict[int, int] = {}
        self.local_counter: dict[tuple[int, str], int] = {}

    def walk_program(self, root) -> None:
        for c in root.named_children:
            if c.type in TYPE_DECLS:
                self.type_decl(c, None, None, False, member=True)

    # -- declarations --------------------------------------------------------

    def _new_type(self, kind, name, parent, node, body, method, static_ctx) -> TypeDecl:
        t = TypeDecl(
            kind=kind,
            name=name,
            parent=parent,
            node=node,
            body=body,
            start_line=_line(node.start_point),
            end_line=_line(node.end_point),
            static_ctx=static_ctx,
            enclosing_method=method,
            has_error=node.has_error,
        )
        self.idx.types.append(t)
        return t

    def type_decl(self, n, parent: TypeDecl | None, method: MethodDecl | None, static_ctx: bool, member: bool):
        kind = TYPE_DECLS[n.type]
        name_n = n.child_by_field_name("name")
        name = node_text(name_n) if name_n is not None else "?"
        body = n.child_by_field_name("body")
        mods = _modifiers(n)
        local = not member
        t = self._new_type("local" if local else kind, name, parent, n, body, method, static_ctx)
        t.type_params = type_params_of(n)
        t.is_static = (
            "static" in mods or kind in ("interface", "enum", "record", "annotation")
            or (parent is not None and parent.kind in ("interface", "annotation"))
        )
        t.real_kind = kind
        if kind == "class":
            sc = n.child_by_field_name("superclass")
            if sc is not None:
                t.supertype = _simple_type_name(sc.named_children[-1] if sc.named_children else sc)
        if kind == "record":
            t.record_params = _params(n.child_by_field_name("parameters"))
        if parent is None:
            t.binary = (self.idx.package + "." if self.idx.package else "") + name
        elif member and parent.binary:
            t.binary = parent.binary + "$" + name
        elif local and parent is not None:
            key = (id(parent), name)
            self.local_counter[key] = self.local_counter.get(key, 0) + 1
            t.local_index = self.local_counter[key]
            if parent.binary:
                t.binary = f"{parent.binary}${t.local_index}{name}"
        if body is not None:
            self.class_body(body, t, kind)
        return t

    def anon_type(self, n, body, parent: TypeDecl, method, static_ctx, supertype):
        t = self._new_type("anon", None, parent, n, body, method, static_ctx)
        t.real_kind = "anon"
        t.supertype = supertype
        self.anon_counter[id(parent)] = self.anon_counter.get(id(parent), 0) + 1
        t.anon_index = self.anon_counter[id(parent)]
        if parent.binary:
            t.binary = f"{parent.binary}${t.anon_index}"
        self.class_body(body, t, "anon")
        return t

    def class_body(self, body, t: TypeDecl, kind: str) -> None:
        for c in body.named_children:
            if c.type in TYPE_DECLS:
                self.type_decl(c, t, None, False, member=True)
            elif c.type in METHOD_DECLS:
                self.method_decl(c, t)
            elif c.type == "enum_body_declarations":
                self.class_body(c, t, kind)
            elif c.type == "enum_constant":
                args = c.child_by_field_name("arguments")
                if args is not None:
                    self.expr(args, t, None, True)
                cb = c.child_by_field_name("body")
                if cb is not None:
                    self.anon_type(c, cb, t, None, True, t.name)
            elif c.type == "field_declaration" or c.type == "constant_declaration":
                static = "static" in _modifiers(c) or kind in ("interface", "annotation")
                for d in c.named_children:
                    if d.type == "variable_declarator":
                        v = d.child_by_field_name("value")
                        if v is not None:
                            self.visit(v, t, None, static)
            elif c.type == "static_initializer":
                self.expr(c, t, None, True)
            elif c.type == "block":
                self.expr(c, t, None, False)

    def method_decl(self, n, t: TypeDecl) -> None:
        kind = {"method_declaration": "method", "constructor_declaration": "ctor",
                "compact_constructor_declaration": "compact"}[n.type]
        name_n = n.child_by_field_name("name")
        name = node_text(name_n) if name_n is not None else "?"
        body = n.child_by_field_name("body")
        mods = _modifiers(n)
        if kind == "method":
            rt = n.child_by_field_name("type")
            ret = node_text(rt) if rt is not None else None
            dims = n.child_by_field_name("dimensions")
            if ret is not None and dims is not None:
                ret += "[]" * node_text(dims).count("[")
            params = _params(n.child_by_field_name("parameters"))
        elif kind == "ctor":
            name, ret = "<init>", None
            params = _params(n.child_by_field_name("parameters"))
        else:
            name, ret = "<init>", None
            params = list(t.record_params or [])
        m = MethodDecl(
            name=name,
            kind=kind,
            owner=t,
            node=n,
            body=body,
            params=params,
            ret_type=ret,
            type_params=type_params_of(n),
            start_line=_line(n.start_point),
            end_line=_line(n.end_point),
            has_error=n.has_error,
            is_static="static" in mods,
        )
        t.methods.append(m)
        self.idx.methods.append(m)
        if body is not None:
            self.expr(body, t, m, m.is_static)

    # -- bodies: find local/anonymous classes (textual order = javac numbering) --

    def expr(self, x, t: TypeDecl, m: MethodDecl | None, static_ctx: bool) -> None:
        for c in x.named_children:
            self.visit(c, t, m, static_ctx)

    def visit(self, x, t: TypeDecl, m: MethodDecl | None, static_ctx: bool) -> None:
        ty = x.type
        if ty in TYPE_DECLS:
            self.type_decl(x, t, m, static_ctx, member=False)
            return
        if ty == "object_creation_expression":
            cb = None
            for c in x.named_children:
                if c.type == "class_body":
                    cb = c
            if cb is not None:
                for c in x.named_children:
                    if c is not cb and c.type != "class_body":
                        self.visit(c, t, m, static_ctx)
                self.anon_type(x, cb, t, m, static_ctx, _simple_type_name(x.child_by_field_name("type")))
                return
        for c in x.named_children:
            self.visit(c, t, m, static_ctx)
