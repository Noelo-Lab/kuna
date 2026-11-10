import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktlockexpr", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


KT = "   public fun f(i: Int): Boolean {"
JAVA = "   int f() {"
FIXES = ("kt_inline_lock_local", "synchronized_lock_assignment")


class TestLockLocals(unittest.TestCase):
    """The recognisers of kt_inline_lock_local and synchronized_lock_assignment: the value a `synchronized`
    locks stays in the local the bytecode locks."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, a, b):
        """The fixes whose recogniser takes every hunk of a -> b."""
        return {fix for fix in FIXES
                if all(self.tool.CONTEXT_RECOGNISERS[fix](a, b, *h, set()) for h in hunks(a, b))}

    def test_element_lock_is_its_local(self):
        a = [KT, "      val e: Any = this.slots[i]", "      synchronized (this.slots[i]) {", "         return e.ok()",
             "      }", "   }"]
        b = a[:2] + ["      synchronized (e) {"] + a[3:]
        self.assertEqual(self.accepts(a, b), {"kt_inline_lock_local"})

    def test_other_uses_of_the_lock_local(self):
        a = [KT, "      val state: Any = `this_$iv`.state", "      if (c && `this_$iv`.state is Finishing) {",
             "         synchronized (`this_$iv`.state) {", "            return add(state)", "         }", "      }",
             "      return false", "   }"]
        b = [a[0], a[1], "      if (c && state is Finishing) {", "         synchronized (state) {"] + a[4:]
        self.assertEqual(self.accepts(a, b), {"kt_inline_lock_local"})

    def test_not_the_local_value(self):
        a = [KT, "      val e: Any = this.slots[i]", "      synchronized (this.slots[0]) {", "         return e.ok()",
             "      }", "   }"]
        b = a[:2] + ["      synchronized (e) {"] + a[3:]
        self.assertEqual(self.accepts(a, b), set())

    def test_no_synchronized_changed(self):
        a = [KT, "      val l: Any = this.lock", "      synchronized (l) {", "         use(this.lock)", "      }", "   }"]
        b = a[:3] + ["         use(l)"] + a[4:]
        self.assertEqual(self.accepts(a, b), set())

    def test_lock_assignment_kept(self):
        a = [JAVA, "      this.lock = new Object();", "      synchronized (this.lock) {", "         this.count++;",
             "      }", "      return this.count;", "   }"]
        b = [JAVA, "      Object l = this.lock;", a[1], "      synchronized (l) {"] + a[3:]
        self.assertEqual(self.accepts(a, b), {"synchronized_lock_assignment"})

    def test_repeated_call_kept(self):
        a = [JAVA, "      synchronized (this.next()) {", "         this.count++;", "      }", "      return this.calls;",
             "   }"]
        b = [JAVA, "      Object l = this.next();"] + a[1:]
        self.assertEqual(self.accepts(a, b), {"synchronized_lock_assignment"})

    def test_declaration_of_another_value(self):
        a = [JAVA, "      synchronized (this.lock) {", "         this.count++;", "      }", "   }"]
        b = [JAVA, "      Object l = this.other;", "      synchronized (l) {"] + a[2:]
        self.assertEqual(self.accepts(a, b), set())

    def test_other_change_with_it(self):
        a = [JAVA, "      this.lock = new Object();", "      synchronized (this.lock) {", "         this.count++;",
             "      }", "   }"]
        b = [JAVA, "      Object l = this.lock;", a[1], "      synchronized (l) {", "         this.count--;"] + a[4:]
        self.assertEqual(self.accepts(a, b), set())


if __name__ == "__main__":
    unittest.main()
