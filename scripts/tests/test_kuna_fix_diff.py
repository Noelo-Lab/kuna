import importlib.util
import os
import signal
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


check = load("kuna_fix_check", ROOT / "tests/jvm/kuna_fix_check.py")

IDS = {"external_class_diamond", "kt_property_reference"}
BASE = """
def diamond(old, new):
    return False


def scope(a, b, i1, i2, j1, j2, throw_names):
    return False


CONTEXT_RECOGNISERS = {'external_class_diamond': scope}
RECOGNISERS = {'external_class_diamond': diamond}
"""

HUNKS = [
    ([], []),
    ([""], ["import a.B;"]),
    (["    int x = (int)y;"], ["    int x = y;"]),
    (["      List a = new ArrayList();"], ["      List<String> a = new ArrayList<>();"]),
    (["   x = c ? (long)a : b;", "   return;"], ["   x = c ? a : (long)b;"]),
    (["   for (i in 0..n) {", "      f(i)", "   }"],
     ["   // $VF: Unable to resugar Kotlin loop from Java for loop", "   var i: Int = 0", "   while (i < n) {",
      "      f(i)", "      i++", "   }"]),
    (["   val a: Int = b.toInt()", "   throw new AssertionError(\"x\");"],
     ["   val a: Int = (int)b", "   assert x;", "   switch (v) {", "      case 1 -> yield 2;", "   }"]),
    (["   Object o = (Foo)(() -> f());"], ["   Object o = (Foo)(Bar)(x::y) + ((((z))));"]),
]


class TestSelfCheck(unittest.TestCase):
    def test_tool_passes(self):
        self.assertEqual(check.problems(TOOL.read_text(), check.fix_ids()), [])

    def test_fix_ids_are_read(self):
        ids = check.fix_ids()
        self.assertIn("report_to_kuna", ids)
        self.assertIn("external_class_diamond", ids)

    def test_minimal_source_passes(self):
        self.assertEqual(check.problems(BASE, IDS), [])

    def test_two_functions_of_one_name(self):
        found = check.problems(BASE + "\n\ndef diamond(old, new):\n    return True\n", IDS)
        self.assertTrue(any("`diamond` is bound again" in p for p in found), found)

    def test_missing_recogniser(self):
        found = check.problems(BASE + "RECOGNISERS['kt_property_reference'] = gone\n", IDS)
        self.assertTrue(any("`gone`, which is not a module-level function" in p for p in found), found)

    def test_entry_naming_no_fix(self):
        found = check.problems(BASE.replace("{'external_class_diamond': diamond}", "{'diamonds': diamond}"), IDS)
        self.assertTrue(any("['diamonds'] names no KunaFix id" in p for p in found), found)

    def test_fix_group(self):
        source = BASE.replace("{'external_class_diamond': diamond}", "{'refs': diamond}")
        self.assertEqual(check.problems(source + "FIX_GROUPS = {'refs': ('kt_property_reference',)}\n", IDS), [])
        found = check.problems(source + "FIX_GROUPS = {'refs': ('kt_property_references',)}\n", IDS)
        self.assertTrue(any("names 'kt_property_references', not a KunaFix id" in p for p in found), found)

    def test_update_from_a_list(self):
        source = BASE + ("ORDER = ['kt_property_reference', 'nope']\n\n\ndef make(fix):\n    return diamond\n\n\n"
                         "RECOGNISERS.update((fix, make(fix)) for fix in ORDER[:-1])\n")
        self.assertEqual(check.problems(source, IDS), [])
        found = check.problems(source.replace("ORDER[:-1]", "ORDER"), IDS)
        self.assertTrue(any("['nope'] names no KunaFix id" in p for p in found), found)

    def test_wrong_arity(self):
        found = check.problems(BASE.replace("{'external_class_diamond': scope}", "{'external_class_diamond': diamond}"),
                               IDS)
        self.assertTrue(any("does not take 7 arguments" in p for p in found), found)


class TestRecognisers(unittest.TestCase):
    """Every recogniser returns on small hunks (a merge that cuts the tail off a loop makes one spin)."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def call(self, name, f, *args):
        def timeout(*_):
            raise TimeoutError(f"{name} did not return within 10 s")
        previous = signal.signal(signal.SIGALRM, timeout)
        signal.alarm(10)
        try:
            f(*args)
        finally:
            signal.alarm(0)
            signal.signal(signal.SIGALRM, previous)

    def test_recognisers_return(self):
        for old, new in HUNKS:
            for name, f in self.tool.RECOGNISERS.items():
                with self.subTest(recogniser=name, old=old, new=new):
                    self.call(name, f, list(old), list(new))
            a, b = ["class A {"] + list(old) + ["}"], ["class A {"] + list(new) + ["}"]
            for name, f in self.tool.CONTEXT_RECOGNISERS.items():
                with self.subTest(context=name, old=old, new=new):
                    self.call(name, f, a, b, 1, 1 + len(old), 1, 1 + len(new), {"x"})

    def test_trees(self):
        with tempfile.TemporaryDirectory() as tmp:
            compat, default = Path(tmp, "compat/p"), Path(tmp, "default/p")
            compat.mkdir(parents=True)
            default.mkdir(parents=True)
            (compat / "A.java").write_text("package p;\n\nclass A {\n   Object f() {\n      return new B(1);\n"
                                           "   }\n}\n")
            (default / "A.java").write_text("package p;\n\nimport q.C;\n\nclass A {\n   Object f() {\n"
                                            "      return new B<>(1);\n   }\n}\n")
            (compat / "D.java").write_text("class D {\n   int g(int a) {\n      return a + 1;\n   }\n}\n")
            (default / "D.java").write_text("class D {\n   int g(int a) {\n      return a - 1;\n   }\n}\n")
            run = subprocess.run([sys.executable, "-I", str(TOOL), str(compat.parent), str(default.parent)],
                                 capture_output=True, text=True, timeout=120, env={**os.environ})
        self.assertEqual(run.returncode, 1, run.stderr)
        summary, *collateral = run.stdout.splitlines()
        self.assertIn("files 2, differing 2;", summary)
        self.assertIn("1 imports, 1 other, 1 external_class_diamond,", summary)
        self.assertEqual(collateral, ["COLLATERAL p/D.java:3:       return a + 1; | -> |       return a - 1;"])



class TestKtDefaultArgsCall(unittest.TestCase):
    """kt_default_args_call: a bridge call resugared, the arguments its mask defaults left out."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_resugared_calls(self):
        f = self.tool.kt_default_args_call
        self.assertTrue(f(["return K.join$default(this.base, 0, null, 6, null)"], ["return K.join(this.base)"]))
        self.assertTrue(f(["return K.join$default(b, 0, \"m\", 2, null)"], ["return K.join(a = b, c = \"m\")"]))
        self.assertTrue(f(["x = scale$default(other, 0, true, 0.toChar(), 5, null)"], ["x = other.scale(y = true)"]))
        self.assertTrue(f(["return Box(w, 0, null, 6, null).h"], ["return Box(w).h"]))
        self.assertTrue(f(["Job.DefaultImpls.cancel$default(j, (CancellationException)null, 1, null)"], ["j.cancel()"]))

    def test_other_changes_are_not(self):
        f = self.tool.kt_default_args_call
        self.assertFalse(f(["return K.join$default(b, 0, null, 6, null)"], ["return K.join(c)"]))
        self.assertFalse(f(["return K.join$default(b, g(), null, 6, null)"], ["return K.join(b)"]))
        self.assertFalse(f(["return K.join$default(b, 0, null, 2, null)"], ["return K.join(b)"]))
        self.assertFalse(f(["return K.join(b, 0, null)"], ["return K.join(b)"]))

    def test_casts_around_the_call(self):
        f = self.tool.kt_default_args_call_casts
        self.assertTrue(f(["return (C.joinTo$default(x, sb, \"n\", null, 0, 6, null) as StringBuilder).toString()"],
                          ["return C.joinTo(x, sb, \"n\").toString()"]))
        self.assertFalse(f(["return (C.joinTo$default(x, sb, null, 2, null) as StringBuilder).toString()"],
                           ["return C.joinTo(x, y).toString()"]))


class TestKtGeneratedConstructorHidden(unittest.TestCase):
    """kt_generated_constructor_hidden: generated constructors printed as functions are removed."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_removed(self):
        f = self.tool.kt_generated_constructor_hidden
        self.assertTrue(f(["", "   @JvmOverloads", "   fun Builder() {", "      this(null, 1, null)", "   }"], []))
        self.assertTrue(f(["   }", "", "   fun L() {", "      this(0, 1, null)"], []))
        self.assertTrue(f(["class E(m: String = \"x\") : Error(m) {", "   fun E() {", "      this(null, 1, null)",
                           "   }", "}"], ["class E(m: String = \"x\") : Error(m)"]))
        self.assertFalse(f(["   fun Builder() {", "      this(null, 1, null)", "      init()", "   }"], []))
        self.assertFalse(f(["   fun build() {", "      this(null, 1, null)", "   }"], []))



class TestKtJavaReferenceCast(unittest.TestCase):
    """kt_java_reference_cast and kt_explicit_type_arguments: Java's casts and explicit type arguments
    written in Kotlin's syntax."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_casts(self):
        f = self.tool.kt_java_reference_cast
        self.assertTrue(f(["      ((Http2Stream)this).connection.remove(((Http2Stream)this).id)"],
                          ["      (this as Http2Stream).connection.remove((this as Http2Stream).id)"]))
        self.assertTrue(f(["   return (java.util.Set<T>)(if (e != null) S.setOf(e) else S.emptySet())"],
                          ["   return (if (e != null) S.setOf(e) else S.emptySet()) as Set<T>"]))
        self.assertTrue(f(["      A.copyInto((Object[])this.data, (Object[])b, 0)"], ["      A.copyInto(this.data, b as Array<Any>, 0)"]))
        self.assertTrue(f(["      this.values = (V[])values"], ["      this.values = values as Array<V>?"]))
        self.assertTrue(f(["      return O.name((java.lang.String)null)"], ["      return O.name(null as String?)"]))
        self.assertTrue(f(["      val n: Int = ((Object[])`v`).length"], ["      val n: Int = (`v` as Array<Any>).length"]))
        self.assertTrue(f(["      x = (Function2<? super A, ? extends B>)f"], ["      x = f as (A) -> B"]))
        self.assertFalse(f(["      return x + (int)y"], ["      return x - y as Int"]))
        self.assertFalse(f(["      return y as Int"], ["      return y as Long"]))

    def test_cast_tail(self):
        f = self.tool.kt_java_reference_cast_tail
        a = ["      return (java.util.Iterator<T>)object : Iterator<Any> {", "         x()", "      }"]
        b = ["      return object : Iterator<Any> {", "         x()", "      } as Iterator<T>"]
        self.assertTrue(f(a, b, 2, 3, 2, 3, set()))
        self.assertFalse(f(["      g()", "      }"], ["      g()", "      } as Iterator<T>"], 1, 2, 1, 2, set()))

    def test_type_arguments(self):
        f = self.tool.kt_explicit_type_arguments
        self.assertTrue(f(["      return C.<java.lang.String>last(list).length()"], ["      return C.last<String>(list).length()"]))
        self.assertTrue(f(["      v = \"${S.<Class>last(u).getName()}\""], ["      v = \"${S.last<Class<*>>(u).getName()}\""]))
        self.assertTrue(f(["      return S.<S.Seq<>>lastOf(seq)"], ["      return S.lastOf(seq)"]))
        self.assertFalse(f(["      return a.<T>b(c)"], ["      return a.b<T>(d)"]))

if __name__ == "__main__":
    unittest.main()
