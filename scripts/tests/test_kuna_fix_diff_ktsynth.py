import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktsynth", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


class TestKtSynth(unittest.TestCase):
    """The recognisers of kt_enum_when_mappings, kt_constructor_accessor_call, kt_empty_primary_constructor and
    the parts of kt_extension_call and kt_legacy_constructor_parameter_checks they cover."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def context(self, fix, a, b):
        """Every hunk of a -> b is the fix's, by its plain recogniser (when it has one) or its context one."""
        plain = self.tool.RECOGNISERS.get(fix) if fix in ("kt_enum_when_mappings", "kt_constructor_accessor_call") \
            else None
        return bool(hunks(a, b)) and all((plain is not None and plain(a[i1:i2], b[j1:j2])) or
                                         self.tool.CONTEXT_RECOGNISERS[fix](a, b, i1, i2, j1, j2, set())
                                         for i1, i2, j1, j2 in hunks(a, b))

    def test_enum_when(self):
        r = self.tool.RECOGNISERS["kt_enum_when_mappings"]
        old = ["   when (if (this.variance == null) -1 else KTypeProjection.WhenMappings.`$EnumSwitchMapping$0`"
               "[this.variance.ordinal]) {",
               "      -1 -> var10000 = \"*\"",
               "      0 -> throw NoWhenBranchMatchedException()",
               "      1, 2 -> var10000 = \"in\""]
        new = ["   when (this.variance) {",
               "      null -> var10000 = \"*\"",
               "      KVariance.INVARIANT, KVariance.IN -> var10000 = \"in\""]
        self.assertTrue(r(old, new))
        self.assertFalse(r(old, ["   when (other) {"] + new[1:]))
        self.assertFalse(r(old, new[:1] + ["      null -> var10000 = \"?\""] + new[2:]))
        self.assertFalse(r(old, new[:2] + ["      KVariance.IN -> var10000 = \"in\""]))
        self.assertFalse(r(old[1:], new[1:]))

    def test_enum_when_branches_alone(self):
        a = ["   when (X.WhenMappings.`$EnumSwitchMapping$0`[s.ordinal]) {", "      1 -> {", "         f()",
             "      }", "      2 -> {", "         g()", "      }", "   }"]
        b = ["   when (s) {", "      E.A -> {", "         f()", "      }", "      E.B -> {", "         g()", "      }",
             "   }"]
        self.assertTrue(self.context("kt_enum_when_mappings", a, b))
        self.assertFalse(self.context("kt_enum_when_mappings", ["   when (s) {"] + a[1:], b))

    def test_enum_when_composed(self):
        a = ["class C {", "   fun f() {", "      when (C.WhenMappings.$EnumSwitchMapping$0[this.ordinal()]) {",
             "         1 -> CancellableKt.startCoroutineCancellable(block, receiver, completion)", "         2 -> {}",
             "      }", "   }", "}"]
        b = ["class C {", "   fun f() {", "      when (this) {",
             "         C.DEFAULT -> block.startCoroutineCancellable(receiver, completion)", "         C.LAZY -> {}",
             "      }", "   }", "}"]
        self.assertTrue(self.context("kt_enum_when_mappings", a, b))
        bad = b[:3] + ["         C.DEFAULT -> block.startCoroutineCancellable(completion, receiver)"] + b[4:]
        self.assertFalse(self.context("kt_enum_when_mappings", a, bad))

    def test_accessor_call(self):
        r = self.tool.RECOGNISERS["kt_constructor_accessor_call"]
        self.assertTrue(r(["   return CipherSuite(javaName, null)"], ["   return CipherSuite(javaName)"]))
        self.assertTrue(r(["   x = Platform(null)"], ["   x = Platform()"]))
        self.assertTrue(r(["   return TimedValue(value, d, null)"], ["   return TimedValue(value as T, d)"]))
        self.assertFalse(r(["   return f(javaName, null)"], ["   return f(javaName)"]))
        self.assertFalse(r(["   return C(a, null, b)"], ["   return C(a, b)"]))
        a = ["   return C(", "      a,", "      null,", "      null", "   )"]
        b = ["   return C(", "      a,", "      null", "   )"]
        self.assertTrue(self.context("kt_constructor_accessor_call", a, b))

    def test_accessor_call_composed(self):
        a = ["class X {", "   fun f(): Foo<T> {", "      return Foo<>(value, d, null)", "   }", "}"]
        b = ["class X {", "   fun f(): Foo<T> {", "      return Foo(value as T, d)", "   }", "}"]
        self.assertTrue(self.context("kt_constructor_accessor_call", a, b))
        c = ["class X {", "   fun f(): Foo<T> {", "      return Foo<>(value, d)", "   }", "}"]
        self.assertFalse(self.context("kt_constructor_accessor_call", c, b))
        self.assertFalse(self.context("kt_constructor_accessor_call", a, b[:2] + ["      return Foo(value as T, e)"] + b[3:]))

    def test_empty_primary_constructor(self):
        r = self.tool.RECOGNISERS["kt_empty_primary_constructor"]
        self.assertTrue(r(["public class Dispatcher {"], ["public class Dispatcher() {"]))
        self.assertTrue(r(["   public class Builder<T> : Base {"], ["   public class Builder<T>() : Base {"]))
        self.assertFalse(r(["public class Dispatcher {"], ["public class Dispatcher(x: Int) {"]))
        self.assertFalse(r(["   fun f() {"], ["   fun f()() {"]))

    def test_member_extension_on_a_copy_of_this(self):
        a = ["   fun maxAge(maxAge: Int): B {", "      val `$this$apply`: B = this",
             "      `$this$apply`.seconds = `$this$apply`.clampToInt(unit.toSeconds(maxAge.toLong()))", "   }"]
        b = a[:2] + ["      `$this$apply`.seconds = unit.toSeconds(maxAge.toLong()).clampToInt()", "   }"]
        self.assertTrue(self.context("kt_extension_call", a, b))
        other = ["   fun maxAge(maxAge: Int): B {", "      val `$this$apply`: B = other"] + a[2:]
        self.assertFalse(self.context("kt_extension_call", other, other[:2] + b[2:]))

    def test_legacy_check_before_a_branching_delegation(self):
        a = ["   constructor(r: R) : Intrinsics.checkParameterIsNotNull(r, \"r\") {", "      val v: S = r.s",
             "      if (v != null) {", "         this(v)", "         return", "      }", "", "      throw E()", "   }"]
        b = ["   constructor(r: R)  {"] + a[1:]
        self.assertTrue(self.context("kt_legacy_constructor_parameter_checks", a, b))
        c = ["   constructor(r: R) : Intrinsics.checkParameterIsNotNull(r, \"r\") {", "      f(r)", "   }"]
        self.assertFalse(self.context("kt_legacy_constructor_parameter_checks", c, ["   constructor(r: R)  {"] + c[1:]))


if __name__ == "__main__":
    unittest.main()
