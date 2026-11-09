"""Statement-level CFG extraction from Java source text (tree-sitter-java).

ONE extractor is applied identically to the original source and to every
decompiler's output. The graph is built at statement granularity (plus one node
per short-circuit / ternary operand, as Joern does for C), then collapsed into
basic blocks with PyJoern's merge rule (merge u->v when u has exactly one
successor and v exactly one predecessor), and the block containing the
synthetic ENTRY / EXIT node carries ``is_entrypoint`` / ``is_exitpoint`` -- the
only attributes decbench's GED reads besides topology.

Modelling rules (both sides): ``&&``/``||``/``?:`` branch; constant ``true``
conditions have no false edge (``while(true)`` == ``for(;;)``); ``return`` and
``throw`` go to EXIT; ``break``/``continue`` are pure jumps with no node of
their own (Joern's CONTROL_STRUCTURE jumps are invisible in decbench's
projection); ``try`` has an edge to every ``catch`` and, if present, to
``finally`` (whose exits also reach EXIT for the rethrow path); abrupt jumps out
of a ``try`` with ``finally`` are routed through it; lambda bodies and
anonymous/local class bodies are opaque (they are separate methods in bytecode).
"""

from __future__ import annotations

import sys
import threading
from dataclasses import dataclass

sys.setrecursionlimit(100000)

ENTRY, EXIT = 0, 1


class Block:
    """Graph node object; decbench's GED reads only these two role flags."""

    __slots__ = ("id", "is_entrypoint", "is_exitpoint", "size")

    def __init__(self, id_: int, entry: bool, exit_: bool, size: int = 1):
        self.id = id_
        self.is_entrypoint = entry
        self.is_exitpoint = exit_
        self.size = size

    def __repr__(self) -> str:  # pragma: no cover
        return f"B{self.id}{'E' if self.is_entrypoint else ''}{'X' if self.is_exitpoint else ''}"


@dataclass
class CFG:
    roles: list[tuple[bool, bool]]
    edges: list[tuple[int, int]]
    statements: int  # real statement nodes reachable from ENTRY (0 => degenerate)
    raw_nodes: int

    def to_json(self) -> dict:
        return {
            "roles": [[int(a), int(b)] for a, b in self.roles],
            "edges": [list(e) for e in self.edges],
            "statements": self.statements,
            "raw_nodes": self.raw_nodes,
        }

    @classmethod
    def from_json(cls, d: dict) -> "CFG":
        return cls([(bool(a), bool(b)) for a, b in d["roles"]], [tuple(e) for e in d["edges"]],
                   d["statements"], d.get("raw_nodes", 0))

    @property
    def n_blocks(self) -> int:
        return len(self.roles)

    def to_networkx(self):
        import networkx as nx

        g = nx.DiGraph()
        blocks = [Block(i, a, b) for i, (a, b) in enumerate(self.roles)]
        g.add_nodes_from(blocks)
        g.add_edges_from((blocks[u], blocks[v]) for u, v in self.edges)
        return g


def _unwrap(n):
    while n is not None and n.type in ("parenthesized_expression", "condition") and n.named_child_count == 1:
        n = n.named_children[0]
    return n


def _op(n) -> str | None:
    o = n.child_by_field_name("operator")
    return o.type if o is not None else None


class _Builder:
    def __init__(self):
        self.kinds: list[str] = ["ENTRY", "EXIT"]
        self.succ: list[list[int]] = [[], []]
        self.ctx: list[list] = []  # jump-target contexts (innermost last)

    def new(self, kind: str) -> int:
        self.kinds.append(kind)
        self.succ.append([])
        return len(self.kinds) - 1

    def connect(self, preds, node: int) -> None:
        for p in preds:
            if node not in self.succ[p]:
                self.succ[p].append(node)

    # -- expressions -----------------------------------------------------

    def expr(self, n, preds: list[int]) -> list[int]:
        if n is None:
            return preds
        t = n.type
        if t in ("lambda_expression", "class_body"):
            return preds
        if t == "binary_expression" and _op(n) in ("&&", "||"):
            tr, fl = self.cond(n, preds)
            return tr + fl
        if t == "ternary_expression":
            tr, fl = self.cond(n.child_by_field_name("condition"), preds)
            a = self.value(n.child_by_field_name("consequence"), tr)
            b = self.value(n.child_by_field_name("alternative"), fl)
            return a + b
        if t == "switch_expression":
            return self.switch(n, preds, as_expr=True)
        for c in n.named_children:
            preds = self.expr(c, preds)
        return preds

    def value(self, n, preds: list[int]) -> list[int]:
        preds = self.expr(n, preds)
        v = self.new("val")
        self.connect(preds, v)
        return [v]

    def cond(self, n, preds: list[int]) -> tuple[list[int], list[int]]:
        n = _unwrap(n)
        if n is None:
            return preds, []
        t = n.type
        if t == "unary_expression" and _op(n) == "!":
            tr, fl = self.cond(n.child_by_field_name("operand"), preds)
            return fl, tr
        if t == "binary_expression" and _op(n) == "&&":
            t1, f1 = self.cond(n.child_by_field_name("left"), preds)
            t2, f2 = self.cond(n.child_by_field_name("right"), t1)
            return t2, f1 + f2
        if t == "binary_expression" and _op(n) == "||":
            t1, f1 = self.cond(n.child_by_field_name("left"), preds)
            t2, f2 = self.cond(n.child_by_field_name("right"), f1)
            return t1 + t2, f2
        if t == "ternary_expression":
            tc, fc = self.cond(n.child_by_field_name("condition"), preds)
            ta, fa = self.cond(n.child_by_field_name("consequence"), tc)
            tb, fb = self.cond(n.child_by_field_name("alternative"), fc)
            return ta + tb, fa + fb
        if t == "true":
            return preds, []
        if t == "false":
            return [], preds
        preds = self.expr(n, preds)
        c = self.new("cond")
        self.connect(preds, c)
        return [c], [c]

    # -- jumps -------------------------------------------------------------

    def jump(self, kind: str, label: str | None, src: list[int]) -> None:
        """Route a break/continue/yield/return from ``src`` to its target,
        through every enclosing ``finally`` (innermost first)."""
        for i in range(len(self.ctx) - 1, -1, -1):
            c = self.ctx[i]
            ck = c[0]
            if ck == "finally":
                fin = c[1]
                self.connect(src, fin)
                c[2].append((kind, label, i))
                return
            if ck == "barrier":
                break
            if kind == "return":
                continue
            if kind == "yield":
                if ck == "switchexpr":
                    c[2].extend(src)
                    return
                continue
            labels = c[1]
            if kind == "break":
                if label is None and ck in ("loop", "switch"):
                    c[2].extend(src)
                    return
                if label is not None and label in labels:
                    c[2].extend(src)
                    return
            if kind == "continue" and ck == "loop":
                if label is None or label in labels:
                    self.connect(src, c[3])
                    return
        if kind == "return" or True:
            self.connect(src, EXIT)

    def resume_from_finally(self, pending, fin_exits: list[int], depth: int) -> None:
        saved = self.ctx
        self.ctx = saved[:depth]
        try:
            for kind, label, _d in pending:
                self.jump(kind, label, fin_exits)
        finally:
            self.ctx = saved

    # -- statements ----------------------------------------------------------

    def block(self, n, preds: list[int]) -> list[int]:
        for c in n.named_children:
            preds = self.stmt(c, preds)
        return preds

    def simple(self, n, preds: list[int], kind: str = "stmt") -> list[int]:
        preds = self.expr(n, preds)
        s = self.new(kind)
        self.connect(preds, s)
        return [s]

    def stmt(self, n, preds: list[int], labels: tuple = ()) -> list[int]:
        t = n.type
        if t in ("block", "constructor_body"):
            if labels:
                c = ["block", labels, []]
                self.ctx.append(c)
                out = self.block(n, preds)
                self.ctx.pop()
                return out + c[2]
            return self.block(n, preds)
        if t in ("line_comment", "block_comment"):
            return preds
        if t in ("class_declaration", "interface_declaration", "enum_declaration",
                 "record_declaration", "annotation_type_declaration"):
            return preds
        if t == "labeled_statement":
            lab = None
            body = None
            for c in n.named_children:
                if c.type == "identifier" and lab is None:
                    lab = c.text.decode()
                else:
                    body = c
            if body is None:
                return preds
            return self.stmt(body, preds, labels + ((lab,) if lab else ()))
        if t == "if_statement":
            tr, fl = self.cond(n.child_by_field_name("condition"), preds)
            out = self.stmt(n.child_by_field_name("consequence"), tr)
            alt = n.child_by_field_name("alternative")
            if alt is not None:
                out = out + self.stmt(alt, fl)
            else:
                out = out + fl
            return out
        if t == "while_statement":
            head = self.new("loop")
            self.connect(preds, head)
            tr, fl = self.cond(n.child_by_field_name("condition"), [head])
            c = ["loop", labels, [], head]
            self.ctx.append(c)
            body_out = self.stmt(n.child_by_field_name("body"), tr)
            self.ctx.pop()
            self.connect(body_out, head)
            return fl + c[2]
        if t == "do_statement":
            head = self.new("loop")
            self.connect(preds, head)
            cexpr = _unwrap(n.child_by_field_name("condition"))
            const_true = cexpr is not None and cexpr.type == "true"
            cnode = head if const_true else self.new("docond")
            c = ["loop", labels, [], cnode]
            self.ctx.append(c)
            body_out = self.stmt(n.child_by_field_name("body"), [head])
            self.ctx.pop()
            if const_true:
                self.connect(body_out, head)
                return c[2]
            self.connect(body_out, cnode)
            tr, fl = self.cond(cexpr, [cnode])
            self.connect(tr, head)
            return fl + c[2]
        if t == "for_statement":
            for init in n.children_by_field_name("init"):
                if init.type == "local_variable_declaration":
                    preds = self.simple(init, preds)
                else:
                    preds = self.simple(init, preds)
            head = self.new("loop")
            self.connect(preds, head)
            cnd = n.child_by_field_name("condition")
            if cnd is not None:
                tr, fl = self.cond(cnd, [head])
            else:
                tr, fl = [head], []
            updates = n.children_by_field_name("update")
            upd = self.new("update") if updates else head
            c = ["loop", labels, [], upd]
            self.ctx.append(c)
            body_out = self.stmt(n.child_by_field_name("body"), tr)
            self.ctx.pop()
            if updates:
                self.connect(body_out, upd)
                cur = [upd]
                for u in updates:
                    cur = self.expr(u, cur)
                self.connect(cur, head)
            else:
                self.connect(body_out, head)
            return fl + c[2]
        if t == "enhanced_for_statement":
            preds = self.expr(n.child_by_field_name("value"), preds)
            head = self.new("iter")
            self.connect(preds, head)
            c = ["loop", labels, [], head]
            self.ctx.append(c)
            body_out = self.stmt(n.child_by_field_name("body"), [head])
            self.ctx.pop()
            self.connect(body_out, head)
            return [head] + c[2]
        if t == "switch_expression" or t == "switch_statement":
            return self.switch(n, preds, as_expr=False, labels=labels)
        if t == "expression_statement":
            inner = n.named_children[0] if n.named_child_count else None
            if inner is not None and inner.type == "switch_expression":
                return self.switch(inner, preds, as_expr=False, labels=labels)
            return self.simple(n, preds)
        if t == "return_statement":
            s = self.simple(n, preds, "return")
            self.jump("return", None, s)
            return []
        if t == "throw_statement":
            s = self.simple(n, preds, "throw")
            self.connect(s, EXIT)
            return []
        if t in ("break_statement", "continue_statement"):
            # pure jumps: like Joern's CONTROL_STRUCTURE break/continue, which the
            # decbench projection makes invisible, they get no node of their own
            lab = None
            for c in n.named_children:
                if c.type == "identifier":
                    lab = c.text.decode()
            self.jump(t.split("_")[0], lab, preds)
            return []
        if t == "yield_statement":
            s = self.simple(n, preds, "yield")
            self.jump("yield", None, s)
            return []
        if t == "synchronized_statement":
            lock = None
            body = n.child_by_field_name("body")
            for c in n.named_children:
                if c.type == "parenthesized_expression":
                    lock = c
            s = self.simple(lock, preds, "sync") if lock is not None else preds
            return self.stmt(body, s) if body is not None else s
        if t in ("try_statement", "try_with_resources_statement"):
            return self.try_(n, preds, labels)
        if t == "assert_statement":
            exprs = n.named_children
            if not exprs:
                return preds
            tr, fl = self.cond(exprs[0], preds)
            if fl:
                fail = self.new("assertfail")
                cur = [fail]
                self.connect(fl, fail)
                for e in exprs[1:]:
                    cur = self.expr(e, cur)
                self.connect(cur, EXIT)
            ok = self.new("assert")
            self.connect(tr, ok)
            return [ok]
        if t == "local_variable_declaration" or t == "explicit_constructor_invocation":
            return self.simple(n, preds)
        if t == "ERROR":
            return self.simple(n, preds, "error")
        # any other statement form: a plain node
        return self.simple(n, preds)

    def switch(self, n, preds: list[int], as_expr: bool, labels: tuple = ()) -> list[int]:
        cond = n.child_by_field_name("condition")
        preds = self.expr(cond, preds)
        head = self.new("switch")
        self.connect(preds, head)
        body = n.child_by_field_name("body")
        c = ["switchexpr" if as_expr else "switch", labels, []]
        self.ctx.append(c)
        out: list[int] = []
        has_default = False
        fall: list[int] = []
        groups = body.named_children if body is not None else []
        for g in groups:
            if g.type == "switch_block_statement_group":
                labs = [x for x in g.named_children if x.type == "switch_label"]
                stmts = [x for x in g.named_children if x.type != "switch_label"]
                if any(_is_default(x) for x in labs):
                    has_default = True
                entry = self.new("case")
                self.connect([head] + fall, entry)
                fall = self.block_list(stmts, [entry])
            elif g.type == "switch_rule":
                labs = [x for x in g.named_children if x.type == "switch_label"]
                if any(_is_default(x) for x in labs):
                    has_default = True
                entry = self.new("case")
                self.connect([head], entry)
                rest = [x for x in g.named_children if x.type != "switch_label" and x.type != "guard"]
                if not rest:
                    out.append(entry)
                    continue
                r = rest[-1]
                if r.type == "expression_statement" and as_expr:
                    res = self.simple(r, [entry], "yield")
                    out += res
                elif r.type in ("block", "throw_statement", "expression_statement"):
                    out += self.stmt(r, [entry])
                else:
                    res = self.simple(r, [entry], "yield" if as_expr else "stmt")
                    out += res
        out += fall
        self.ctx.pop()
        if not has_default and not as_expr:
            out.append(head)
        return out + c[2]

    def block_list(self, stmts, preds: list[int]) -> list[int]:
        for s in stmts:
            preds = self.stmt(s, preds)
        return preds

    def try_(self, n, preds: list[int], labels: tuple) -> list[int]:
        res = n.child_by_field_name("resources")
        if res is not None:
            for r in res.named_children:
                if r.type == "resource":
                    preds = self.simple(r, preds, "resource")
        head = self.new("try")
        self.connect(preds, head)
        catches = [c for c in n.named_children if c.type == "catch_clause"]
        fin_clause = next((c for c in n.named_children if c.type == "finally_clause"), None)
        fctx = None
        if fin_clause is not None:
            fin = self.new("finally")
            fctx = ["finally", fin, []]
            self.ctx.append(fctx)
        lab_ctx = None
        if labels:
            lab_ctx = ["block", labels, []]
            self.ctx.insert(len(self.ctx) - (1 if fctx else 0), lab_ctx)
        out = self.stmt(n.child_by_field_name("body"), [head])
        for cc in catches:
            cnode = self.new("catch")
            self.connect([head], cnode)
            body = cc.child_by_field_name("body")
            out += self.stmt(body, [cnode]) if body is not None else [cnode]
        if fctx is not None:
            self.ctx.pop()
            fin = fctx[1]
            self.connect(out + [head], fin)
            fblock = next((c for c in fin_clause.named_children if c.type == "block"), None)
            fin_out = self.stmt(fblock, [fin]) if fblock is not None else [fin]
            self.connect(fin_out, EXIT)  # exceptional path: finally then rethrow
            if fctx[2] and fin_out:
                self.resume_from_finally(fctx[2], fin_out, len(self.ctx))
            out = fin_out
        if lab_ctx is not None:
            self.ctx.remove(lab_ctx)
            out = out + lab_ctx[2]
        return out


def _is_default(label) -> bool:
    for c in label.children:
        if c.type == "default":
            return True
    return label.text.strip().startswith(b"default")


def _collapse(b: _Builder) -> CFG:
    n = len(b.kinds)
    seen = [False] * n
    order = []
    stack = [ENTRY]
    while stack:
        x = stack.pop()
        if seen[x]:
            continue
        seen[x] = True
        order.append(x)
        for y in reversed(b.succ[x]):
            if not seen[y]:
                stack.append(y)
    order.sort()
    alive = set(order)
    succ = {x: [y for y in b.succ[x] if y in alive] for x in order}
    pred: dict[int, list[int]] = {x: [] for x in order}
    for x in order:
        for y in succ[x]:
            pred[y].append(x)
    statements = sum(1 for x in order if x not in (ENTRY, EXIT))
    members = {x: [x] for x in order}
    # PyJoern/cfgutils merge: u->v when out(u)==1, in(v)==1, u != v.
    changed = True
    while changed:
        changed = False
        for u in sorted(succ):
            if u not in succ:
                continue
            if len(succ[u]) != 1:
                continue
            v = succ[u][0]
            if v == u or len(pred[v]) != 1 or v == ENTRY:
                continue
            members[u].extend(members.pop(v))
            succ[u] = [w if w != v else u for w in succ.pop(v)]
            for w in succ[u]:
                pred[w] = [u if p == v else p for p in pred[w]]
            pred.pop(v)
            changed = True
    ids = {x: i for i, x in enumerate(sorted(succ))}
    roles = []
    for x in sorted(succ):
        mem = members[x]
        roles.append((ENTRY in mem, EXIT in mem))
    edges = sorted({(ids[u], ids[v]) for u in succ for v in succ[u]})
    return CFG(roles, edges, statements, len(order))


def build_cfg(body) -> CFG:
    """CFG of a method/constructor body node (``None`` body -> degenerate)."""
    b = _Builder()
    if body is None:
        b.connect([ENTRY], EXIT)
        return _collapse(b)
    if body.type in ("block", "constructor_body"):
        out = b.block(body, [ENTRY])
    else:  # expression-bodied (not used for methods, kept for robustness)
        out = b.simple(body, [ENTRY])
    b.connect(out, EXIT)
    return _collapse(b)


def run_big_stack(fn, *args, stack_mb: int = 512):
    """Run ``fn`` in a thread with a large C stack (deeply nested expressions)."""
    result: list = []
    error: list = []

    def target():
        try:
            result.append(fn(*args))
        except BaseException as e:  # pragma: no cover
            error.append(e)

    old = threading.stack_size()
    threading.stack_size(stack_mb * 1024 * 1024)
    try:
        th = threading.Thread(target=target)
        th.start()
        th.join()
    finally:
        threading.stack_size(old)
    if error:
        raise error[0]
    return result[0]
