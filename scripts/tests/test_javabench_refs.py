"""The reference-sequence comparison of `scripts/javabench/refs.py`: member resolution and diff categories; needs no JDK."""

from __future__ import annotations

import unittest
from collections import Counter
from types import SimpleNamespace

from scripts.javabench.classfile import ACC_INTERFACE, ACC_PUBLIC, ACC_STATIC, ClassFile, Method
from scripts.javabench.refs import Hierarchy, _old_synthetic, diff_refs


def cls(name, sup="java/lang/Object", ifs=(), methods=(), fields=(), access=ACC_PUBLIC):
    return ClassFile(52, 0, access, name, sup, list(ifs), [Method(a, n, d) for a, n, d in methods], list(fields))


def hierarchy(*classes):
    h = Hierarchy([], extra={c.name: c for c in classes})
    h.where.clear()
    return h


def refs(*tokens, lambdas=None, exceptions=(), blocks=None):
    return SimpleNamespace(tokens=list(tokens), blocks=Counter(blocks or [tuple(tokens)]), lambdas=lambdas or {},
                           exceptions=list(exceptions))


def call(owner, name, desc, kind="virtual"):
    return ("call", kind, owner, name, desc)


class Resolution(unittest.TestCase):
    def setUp(self):
        self.h = hierarchy(
            cls("p/Coll", ifs=(), methods=[(ACC_PUBLIC, "add", "(Ljava/lang/Object;)Z")], access=ACC_INTERFACE),
            cls("p/List", ifs=("p/Coll",), methods=[(ACC_PUBLIC, "add", "(Ljava/lang/Object;)Z")], access=ACC_INTERFACE),
            cls("p/ArrayList", ifs=("p/List",), methods=[(ACC_PUBLIC, "add", "(Ljava/lang/Object;)Z")]),
            cls("p/Base", methods=[(ACC_STATIC, "s", "()V")], fields=["f"]),
            cls("p/Sub", sup="p/Base"),
            cls("p/Hide", sup="p/Base", methods=[(ACC_STATIC, "s", "()V")], fields=["f"]),
        )

    def test_virtual_call_by_topmost_declaration(self):
        for owner in ("p/ArrayList", "p/List", "p/Coll"):
            self.assertEqual(self.h.canonical("virtual", owner, "add", "(Ljava/lang/Object;)Z"), "p/Coll")

    def test_static_and_field_by_declaring_class(self):
        self.assertEqual(self.h.canonical("static", "p/Sub", "s", "()V"), "p/Base")
        self.assertEqual(self.h.canonical("static", "p/Hide", "s", "()V"), "p/Hide")
        self.assertEqual(self.h.canonical("field", "p/Sub", "f", "I"), "p/Base")
        self.assertEqual(self.h.canonical("field", "p/Hide", "f", "I"), "p/Hide")

    def test_unknown_owner_is_kept(self):
        self.assertEqual(self.h.canonical("virtual", "q/Unknown", "m", "()V"), "q/Unknown")


class Categories(unittest.TestCase):
    def test_equal_and_exception_ranges(self):
        a = refs(call("p/A", "m", "()V"), exceptions=[("java/io/IOException", (0,))])
        self.assertEqual(diff_refs(a, refs(call("p/A", "m", "()V"), exceptions=[("java/io/IOException", (0,))]))[0], [])
        self.assertEqual(diff_refs(a, refs(call("p/A", "m", "()V")))[0], ["exception range differs"])

    def test_other_overload_and_missing_call(self):
        a = refs(call("p/A", "m", "(Lp/Point;)D"), call("p/A", "n", "()V"))
        self.assertEqual(diff_refs(a, refs(call("p/A", "m", "(Lp/Vector;)D"), call("p/A", "n", "()V")))[0], ["other overload"])
        self.assertEqual(diff_refs(a, refs(call("p/A", "m", "(Lp/Point;)D")))[0], ["call missing"])

    def test_moved_duplicated_and_reordered(self):
        x, y, z = call("p/A", "x", "()V"), call("p/A", "y", "()V"), call("p/A", "z", "()V")
        self.assertEqual(diff_refs(refs(x, y), refs(y, x))[0], ["same references, other order"])
        self.assertEqual(diff_refs(refs(x, y, blocks=[(x,), (y,)]), refs(y, x, blocks=[(y,), (x,)]))[0],
                         ["same blocks, other order"])
        self.assertEqual(diff_refs(refs(x, y, z), refs(x, y, z, y))[0], ["code duplicated"])
        self.assertIn("references moved", diff_refs(refs(x, y, z, ("const", "I", 2)), refs(z, x, ("const", "I", 3)))[0])

    def test_boolean_materialization_and_swapped_constant(self):
        x = call("p/A", "x", "()Z")
        one, zero = ("const", "I", 1), ("const", "I", 0)
        self.assertEqual(diff_refs(refs(x, one, zero, call("p/A", "y", "(Z)V")), refs(x, call("p/A", "y", "(Z)V")))[0],
                         ["boolean materialization"])
        self.assertEqual(diff_refs(refs(x, one), refs(x, zero))[0], ["constant differs"])

    def test_builder_chain_for_concatenation(self):
        sb = "java/lang/StringBuilder"
        a = refs(("new", sb), call(sb, "<init>", "()V", "special"), call(sb, "append", "(I)Ljava/lang/StringBuilder;"),
                 call("java/lang/Object", "toString", "()Ljava/lang/String;"), call("p/A", "use", "(Ljava/lang/String;)V"))
        b = refs(("concat", "(I)A", ("str:'\\x01'",)), call("p/A", "use", "(Ljava/lang/String;)V"))
        self.assertEqual(diff_refs(a, b)[0], ["string concatenation form"])

    def test_lambda_bodies_paired_by_place(self):
        lam = ("lambda", "run", "()Lp/R;", "mtype:()V", "mtype:()V", 6)
        cats, _details, pairs = diff_refs(refs(lam, call("p/A", "x", "()V"), lambdas={0: ("lambda$m$0", "()V")}),
                                          refs(lam, call("p/A", "x", "()V"), lambdas={0: ("lambda$m$3", "()V")}))
        self.assertEqual((cats, pairs), ([], [(0, 0)]))

    def test_old_synthetic_members(self):
        self.assertTrue(_old_synthetic(Method(0, "access$200", "(Lp/A;)I")))
        self.assertTrue(_old_synthetic(Method(0, "<init>", "(Lp/A;Lp/A$1;)V")))
        self.assertFalse(_old_synthetic(Method(0, "<init>", "(Lp/A;)V")))


if __name__ == "__main__":
    unittest.main()
