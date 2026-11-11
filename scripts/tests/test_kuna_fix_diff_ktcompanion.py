import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktcompanion", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


CLASS = [
    "public abstract class Gen {",
    "   public fun stepped(x: Int): Int {",
    "      return this.step(x)",
    "   }",
    "",
    "   public companion object Default : Gen() {",
    "      public override fun next(until: Int): Int {",
    "         return until - 1",
    "      }",
    "",
    "      private fun step(x: Int): Int {",
    "         return x + 2",
    "      }",
    "   }",
    "}",
]


class TestKtCompanion(unittest.TestCase):
    """The recognisers of kt_companion_static_copy and kt_reused_inline_object."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def companion(self, a, b):
        return all(self.tool.CONTEXT_RECOGNISERS["kt_companion_static_copy"](a, b, *h, set()) for h in hunks(a, b))

    def test_members_the_companion_declares(self):
        b = CLASS[:1] + [
            "   public abstract fun next(until: Int): Int",
            "",
            "   private fun step(x: Int): Int {",
            "      return x + 1",
            "   }",
            "",
        ] + CLASS[1:]
        self.assertTrue(self.companion(CLASS, b))

    def test_a_member_the_companion_does_not_declare(self):
        b = CLASS[:1] + ["   public abstract fun other(until: Int): Int", ""] + CLASS[1:]
        self.assertFalse(self.companion(CLASS, b))

    def test_a_changed_line_is_not_an_addition(self):
        b = [CLASS[0], "   public fun stepped(x: Int): Int = x"] + CLASS[2:]
        self.assertFalse(self.companion(CLASS, b))

    def test_an_addition_the_matcher_shifted(self):
        a = CLASS[:4] + CLASS[4:]
        b = CLASS[:4] + ["", "   private fun step(x: Int): Int {", "      return x + 1", "   }"] + CLASS[4:]
        self.assertTrue(self.companion(a, b))

    def test_members_beside_a_line_another_fix_changed(self):
        a = ["public abstract class Gen {", "   public companion object Default : Gen() {",
             "      public override fun next(until: Int): Int {", "         return until - 1", "      }", "   }", "}"]
        decl = ["   public abstract fun next(until: Int): Int", ""]
        reflowed = a[:1] + decl + ["   public companion object Default :", "      Gen() {"] + a[2:]
        changed = a[:1] + decl + ["   public companion object Default : Gen(), Cloneable {"] + a[2:]
        recognise = self.tool.CONTEXT_RECOGNISERS["kt_companion_static_copy"]
        self.assertTrue(recognise(a, reflowed, 1, 2, 1, 5, set()))
        self.assertFalse(recognise(a, changed, 1, 2, 1, 4, set()))

    def reused(self, old, new):
        return self.tool.RECOGNISERS["kt_reused_inline_object"](old, new)

    def test_an_object_became_the_call(self):
        old = [
            "      this.sortWith(object : Comparator {",
            "         override final fun compare(a: T, b: T): Int {",
            "            return compareValues(selector(a), selector(b))",
            "         }",
            "      })",
        ]
        self.assertTrue(self.reused(old, ["      this.sortWith(compareBy(selector))"]))
        self.assertFalse(self.reused(old, ["      this.sortWith(compareBy(selector, 1))"]))
        self.assertFalse(self.reused(old, ["      this.sortedWith(compareBy(selector))"]))
        self.assertFalse(self.reused(["      this.sortWith(c)"], ["      this.sortWith(compareBy(c))"]))


if __name__ == "__main__":
    unittest.main()
