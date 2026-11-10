import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktvolatile", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


KT = "   public fun f(i: Int): Int {"
JAVA = "   public final int f(int i) {"


class TestKtVolatile(unittest.TestCase):
    """The recognisers of kt_property_field_flags, kt_jvm_field_setter and kt_inline_heap_read."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def flags(self, a, b):
        """Every hunk that is not blank lines only (the tool's `import` kind) is the fix's."""
        return all(self.tool.RECOGNISERS["kt_property_field_flags"](a[i1:i2], b[j1:j2])
                   for i1, i2, j1, j2 in hunks(a, b) if any(x.strip() for x in a[i1:i2] + b[j1:j2]))

    def context(self, fix, a, b):
        """Every hunk of a -> b, both in a class, is the fix's."""
        a, b = ["class C {"] + a + ["}"], ["class C {"] + b + ["}"]
        return all(self.tool.CONTEXT_RECOGNISERS[fix](a, b, *h, set()) for h in hunks(a, b))

    def test_field_flag_annotations_added(self):
        a = ["   public final var state: Any? = null", "   private final val memo: IntArray = IntArray(4)"]
        b = ["   @Volatile", a[0], "", "   @Transient", a[1], ""]
        self.assertTrue(self.flags(a, b))
        self.assertTrue(self.flags(["   val x by lazy"], ["   @delegate:Transient", "   val x by lazy"]))

    def test_other_annotation_is_not_a_field_flag(self):
        a = ["   public final var state: Any? = null"]
        self.assertFalse(self.flags(a, ["   @JvmField", a[0]]))
        self.assertFalse(self.flags(a, ["   @Volatile", "   public final var state: Any = null"]))

    def test_jvm_field_private_set_dropped(self):
        a = ["   @JvmField", "   @Nullable", "   public final var head: Segment? = null", "      private set", "",
             "   fun g() {"]
        b = a[:3] + a[4:]
        self.assertTrue(self.context("kt_jvm_field_setter", a, b))

    def test_private_set_without_jvm_field_kept(self):
        a = ["   @Nullable", "   public final var head: Segment? = null", "      private set", "", "   fun g() {"]
        b = a[:2] + a[3:]
        self.assertFalse(self.context("kt_jvm_field_setter", a, b))

    def test_heap_read_kept_in_its_local(self):
        a = [KT, "      return if (this.plain != null) this.plain.value else 0", "   }"]
        b = [KT, "      val p: Node = this.plain", "      return if (p != null) p.value else 0", "   }"]
        self.assertTrue(self.context("kt_inline_heap_read", a, b))

    def test_partly_copied_heap_read(self):
        a = [KT, "      val e: Int = this[i]", "      if (max < this[i]) {", "         max = e", "      }",
             "      return max", "   }"]
        b = a[:2] + ["      if (max < e) {"] + a[3:]
        self.assertTrue(self.context("kt_inline_heap_read", a, b))

    def test_java_safe_call_kept(self):
        a = [JAVA, "      return this.plain != null ? this.plain.value : -1;", "   }"]
        b = [JAVA, "      Node var10000 = this.plain;", "      return var10000 != null ? var10000.value : -1;", "   }"]
        self.assertTrue(self.context("kt_inline_heap_read", a, b))

    def test_kept_call_is_not_a_heap_read(self):
        a = [KT, "      return if (next() != null) next().value else 0", "   }"]
        b = [KT, "      val p: Node = next()", "      return if (p != null) p.value else 0", "   }"]
        self.assertFalse(self.context("kt_inline_heap_read", a, b))

    def test_other_value_is_not_the_read(self):
        a = [KT, "      return if (this.plain != null) this.other.value else 0", "   }"]
        b = [KT, "      val p: Node = this.plain", "      return if (p != null) p.value else 0", "   }"]
        self.assertFalse(self.context("kt_inline_heap_read", a, b))


if __name__ == "__main__":
    unittest.main()
