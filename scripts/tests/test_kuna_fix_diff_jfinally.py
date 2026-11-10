import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"
MINE = ("finally_exit_versions", "var_merge_lost_definition", "head_definition_scope")


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_jfinally", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


HEAD = "   public void f(Src src) {"


class TestJfinallyRecognisers(unittest.TestCase):
    """The recognisers of finally_exit_versions, var_merge_lost_definition and head_definition_scope."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, a, b):
        """The fixes whose recogniser takes every hunk of a -> b."""
        out = set()
        for fix in MINE:
            f = self.tool.CONTEXT_RECOGNISERS[fix]
            if all(f(a, b, *h, set()) for h in hunks(a, b)):
                out.add(fix)
        return out

    def test_finally_assignment_kept(self):
        a = [HEAD, "      while (cf != null) {", "         try {", "            h = src.open(cf);", "         } finally {",
             "            if (h != null) {", "               h.close();", "               Handle var9 = null;",
             "            }", "         }", "      }", "   }"]
        b = a[:7] + ["               h = null;"] + a[8:]
        self.assertEqual(self.accepts(a, b), {"finally_exit_versions"})

    def test_finally_value_read_after_try(self):
        a = [HEAD, '      String var2 = "a";', "", "      try {", "         src.work();", "      } finally {",
             "         Object var6 = src.obj();", "      }", "", "      src.use(var2);", "   }"]
        b = [HEAD, '      String var2 = "a";', "", "      Object var6;", "      try {", "         src.work();",
             "      } finally {", "         var6 = src.obj();", "      }", "", "      src.use(var6);", "   }"]
        self.assertEqual(self.accepts(a, b), {"finally_exit_versions"})

    def test_assignment_outside_finally_not_taken(self):
        a = [HEAD, "      Handle var9 = null;", "      src.use(h);", "   }"]
        b = [HEAD, "      h = null;", "      src.use(h);", "   }"]
        self.assertEqual(self.accepts(a, b), set())

    def test_lost_declaration_restored(self):
        a = [HEAD, '      String var2 = "a";', "", "      try {", "         var6 = src.item();", "      } finally {",
             "         src.work();", "      }", "", "      src.use(var6);", "   }"]
        b = a[:3] + ["      Item var6;"] + a[3:]
        self.assertEqual(self.accepts(a, b), {"var_merge_lost_definition"})

    def test_declared_variable_not_restored(self):
        a = [HEAD, "      Item var6 = src.item();", "      src.use(var6);", "   }"]
        b = [HEAD, "      Item var7;", "      Item var6 = src.item();", "      src.use(var6);", "   }"]
        self.assertEqual(self.accepts(a, b), set())

    def test_head_declaration_clash_renamed(self):
        a = [HEAD, "      boolean item = false;", "      synchronized (this) {", "         src.work();", "      }", "",
             "      for (Item item : src.items()) {", "         src.use(item);", "      }", "   }"]
        b = a[:6] + ["      for (Item itemx : src.items()) {", "         src.use(itemx);"] + a[8:]
        self.assertEqual(self.accepts(a, b), {"head_definition_scope"})

    def test_rename_without_head_declaration_not_taken(self):
        a = [HEAD, "      boolean item = false;", "      src.work();", "", "      for (Item item : src.items()) {",
             "         src.use(item);", "      }", "   }"]
        b = a[:4] + ["      for (Item itemx : src.items()) {", "         src.use(itemx);"] + a[6:]
        self.assertEqual(self.accepts(a, b), set())


if __name__ == "__main__":
    unittest.main()
