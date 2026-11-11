import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktinlineclass", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestKtInlineClass(unittest.TestCase):
    """The recogniser of kt_inline_class_value."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, old, new):
        return self.tool.RECOGNISERS["kt_inline_class_value"](old, new)

    def test_plumbing_calls(self):
        self.assertTrue(self.accepts(
            ["      return TestKtInlineClassValue.Meters.plus_HWDY4ac/* $VF was: plus-HWDY4ac */(a, b)"],
            ["      return a.plus(b)"]))
        self.assertTrue(self.accepts(
            ["      c.resumeWith(Result.constructor_impl/* $VF was: constructor-impl */(createFailure(e)))"],
            ["      c.resumeWith(Result.failure(e))"]))
        self.assertTrue(self.accepts(
            ["      cont.resumeWith(Result.constructor_impl/* $VF was: constructor-impl */(Unit))"],
            ["      cont.resumeWith(Result.success(Unit))"]))
        self.assertTrue(self.accepts(
            ["      return listOf(Meters.box_impl/* $VF was: box-impl */(m))"], ["      return listOf(m)"]))
        self.assertTrue(self.accepts(
            ["      return Meters.equals_impl0/* $VF was: equals-impl0 */(a, b)"], ["      return a == b"]))
        self.assertTrue(self.accepts(
            ["      s.trySend_JP2dKIU/* $VF was: trySend-JP2dKIU */(this.value)"], ["      s.trySend(this.value)"]))
        self.assertTrue(self.accepts(
            ["      return Duration.isPositive_impl/* $VF was: isPositive-impl */(d)"], ["      return d.isPositive()"]))

    def test_underlying_reads_and_receivers(self):
        self.assertTrue(self.accepts(["      return m * 2"], ["      return m.v * 2"]))
        self.assertTrue(self.accepts(["      return \"len $m\""], ["      return \"len ${m.v}\""]))
        self.assertTrue(self.accepts(["      return if (c) a else b"], ["      return (if (c) a else b).v"]))
        self.assertTrue(self.accepts(
            ["      return (other as Meters).unbox_impl/* $VF was: unbox-impl */()"], ["      return other as Meters"]))
        self.assertTrue(self.accepts(["   return StartedWhileSubscribed(inWholeMilliseconds, inWholeMilliseconds)"],
                                     ["   return StartedWhileSubscribed(a.inWholeMilliseconds, b.inWholeMilliseconds)"]))
        self.assertTrue(self.accepts(["   val var2: IntArray = this"], ["   val var2: UIntArray = this"]))

    def test_with_another_fix(self):
        self.assertTrue(self.accepts(["   return (char)(var0 and 65535)"],
                                     ["   return (var0.data.toInt() and 65535).toChar()"]))
        self.assertFalse(self.accepts(["   return x.value"], ["   return x"]))

    def test_other_changes(self):
        self.assertFalse(self.accepts(["      return a.plus(b)"], ["      return b.plus(a)"]))
        self.assertFalse(self.accepts(["      return m.v * 2"], ["      return m * 2"]))
        self.assertFalse(self.accepts(["   val x: Foo = y"], ["   val x: Bar = y"]))
        self.assertFalse(self.accepts(
            ["      return Meters.plus_HWDY4ac/* $VF was: plus-HWDY4ac */(a, b)"], ["      return b.plus(a)"]))
        self.assertFalse(self.accepts(["      return UnsignedKt.uintCompare(a, b)"], ["      return uintCompare(a, b)"]))


if __name__ == "__main__":
    unittest.main()
