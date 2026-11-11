import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktinlined2", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CALL = ["      this.queue.schedule(Foo$send$$inlined$execute$1(\"send $id\", true, this, id), 0L)"]
OBJECT = [
    "      val `cancelable$iv`: Boolean = true",
    "      this.queue.schedule(object : Task(\"send $id\", `cancelable$iv`) {",
    "         public override fun runOnce(): Long {",
    "            this@Foo.sent += id",
    "            return -1L",
    "         }",
    "      }, 0L)",
]
UNIT = [
    "package pkg",
    "",
    "import kotlin.jvm.functions.Function1",
    "",
    "/* $VF: local visibility outside of method */ internal class `Foo$send$$inlined$execute$1` : Task {",
    "   public override fun runOnce(): Long {",
    "      return -1L",
    "   }",
    "}",
    "",
]


class TestUnlistedAnonymousClass(unittest.TestCase):
    """The recognisers of unlisted_anonymous_class and refresh_printed_var_names."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def site(self, old, new):
        return self.tool.RECOGNISERS["unlisted_anonymous_class"](old, new)

    def test_constructor_call_became_object(self):
        self.assertTrue(self.site(CALL, OBJECT))

    def test_lambda_with_cast(self):
        old = ["      sink = FaultSink(var18, Foo$Editor$newSink$$inlined$synchronized$lambda$1(this, index))"]
        new = ["      sink = FaultSink(var18, { it: IOException ->", "         this@Editor.detach()",
               "      } as (IOException?) -> Unit)"]
        self.assertTrue(self.site(old, new))

    def test_type_and_captured_val(self):
        old = ["   var result: ObjectRef", "   var `collector$iv`: Foo$first$$inlined$collectWhile$1"]
        new = ["   val result: ObjectRef", "   var `collector$iv`: <unrepresentable>"]
        self.assertTrue(self.site(old, new))

    def test_other_change_beside_the_object(self):
        changed = OBJECT[:-1] + ["      }, 1L)"]
        self.assertFalse(self.site(CALL, changed))

    def test_object_without_a_call(self):
        self.assertFalse(self.site(["      this.queue.schedule(task, 0L)"], OBJECT))

    def test_named_class_call_is_not_anonymous(self):
        old = ["      this.queue.schedule(Foo$Sender(\"send $id\", true, this, id), 0L)"]
        self.assertFalse(self.site(old, OBJECT))

    def test_object_beside_another_fixs_change(self):
        old = ["   return FlowKt.transformLatest(`$this$m`, Foo$m$$inlined$latest$1(transform, null))"]
        new = ["   return this.transformLatest({", "      // $VF: Could not decompile lambda", "   } as Function3)"]
        self.assertEqual(self.tool.uac_placeholders(old, new),
                         (["   return FlowKt.transformLatest(`$this$m`, uacObject)"],
                          ["   return this.transformLatest(uacObject)"]))
        self.assertFalse(self.site(old, ["   return this.transformLatest(transform)"]))

    def test_unit_of_anonymous_class_removed(self):
        unit = self.tool.UNIT_RECOGNISERS["unlisted_anonymous_class"]
        self.assertTrue(unit(UNIT, []))
        self.assertTrue(unit(["package pkg", "", "{ it: IOException ->", "   it.close()", "}"], []))
        self.assertFalse(unit([], UNIT))
        named = [x.replace("$execute$1", "$Execute") for x in UNIT]
        self.assertFalse(unit(named, []))
        self.assertFalse(unit(UNIT + ["class Other {", "}"], []))

    def test_unit_file_in_compat_tree_only(self):
        with tempfile.TemporaryDirectory() as tmp:
            compat, default = Path(tmp, "compat"), Path(tmp, "default")
            for root in (compat, default):
                (root / "pkg").mkdir(parents=True)
                (root / "pkg/Foo.kt").write_text("package pkg\n\nclass Foo {\n" + "\n".join(OBJECT) + "\n}\n")
            (compat / "pkg/Foo.kt").write_text("package pkg\n\nclass Foo {\n" + "\n".join(CALL) + "\n}\n")
            (compat / "pkg/Foo$send$$inlined$execute$1.kt").write_text("\n".join(UNIT))
            run = subprocess.run([sys.executable, "-I", str(TOOL), str(compat), str(default)],
                                 capture_output=True, text=True, timeout=600)
            self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
            self.assertRegex(run.stdout, r", [12] unlisted_anonymous_class,")
            (compat / "pkg/Other.kt").write_text("package pkg\n\nclass Other {\n}\n")
            run = subprocess.run([sys.executable, "-I", str(TOOL), str(compat), str(default)],
                                 capture_output=True, text=True, timeout=600)
            self.assertEqual(run.returncode, 1)
            self.assertIn("pkg/Other.kt: only in compat", run.stdout)

    def test_refreshed_name(self):
        rename = self.tool.RECOGNISERS["refresh_printed_var_names"]
        self.assertTrue(rename(["      var countx: Long = countx"], ["      var countx: Long = count"]))
        self.assertTrue(rename(["   fun emit(valuex: T) {"], ["   fun emit(value: T) {"]))
        self.assertFalse(rename(["      var countx: Long = countx"], ["      var countx: Long = amount"]))
        self.assertFalse(rename(["      var countx: Long = countx + 1"], ["      var countx: Long = count"]))


if __name__ == "__main__":
    unittest.main()
