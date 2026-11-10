import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktinline", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


FUN = "   public fun f(h: Holder, src: Src) {"


class TestKeptLocals(unittest.TestCase):
    """The recognisers of field_read_all_paths and kt_inline_evaluation_order: a local Vineflower's Kotlin
    inliner moved is kept."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, a, b):
        """The fixes whose recogniser takes every hunk of a -> b."""
        out = set()
        for fix in ("field_read_all_paths", "kt_inline_evaluation_order"):
            f = self.tool.CONTEXT_RECOGNISERS[fix]
            if all(f(a, b, *h, set()) for h in hunks(a, b)):
                out.add(fix)
        return out

    def test_field_read_kept_before_a_finally(self):
        a = [FUN, "      try {", "         src.work()", "      } finally {", "         src.use(h.count)", "      }", "   }"]
        b = [FUN, "      val v: Int = h.count", "", "      try {", "         src.work()", "      } finally {",
             "         src.use(v)", "      }", "   }"]
        self.assertEqual(self.accepts(a, b), {"field_read_all_paths"})

    def test_call_kept_in_order(self):
        a = [FUN, "      src.take(src.next(), src.next())", "   }"]
        b = [FUN, "      val first: Int = src.next()", "      src.take(src.next(), first)", "   }"]
        self.assertEqual(self.accepts(a, b), {"kt_inline_evaluation_order"})

    def test_call_kept_out_of_a_template(self):
        a = [FUN, '      out.print("state: ${src.state()} at ${h.line}")', "   }"]
        b = [FUN, "      val state: String = src.state()", '      out.print("state: $state at ${h.line}")', "   }"]
        self.assertEqual(self.accepts(a, b), {"kt_inline_evaluation_order"})

    def test_kept_local_stops_a_conditional_return(self):
        a = [FUN, "      return if (c) -1 else h.read(Math.min(n, h.size))", "   }"]
        b = [FUN, "      if (c) {", "         return -1", "      } else {", "         val var5: Int = Math.min(n, h.size)",
             "         return h.read(var5)", "      }", "   }"]
        self.assertEqual(self.accepts(a, b), {"kt_inline_evaluation_order"})

    def test_other_changes_are_not_taken(self):
        a = [FUN, "      src.take(src.next(), src.next())", "   }"]
        for b in ([FUN, "      val first: Int = src.next()", "      src.take(src.next(), src.next())", "   }"],
                  [FUN, "      src.take(src.next(), src.next(), 1)", "   }"],
                  [FUN, "      val first: Int = src.next()", "      src.take(src.next(), first + 1)", "   }"]):
            self.assertEqual(self.accepts(a, b), set(), b)

    def test_substitution_keeps_members_and_targets(self):
        f = self.tool.kt_kept_sub_text
        self.assertEqual(f("v = v + h.v", {"v": "x"}), "v = (x) + h.v")
        self.assertEqual(f('use("v $v ${v}")', {"v": "g()"}), 'use("v ${g()} ${(g())}")')


if __name__ == "__main__":
    unittest.main()
