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



class TestKtVarargFromMetadata(unittest.TestCase):
    """kt_vararg_from_metadata: `vararg x: Any` written as the array it is, a primitive vararg as `vararg`."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_parameters(self):
        f = self.tool.kt_vararg_from_metadata
        self.assertTrue(f(["class Options(vararg byteStrings: Any, trie: IntArray) {"],
                          ["class Options(byteStrings: Array<ByteString>, trie: IntArray) {"]))
        self.assertTrue(f(["   constructor(vararg specs: Any) : this(specs.size)"],
                          ["   constructor(specs: Array<out ConnectionSpec?>) : this(specs.size)"]))
        self.assertTrue(f(["fun of(xs: UIntArray): Set<UInt> {"], ["fun of(vararg xs: UInt): Set<UInt> {"]))
        self.assertFalse(f(["fun f(vararg x: Any) {"], ["fun f(x: List<String>) {"]))
        self.assertFalse(f(["fun f(xs: IntArray) {"], ["fun f(vararg xs: Long) {"]))
        self.assertFalse(f(["f(vararg x: Any, y)"], ["f(x: Array<String>, z)"]))
        self.assertFalse(f(["fun f(vararg x: Any)"], ["fun f(x: Array"]))


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


class TestKtCtorJumps(unittest.TestCase):
    """kt_abstract_class_constructor, kt_constructor_body_after_first_block, kt_inline_class_constructor_accessor,
    kt_constructor_default_signature, kt_jump_to_non_loop_label and their file-level context recogniser."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_abstract_class_constructor(self):
        f = self.tool.kt_abstract_class_constructor
        self.assertTrue(f(["public abstract class A<E> : B<E>, C {", "   open fun A() {", "   }", ""],
                          ["public abstract class A<E> protected constructor() : B<E>, C {"]))
        self.assertTrue(f(["abstract class S : T {"], ["abstract class S(n: Int) : T(n, \"x\") {"]))
        self.assertTrue(f(["   open fun S(n: Int) {", "      super(n)", "      this.n = n"], ["   init {", "      this.n = n"]))
        self.assertTrue(f(["   open fun L(a: Int) {", "      this(a, null)", "   }"], ["   public constructor(a: Int) : this(a, null)"]))
        self.assertFalse(f(["   open fun S(n: Int) {", "      this.n = n"], ["   init {", "      this.n = m"]))
        self.assertFalse(f(["class S : T {"], ["class S(n: Int) : T(n) {"]))

    def test_constructor_body_after_first_block(self):
        f = self.tool.kt_constructor_body_after_first_block
        self.assertTrue(f(["class C(n: Int) : B(n)"], ["class C(n: Int) : B(n) {", "   init {", "      if (n > 5) {",
                                                          "         throw E()", "      }", "   }", "}"]))
        self.assertTrue(f(["   constructor(a: Int) : this(a, a)"], ["   constructor(a: Int) : this(a, a) {",
                                                                    "      if (a < 0) {", "         f()", "      }", "   }"]))
        self.assertTrue(f(["   }", ""], ["   }", "", "   init {", "      if (s) {", "         g()", "      }", "   }"]))
        self.assertFalse(f(["   x()"], ["   init {", "   }", "   x()"]))
        self.assertFalse(f(["   x()"], ["   y()"]))

    def test_inline_class_constructor_accessor(self):
        f = self.tool.kt_inline_class_constructor_accessor
        self.assertTrue(f(["class R(s: UInt) : R(s), C<UInt> {", "   fun R(s: Int) {", "      super(s, 1, null)", "   }", ""],
                          ["class R(s: UInt) : P(s, 1), C<UInt> {"]))
        self.assertTrue(f(["open class P(s: UInt) : P(s), I {"], ["open class P(s: UInt) : I {"]))
        self.assertTrue(f(["   fun P(s: Int) {", "      this.s = s"], ["   init {", "      this.s = s"]))
        self.assertFalse(f(["class R(s: UInt) : Q(s), C<UInt> {"], ["class R(s: UInt) : P(s, 1), C<UInt> {"]))

    def test_constructor_default_signature(self):
        f = self.tool.kt_constructor_default_signature
        self.assertTrue(f(["   fun Unit() {", "   }", ""], []))
        self.assertFalse(f(["   open fun Unit() {", "   }", ""], []))
        self.assertFalse(f(["   fun Unit() {", "      x()", "   }"], []))

    def test_jump_to_non_loop_label(self):
        f = self.tool.kt_jump_to_non_loop_label
        self.assertTrue(f(["      label7@", "      if (a) {", "         if (b) {", "            break@label7", "         }",
                           "      }"],
                          ["      run label7@{", "         if (a) {", "            if (b) {", "               return@label7",
                           "            }", "         }", "      }"]))
        self.assertTrue(f(["               continue@label3"], ["               return@label3"]))
        self.assertTrue(f(["               break@label9"], ["               return@label9"]))
        self.assertFalse(f(["               break@label9"], ["               return@label8"]))
        self.assertFalse(f(["      label7@", "      if (a) {"], ["      run label7@{", "         if (b) {"]))

    def test_file_context(self):
        f = self.tool.kt_ktprint4_file
        a = ["fun f() {", "   label7@", "   if (a) {", "      g((T)x)", "      if (b) {", "         break@label7",
             "      }", "   }", "}"]
        b = ["fun f() {", "   run label7@{", "      if (a) {", "         g(x as T)", "         if (b) {",
             "            return@label7", "         }", "      }", "   }", "}"]
        self.assertTrue(f(a, b, 1, 8, 1, 9, set()))
        c = list(b)
        c[3] = "         g(x as T, 1)"
        self.assertFalse(f(a, c, 1, 8, 1, 9, set()))


class TestKtSyntheticObjectSuperCall(unittest.TestCase):
    """kt_synthetic_object_super_call: a suspend function's failure comment replaced by its state machine, whose
    continuation is an `object : ContinuationImpl(..)`; the file-level recogniser sets both aside."""

    FAILED = ["   fun f(x: Int): Any {", "      // $VF: Couldn't be decompiled", "      // Bytecode:",
              "      // 00: aload 1", "   }"]
    FIXED = ["   fun f(x: Int): Any? {", "      var `$continuation`: Continuation", "      run label7@{",
             "         `$continuation` = object : ContinuationImpl(`$completion`) {", "            open int label;",
             "         }", "      }", "      return g(`$continuation`)", "   }"]

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_body_replaces_failure(self):
        f = self.tool.kt_synthetic_object_super_call
        self.assertTrue(f(self.FAILED, self.FIXED))
        self.assertTrue(f(["   val r = this.flow", "      .collect(", "         { it: T ->",
                           "            // $VF: Couldn't be decompiled", "         },", "         `$completion`", "      )"],
                          ["   val r = this.flow.collect({ it: T ->", "      run label7@{",
                           "         `$continuation` = object : ContinuationImpl(`$completion`) {", "         }",
                           "      }", "   }, `$completion`)"]))

    def test_inserted_function(self):
        f = self.tool.kt_synthetic_object_super_call
        self.assertTrue(f([], [""] + self.FIXED))

    def test_other_changes_are_not(self):
        f = self.tool.kt_synthetic_object_super_call
        no_object = [x.replace("object : ContinuationImpl", "Cont") for x in self.FIXED]
        self.assertFalse(f(self.FAILED, no_object))
        self.assertFalse(f(self.FAILED + ["   h()"], self.FIXED + ["   h(1)"]))

    def test_file_context(self):
        f = self.tool.kt_synthetic_object_super_call_file
        a = ["class A {", "   fun e() {", "      a()", "   }", ""] + self.FAILED + ["}"]
        b = ["class A {", "   fun e() {", "      a()", "   }", ""] + self.FIXED + ["}"]
        self.assertTrue(f(a, b, 5, 10, 5, 14, set()))
        c = list(b)
        c[2] = "      a(1)"
        self.assertFalse(f(a, c, 2, 3, 2, 3, set()))


class KtPrint5Recognisers(unittest.TestCase):
    """kt_file_class_constructor, kt_when_break, kt_java_statement_jump and kt_catch_block_jump."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_file_class_constructor(self):
        f = self.tool.kt_file_class_constructor
        self.assertTrue(f(["", "open fun StringsKt__StringsKt() {", "}"], []))
        self.assertFalse(f(["", "   open fun Inner() {", "   }"], []))
        self.assertFalse(f(["open fun A() {", "   x()", "}"], []))
        self.assertFalse(f(["open fun A() {", "}"], ["fun A() {"]))

    def test_when_break(self):
        f = self.tool.kt_when_break
        self.assertTrue(f(["            break"], []))
        self.assertTrue(f(["            var10000 = 1", "            break", "         }"],
                          ["            var10000 = 1", "         }"]))
        self.assertTrue(f(["               break;"], []))
        self.assertTrue(f(["      when (x) {", "         1 -> {", "            if (a) {", "               r = 1",
                           "               break", "            }", "            throw E()", "         }", "      }",
                           "      return r"],
                          ["      run label4@{", "         when (x) {", "            1 -> {", "               if (a) {",
                           "                  r = 1", "                  return@label4", "               }",
                           "               throw E()", "            }", "         }", "      }", "      return r"]))
        self.assertFalse(f(["            break@label3"], []))
        self.assertFalse(f(["            break", "            x()"], ["            x()"]))
        self.assertFalse(f(["      if (a) {", "         break", "      }"], ["      if (a) {", "         return@label4",
                                                                           "      }"]))

    def test_java_statement_jump(self):
        f = self.tool.kt_java_statement_jump
        self.assertTrue(f(["               break;"], ["               break"]))
        self.assertTrue(f(["               break label161;"], ["               return@label161"]))
        self.assertTrue(f(["               continue label5;"], ["               continue@label5"]))
        self.assertFalse(f(["               break label161;"], ["               return@label162"]))
        self.assertFalse(f(["               break;"], ["               continue"]))
        self.assertFalse(f(["               break"], ["               break"]))

    def test_catch_block_jump(self):
        f = self.tool.kt_catch_block_jump
        self.assertTrue(f([], ["               continue"]))
        self.assertTrue(f([""], ["            break@label7"]))
        self.assertFalse(f([], ["               return"]))
        self.assertFalse(f(["   x()"], ["   x()", "   continue"]))

    def test_supertype_type_arguments(self):
        f = self.tool.kt_supertype_type_arguments
        self.assertTrue(f(["class B<T>(c: C) : A(c, true), D {"], ["class B<T>(c: C) : A<T>(", "   c, true", "), D {"]))
        self.assertTrue(f(["   inner class S : Q.Ch(1) {"], ["   inner class S : Q.Ch<Map<K, V>>(1) {"]))
        self.assertFalse(f(["class B<T>(c: C) : A(c), D {"], ["class B<T>(c: C) : A<T>(c, 1), D {"]))
        self.assertFalse(f(["   f(x)"], ["   f<T>(x)"]))


class TestAnonymousClassCreatedElsewhere(unittest.TestCase):
    """anonymous_class_created_elsewhere: an object's reads of the declaring class's `this` or of its own fields
    become the creating method's variables or the creating instance's `R.this.$f$inlined`."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_captures_read_at_the_creator(self):
        f = self.tool.anonymous_class_created_elsewhere
        self.assertTrue(f(["   if (Outer.this.element < this.$count) {", "      Outer.this.element++"],
                          ["   if (skipped.element < Outer$f$$inlined$g$1.this.$count$inlined) {",
                           "      skipped.element++"]))
        self.assertTrue(f(["   val n: Int = (Outer.this as Array<Any>).length"],
                          ["   val n: Int = Outer$f$$inlined$g$1.this.$flows$inlined.length"]))
        self.assertTrue(f(["   val n: Int = ((Object[])Outer.this).length"],
                          ["   val n: Int = Outer$f$$inlined$g$1.this.$flows$inlined.length"]))
        self.assertTrue(f(["   val c: Sink = Outer$f$$inlined$g$1.super.$this_flow"], ["   val c: Sink = collector"]))
        self.assertTrue(f(["var empty: BooleanRef", "   Outer.this.element = false"],
                          ["val empty: BooleanRef", "   empty.element = false"]))
        self.assertTrue(f(["   Outer.this.element = false", "   Outer.m$Outer(x, this.$marker)"],
                          ["   empty.element = false", "   Outer.access$m$Outer(x, marker)"]))

    def test_other_changes_are_not(self):
        f = self.tool.anonymous_class_created_elsewhere
        self.assertFalse(f(["   val x = a.b"], ["   val x = a.c"]))
        self.assertFalse(f(["   Outer.this.element = false"], ["   empty.element = true"]))
        self.assertFalse(f(["   Outer.this.element++"], ["   g(skipped).element++"]))
        self.assertFalse(f(["   val v = Outer.this"], ["   val v = Outer.this"]))


class TestKtprint6Constructors(unittest.TestCase):
    """kt_legacy_constructor_parameter_checks and kt_hidden_empty_super_call: a constructor's Kotlin 1.3 parameter
    checks and its super call, and an empty super call that `hideEmptySuper` removed."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_legacy_checks(self):
        f = self.tool.kt_legacy_constructor_parameter_checks
        self.assertTrue(f(['   init {', '      Intrinsics.checkParameterIsNotNull(sink, "sink")', '      super()',
                           '      this.sink = sink'], ['   init {', '      this.sink = sink']))
        self.assertTrue(f(['      Intrinsics.checkParameterIsNotNull(a, "a")', '      super(a.length())'], []))
        self.assertTrue(f(['   init {', '      Intrinsics.checkParameterIsNotNull(a, "a")', '      super()', '   }', ''],
                          []))
        self.assertTrue(f(['   public constructor(s: Sink, d: D) : Intrinsics.checkParameterIsNotNull(s, "s") {',
                           '      Intrinsics.checkParameterIsNotNull(d, "d")', '      this(Okio.buffer(s), d)', '   }'],
                          ['   public constructor(s: Sink, d: D) : this(Okio.buffer(s), d)']))
        self.assertTrue(f(['   constructor(s: Source, a: String) : Intrinsics.checkParameterIsNotNull(s, "s") {',
                           '      super(s)'], ['   constructor(s: Source, a: String) : super(s) {']))
        self.assertTrue(f(['   constructor(d: ByteArray) : Intrinsics.checkParameterIsNotNull(d, "d") {', '      super()'],
                          ['   constructor(d: ByteArray) {']))

    def test_legacy_checks_other_changes_are_not(self):
        f = self.tool.kt_legacy_constructor_parameter_checks
        self.assertFalse(f(['      Intrinsics.checkParameterIsNotNull(a, "a")', '      foo(a)'], ['      foo(a)']))
        self.assertFalse(f(['      Intrinsics.checkParameterIsNotNull(a, "a")'], []))
        self.assertFalse(f(['   init {', '      Intrinsics.checkParameterIsNotNull(a, "a")', '      super(a)',
                            '      this.a = a'], ['   init {', '      this.a = b']))

    def test_hidden_empty_super_call(self):
        f = self.tool.kt_hidden_empty_super_call
        self.assertTrue(f(['class A(x: Int) : Base {'], ['class A(x: Int) : Base() {']))
        self.assertTrue(f(['   public object Single : Base'], ['   public object Single : Base()']))
        self.assertTrue(f(['class T(n: Int) : Gen<java.lang.String> {'], ['class T(n: Int) : Gen<String>() {']))
        self.assertTrue(f(['      e: Int', '   )', '   : Dispatcher {'], ['   e: Int', ') : Dispatcher() {']))
        self.assertTrue(f(['   public constructor(s: String)  {'], ['   public constructor(s: String) {']))
        self.assertTrue(f(['   public constructor(s: String) : this.remember(s)'],
                          ['   public constructor(s: String) {', '      this.remember(s)', '   }']))
        self.assertTrue(f([], ['   public constructor()', '']))
        self.assertTrue(f([''], ['   public constructor() {', '      this.h = H()', '   }', '']))

    def test_hidden_empty_super_call_other_changes_are_not(self):
        f = self.tool.kt_hidden_empty_super_call
        self.assertFalse(f(['class A(x: Int) : Base() {'], ['class A(x: Int) : Base {']))
        self.assertFalse(f(['class A(x: Int) : Base {'], ['class A(x: Int) : Base(x) {']))
        self.assertFalse(f(['class A(x: Int) : Base(x) {'], ['class A(x: Int) : Other() {']))
        self.assertFalse(f([], ['   fun f()']))
        self.assertFalse(f(['   public constructor(s: String) : this(s, 1)'],
                           ['   public constructor(s: String) {', '      this(s, 1)', '   }']))

    def test_file_context(self):
        f = self.tool.kt_ktprint6_file
        a = ['class G(sink: Sink) : Sink {', '   val sink: Sink', '   val buffer: Buffer = Buffer()', '',
             '   init {', '      Intrinsics.checkParameterIsNotNull(sink, "sink")', '      super(sink.x())',
             '      this.sink = sink', '   }', '}']
        b = ['class G(sink: Sink) : Sink(sink.x()) {', '   val sink: Sink', '   val buffer: Buffer = Buffer()', '',
             '   init {', '      this.sink = sink', '   }', '}']
        self.assertTrue(f(a, b, 0, 1, 0, 1, set()))
        self.assertTrue(f(a, b, 5, 7, 5, 5, set()))
        bad = list(b)
        bad[0] = 'class G(sink: Sink) : Sink(sink.x(), 1) {'
        self.assertFalse(f(a, bad, 0, 1, 0, 1, set()))
        self.assertTrue(self.tool.kt_ktprint6_moved_initialiser(
            ['   val data: ByteArray = ByteArray(8)'], ['   val data: ByteArray'], ['      this.data = ByteArray(8)']))
        self.assertFalse(self.tool.kt_ktprint6_moved_initialiser(
            ['   val data: ByteArray = ByteArray(8)'], ['   val data: ByteArray'], ['      this.data = data']))



class TestKtCompoundBitwiseAssignment(unittest.TestCase):
    """kt_compound_bitwise_assignment: `x |= y` became `x = x or y`."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_compound_bitwise(self):
        f = self.tool.kt_compound_bitwise_assignment
        self.assertTrue(f(['      this.label |= Integer.MIN_VALUE'], ['      this.label = this.label or Integer.MIN_VALUE']))
        self.assertTrue(f(['   this.set |= 1 shl id'], ['   this.set = this.set or (1 shl id)']))
        self.assertTrue(f(['   vx >>>= 4', '   f(vx)'], ['   vx = vx ushr 4', '   f(vx)']))
        self.assertFalse(f(['   x |= y'], ['   x = x and y']))
        self.assertFalse(f(['   x |= y'], ['   x = z or y']))
        self.assertFalse(f(['   x += y'], ['   x = x + y']))
        self.assertFalse(f(['   this.label |= Integer.MIN_VALUE', '   this.f(null, this)'],
                           ['   this.label = this.label or Integer.MIN_VALUE', '   this.f(null, this)', '   g()']))



class TestKtSwitchHeadIndent(unittest.TestCase):
    """kt_switch_head_indent: the column-0 lines before a `when` indented."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_switch_head_indent(self):
        f = self.tool.kt_switch_head_indent
        self.assertTrue(f(['      val a: Int = f()', 'val b: Int = g(a)', 'var v: Int'],
                          ['      val a: Int = f()', '      val b: Int = g(a)', '      var v: Int']))
        self.assertFalse(f(['val b: Int = g(a)'], ['      val b: Int = g(b)']))
        self.assertFalse(f(['   val b: Int = g(a)'], ['      val b: Int = g(a)']))


class TestCapturedVars(unittest.TestCase):
    """captured_var_clash_name: captured variables read by the creating variable's clashing name;
    captured_var_through_cast: an object's own field read as the variable its `new` cast."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_clash_name_reads(self):
        f = self.tool.captured_var_clash_name
        self.assertTrue(f(["                  return first;"], ["                  return firstx;"]))
        self.assertTrue(f(["   int sum = apply(base -> base + total, 1);"],
                          ["   int sum = apply(base -> base + totalx, 1);"]))
        self.assertTrue(f(["      fromDownstream.element = var9 as T"], ["      fromDownstreamx.element = var9 as T"]))
        self.assertFalse(f(["   int first = other[0];"], ["   int firstx = other[0];"]))
        self.assertFalse(f(["   val first: Int = 1"], ["   val firstx: Int = 1"]))
        self.assertFalse(f(["   fun f(first: Int)"], ["   fun f(firstx: Int)"]))
        self.assertFalse(f(["   return this.first;"], ["   return this.firstx;"]))
        self.assertFalse(f(["   return first;"], ["   return second;"]))

    def test_cast_capture_reads(self):
        f = self.tool.captured_var_through_cast
        self.assertTrue(f(["      this.$cont.resumeWith(Unit.INSTANCE)"], ["      cancellable$iv.resumeWith(Unit.INSTANCE)"]))
        self.assertTrue(f(["   {", "      this.$handle = `$handle`", "   }", ""], []))
        self.assertFalse(f(["      this.$cont.resumeWith(x)"], ["      cancellable$iv.resumeWith(y)"]))
        self.assertFalse(f(["      this.cont.resumeWith(x)"], ["      cont.resumeWith(x)"]))
        self.assertFalse(f(["   {", "      this.$handle = other", "   }"], []))
        self.assertFalse(f(["   a.b()"], []))


class TestKtOperators(unittest.TestCase):
    """kt_operator_precedence: parentheses Kotlin's precedence needs; kt_narrow_bitwise_operand: Byte, Short and
    Char operands of bitwise operators widened; kt_reference_inequality: an identity `!=` written `!==`."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_operator_precedence(self):
        f = self.tool.kt_operator_precedence
        self.assertTrue(f(['      return x or 1 shl k'], ['      return x or (1 shl k)']))
        self.assertTrue(f(['   val c: Int = b2 xor 3968 xor b1 shl 6'], ['   val c: Int = b2 xor 3968 xor (b1 shl 6)']))
        self.assertTrue(f(['   if (method == "GET" xor true) {'], ['   if ((method == "GET") xor true) {']))
        self.assertTrue(f(['   return x ?: a && b'], ['   return x ?: (a && b)']))
        self.assertTrue(f(['   return (this.set and 1 shl id) != 0'], ['   return (this.set and (1 shl id)) != 0']))
        self.assertTrue(f(['   return a with b or c'], ['   return a with (b or c)']))
        self.assertTrue(f(['   val v: Int = f(a,', '      x or y shl 2)'], ['   val v: Int = f(a,', '      x or (y shl 2))']))
        self.assertFalse(f(['      return x or 1 shl k'], ['      return (x or 1) shl k']))
        self.assertFalse(f(['   return a or b and c'], ['   return (a or b) and c']))
        self.assertFalse(f(['   return x ?: a && b'], ['   return (x ?: a) && b']))
        self.assertFalse(f(['   return a or b'], ['   return a or b']))
        self.assertFalse(f(['   return a or b'], ['   return a or c']))
        self.assertFalse(f(['   return x or 1 shl k'], ['   return x or (1 shl k) + 0']))
        self.assertTrue(f(['   return (short)(a shl 8 or b and 255)'], ['   return (a shl 8 or (b and 255)).toShort()']))
        self.assertFalse(f(['   return (int)x and 255'], ['   return x.toInt() and 255']))
        self.assertFalse(f(['   return (a or b) shl 1'], ['   return ((a or b)) shl 1']))

    def test_operator_precedence_context(self):
        f = self.tool.kt_operator_precedence_context
        a = ['   val v: Int = (', '      v or v ushr 1', '   ) ushr 2 or w and 3', '   g()']
        b = ['   val v: Int = (', '      v or (v ushr 1)', '   ) ushr 2 or (w and 3)', '   g()']
        self.assertTrue(f(a, b, 1, 3, 1, 3, set()))
        bad = ['   val v: Int = (', '      (v or v) ushr 1', '   ) ushr 2 or (w and 3)', '   g()']
        self.assertFalse(f(a, bad, 1, 3, 1, 3, set()))

    def test_operator_precedence_undone(self):
        f = self.tool.kt_prec_undone
        self.assertEqual(f(['   return x or (1 shl k)']), ['   return x or 1 shl k'])
        self.assertEqual(f(['   if (n > 1 || (a == b) xor true) {']), ['   if (n > 1 || a == b xor true) {'])
        self.assertEqual(f(['   return a with (b or c)']), ['   return a with b or c'])
        for kept in ('   return (a or b) shl 1', '   return (flags and 4) != 0', '   f((a or b))', '   return -(a or b)',
                     '   return (a shl 1) shl 2', '   return a or (b or c)'):
            self.assertEqual(f([kept]), [kept])

    def test_narrow_bitwise_operand(self):
        f = self.tool.kt_narrow_bitwise_operand
        self.assertTrue(f(['   return b and 255'], ['   return b.toInt() and 255']))
        self.assertTrue(f(['   return c xor 32'], ['   return c.code xor 32']))
        self.assertTrue(f(['   return x shl n'], ['   return x shl n.toInt()']))
        self.assertTrue(f(['   return (b.inv()).toByte()'], ['   return (b.toInt().inv()).toByte()']))
        self.assertTrue(f(['   val v: Long = (data[p++] and 255L) shl 8'], ['   val v: Long = (data[p++].toLong() and 255L) shl 8']))
        self.assertFalse(f(['   val v: Long = data[p++] and 255L shl 8'], ['   val v: Long = (data[p++].toLong() and 255L) shl 8']))
        self.assertTrue(f(['   return this.bytes[i] shl 8 or this.bytes[i + 1] and 255'],
                          ['   return this.bytes[i].toInt() shl 8 or (this.bytes[i + 1].toInt() and 255)']))
        self.assertTrue(f(['               buffer[i] ^= key[var10]'],
                          ['               buffer[i] = (buffer[i].toInt() xor key[var10].toInt()).toByte()']))
        self.assertFalse(f(['   return b and 255'], ['   return b and 255']))
        self.assertFalse(f(['   return f(b)'], ['   return f(b.toInt())']))
        self.assertFalse(f(['   return b and 255'], ['   return b.toInt() and 254']))
        self.assertFalse(f(['   return x or b and 255'], ['   return (x or b.toInt()) and 255']))

    def test_reference_inequality(self):
        f = self.tool.kt_reference_inequality
        self.assertTrue(f(['      if (state != CLOSED) {'], ['      if (state !== CLOSED) {']))
        self.assertTrue(f(['   return "${if (a != b) "x" else ""}"'], ['   return "${if (a !== b) "x" else ""}"']))
        self.assertTrue(f(['   while (s != head && t == u) {'], ['   while (s !== head && t === u) {']))
        self.assertFalse(f(['      if (state != CLOSED) {'], ['      if (state === CLOSED) {']))
        self.assertFalse(f(['      if (state !== CLOSED) {'], ['      if (state != CLOSED) {']))
        self.assertFalse(f(['      if (state != CLOSED) {'], ['      if (state !== CLOSED && x) {']))

    def test_source_locals(self):
        f = self.tool.source_locals
        self.assertTrue(f(['      return 31 * result + this.n;'], ['      result = 31 * result + this.n;', '      return result;']))
        self.assertTrue(f(['      return b * mul;'], ['      b *= mul;', '      return b;']))
        self.assertTrue(f(['      return (Map)map;'], ['      Map<K, V> result = (Map<K, V>)map;', '      return result;']))
        self.assertTrue(f(['      return c ? a : b;'], ['      if (c) {', '         T x = a;', '         return x;', '      } else {',
                                                         '         return b;', '      }']))
        self.assertTrue(f(['      char[] d = new char[]{\'%\', \'\\u0000\', x};'],
                          ['      char[] d = new char[3];', "      d[0] = '%';", '      d[2] = x;']))
        self.assertTrue(f(['      for (String s : src.subList(1, 3)) {'],
                          ['      List<String> part = src.subList(1, 3);', '', '      for (String s : part) {']))
        self.assertTrue(f(['      for (Object e : this) {'], ['      Iterator<E> it = this.iterator();', '',
                                                             '      while (it.hasNext()) {', '         Object e = it.next();']))
        self.assertTrue(f(['      .map(p -> f(p))'], ['      .map(p -> {', '         R r = f(p);', '         return r;', '      })']))
        self.assertTrue(f(['   return java.lang.String(charArrayOf(D[b shr 2], D[b and 3]))'],
                          ['   val result: CharArray = CharArray(2)', '   result[0] = D[b shr 2]', '   result[1] = D[b and 3]',
                           '   return java.lang.String(result)']))
        self.assertTrue(f(['      int a = 1;', '      long t = 0L;', '      int n;'], ['      int a = 1;', '      int n;', '      long t = 0L;']))
        self.assertFalse(f(['      return x + 1;'], ['      return x + 2;']))
        self.assertFalse(f(['      return 31 * result + this.n;'], ['      result = 31 * result + this.m;', '      return result;']))
        self.assertFalse(f(['      int n = 1;'], ['      long n = 1;']))
        self.assertFalse(f(['      return (Foo)x;'], ['      return x;']))
        self.assertFalse(f(['      int a = (Foo)x;'], ['      int a = x;']))
        self.assertFalse(f(['      if (c) {', '         a();', '      }', '      b();'], ['      if (c) {', '         a();', '         b();', '      }']))

    def test_source_locals_diamond(self):
        f = self.tool.source_locals_diamond
        self.assertTrue(f(['      return new P(n, s);'], ['      P<A, B> p = new P<>(n, s);', '      return p;']))
        self.assertFalse(f(['      return new P(n, s);'], ['      P<A, B> p = new P(n, s);', '      return p;']))
        self.assertFalse(f(['      return new P(n, s);'], ['      return new P<>(n, s);']))

    def test_source_locals_kt_narrow(self):
        f = self.tool.source_locals_kt_narrow
        self.assertTrue(f(['   return java.lang.String(charArrayOf(D[b shr 4], D[b and 15]))'],
                          ['   val result: CharArray = CharArray(2)', '   result[0] = D[b.toInt() shr 4]',
                           '   result[1] = D[b.toInt() and 15]', '   return java.lang.String(result)']))
        self.assertFalse(f(['   return D[b shr 4]'], ['   return D[b.toInt() shr 4]']))

    def test_source_locals_context(self):
        f = self.tool.source_locals_context
        a = ['   void f() {', '      long total = 0L;', '', '      int nRead;', '      while (g()) {', '      }', '   }']
        b = ['   void f() {', '      int nRead;', '      long total = 0L;', '', '      while (g()) {', '      }', '   }']
        self.assertTrue(f(a, b, 1, 1, 1, 2, set()))
        self.assertTrue(f(a, b, 2, 4, 3, 3, set()))
        self.assertFalse(f(a, b[:1] + ['      int other;'] + b[2:], 1, 1, 1, 2, set()))

    def test_java_keyword_variable_name(self):
        f = self.tool.kt_java_keyword_variable_name
        self.assertTrue(f(['    return var1 * 31 + var2'], ['    return new * 31 + default']))
        self.assertTrue(f(['   val var2: Int = x', '   return var2'], ['   val int: Int = x', '   return int']))
        self.assertTrue(f(['   return var3'], ['   return `for`']))
        self.assertFalse(f(['   return var3'], ['   return for']))
        self.assertFalse(f(['   return var1 + var2'], ['   return new + new']))
        self.assertFalse(f(['   return var1'], ['   return value']))
        self.assertFalse(f(['   return var1'], ['   return this']))

    def test_default_accessor_flags(self):
        f = self.tool.kt_default_accessor_flags
        self.assertTrue(f(['   public final var x: Int', '      internal set', ''], ['   public final var x: Int']))
        self.assertTrue(f(['      internal final set', ''], []))
        self.assertFalse(f(['      private set'], []))
        self.assertFalse(f(['      internal set'], ['      public set']))
        self.assertFalse(f(['   val x: Int'], []))

    def test_unescaped_name(self):
        f = self.tool.kt_unescaped_name
        self.assertTrue(f(['      return $this$view.length'], ['      return `$this$view`.length']))
        self.assertTrue(f(['   val v: InputStream = System.in'], ['   val v: InputStream = System.`in`']))
        self.assertTrue(f(['   return "${this.a$b()}"'], ['   return "${this.`a$b`()}"']))
        self.assertTrue(f(['   f({ a, <anonymous parameter 1> ->'], ['   f({ a, _ ->']))
        self.assertFalse(f(['   return `$x`'], ['   return $x']))
        self.assertFalse(f(['   return a'], ['   return `a`']))
        self.assertFalse(f(['   return x$y'], ['   return `x$y` + 1']))

    def test_reserved_underscore_name(self):
        f = self.tool.kt_reserved_underscore_name
        self.assertTrue(f(['      for (_ in xs) {', '         __ += _'], ['      for (`_` in xs) {', '         `__` += `_`']))
        self.assertFalse(f(['   val _x = 1'], ['   val `_x` = 1']))
        self.assertFalse(f(['   for (_ in xs) {'], ['   for (`_` in ys) {']))

    def test_java_type_syntax(self):
        f = self.tool.kt_java_type_syntax
        self.assertTrue(f(['public interface R<T extends java.lang.Comparable<? super T>> {'],
                          ['public interface R<T : java.lang.Comparable<in T>> {']))
        self.assertTrue(f(['   val f: Function0 = object : Function0<T[]> {'], ['   val f: Function0 = object : Function0<Array<T>> {']))
        self.assertTrue(f(['public interface C<T extends java.lang.Throwable & C<T>> {'],
                          ['public interface C<T> where T : java.lang.Throwable, T : C<T> {']))
        self.assertTrue(f(['   return c == int[]::class.java'], ['   return c == IntArray::class.java']))
        self.assertTrue(f(['   fun <R> sink(items: MutableList<R>): Int {'], ['   fun <R> sink(items: MutableList<in R>): Int {']))
        self.assertTrue(f(['class K : KClass<Object> {'], ['class K : KClass<Any> {']))
        self.assertFalse(f(['   return c == int[]::class.java'], ['   return c == LongArray::class.java']))
        self.assertFalse(f(['   val x: List<T> = y'], ['   val x: List<T> = z']))

    def test_ktprint7_file(self):
        f = self.tool.kt_ktprint7_file
        a = ['class A {', '   var x: Int', '      internal set', '   fun f() = this.this$0.g()', '}']
        b = ['class A {', '   var x: Int', '   fun f() = this.`this$0`.g()', '}']
        self.assertTrue(f(a, b, 2, 4, 2, 3, set()))
        c = ['class A {', '   var x: Int', '   fun f() = this.`this$0`.g() + 1', '}']
        self.assertFalse(f(a, c, 2, 4, 2, 3, set()))
        d = ['class A {', '   val y: Int', '}']
        e = ['class A {', '   val z: Int', '}']
        self.assertFalse(f(d, e, 1, 2, 1, 2, set()))

    def test_ktprint7_touched(self):
        f = self.tool.kt_ktprint7_touched
        self.assertTrue(f(['      internal set'], []))
        self.assertTrue(f(['   return this.L$0'], ['   return this.`L$0`']))
        self.assertTrue(f(['   val x = System.in'], ['   val x = System.`in`']))
        self.assertTrue(f(['   fun <T extends A> f()'], ['   fun <T : A> f()']))
        self.assertFalse(f(['   for (`i$iv` in a..b) {'], ['   while (`i$iv` < b) {']))
        self.assertFalse(f(['   for (x in y) {'], ['   while (x < y) {']))


class TestJwrongFixes(unittest.TestCase):
    """vararg_array_length, synthetic_null_check_shape, var_merge_declared_type, compact_record_constructor,
    disjoint_generic_cast."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_vararg_array_length(self):
        f = self.tool.vararg_array_length
        self.assertTrue(f(['      return Arrays.asList();'], ['      return Arrays.asList(new String[n]);']))
        self.assertTrue(f(['      return count("p");'], ['      return count("p", new String[3]);']))
        self.assertTrue(f(['      return count();'], ['      return count(new int[n][]);']))
        self.assertTrue(f(['      List<C> s = Arrays.asList();'],
                          ['      List<C> s = Arrays.asList(', '         new C[this.leaderState.getSlices()]', '      );']))
        self.assertFalse(f(['      return count();'], ['      return count(new String[]{"a"});']))
        self.assertTrue(f(['      return count(a);'], ['      return count(a, new String[n]);']))
        self.assertFalse(f(['      return count(a);'], ['      return count(b, new String[n]);']))
        self.assertFalse(f(['      return count();'], ['      return count(new String[n], b);']))

    def test_synthetic_null_check_shape(self):
        f = self.tool.synthetic_null_check_shape
        self.assertTrue(f([], ['      Objects.requireNonNull(o);']))
        self.assertTrue(f([], ['         Objects.requireNonNull(this.impl);']))
        self.assertTrue(f([], ['      o.getClass();']))
        self.assertFalse(f([], ['      Objects.requireNonNull(o, "o");']))
        self.assertFalse(f([], ['      o = Objects.requireNonNull(p);']))
        self.assertFalse(f(['      return 5;'], ['      Objects.requireNonNull(o);', '      return 6;']))

    def test_var_merge_declared_type(self):
        f = self.tool.var_merge_declared_type
        self.assertTrue(f(['      box = new Box<>();', '      box.add(1);', '      SIZES = box;'],
                          ['      Box<Integer> boxx = new Box<>();', '      boxx.add(1);', '      SIZES = boxx;']))
        self.assertFalse(f(['      box = new Box<>();', '      box.add(1);'], ['      Box<Integer> boxx = new Box<>();', '      boxx.add(2);']))

    def test_compact_record_constructor(self):
        f = self.tool.compact_record_constructor
        self.assertTrue(f([], ['      Checked(Object node, String reason) {', '         this.node = Objects.requireNonNull(node);',
                               '         this.reason = reason;', '      }', '']))
        self.assertTrue(f(['   public R {', '      if (a < 0) {', '         throw new E();', '      }'],
                          ['   public R(int a, @Nullable Map<K, V> m) {', '      this.a = Objects.requireNonNull(a);',
                           '      if (a < 0) {', '         throw new E();', '      }', '', '      this.m = Map.copyOf(m);']))
        self.assertFalse(f([], ['      Checked(Object node) {', '         foo(node);', '      }']))
        self.assertFalse(f(['   public R {', '      check(a);'], ['   public R(int a) {', '      check(b);', '      this.a = a;']))

    def test_disjoint_generic_cast(self):
        f = self.tool.disjoint_generic_cast
        self.assertTrue(f(['      return (Collector<E, ?, ImmutableList<E>>)TO_IMMUTABLE_LIST;'],
                          ['      return (Collector)TO_IMMUTABLE_LIST;']))
        self.assertTrue(f(['         ImmutableList<Class<? super T>> c = (ImmutableList<Class<? super T>>)C.FOR_RAW_TYPE',
                           '            .collectTypes(TypeToken.this.getRawTypes());'],
                          ['         ImmutableList<Class<? super T>> c = (ImmutableList)C.FOR_RAW_TYPE.collectTypes(TypeToken.this.getRawTypes());']))
        self.assertFalse(f(['      return (Range<C>)ALL;'], ['      return (Range)OTHER;']))
        self.assertFalse(f(['      return (Range<C>)ALL;'], ['      return ALL;']))
        self.assertFalse(f(['      return (Range)ALL;'], ['      return (Range<C>)ALL;']))

    def test_thrown_type_variable(self):
        f = self.tool.thrown_type_variable
        self.assertTrue(f(['      throw t;'], ['      throw (T)t;']))
        self.assertTrue(f(['      throw new SneakyThrows().throwIt(t);'],
                          ['      throw new SneakyThrows<RuntimeException>().throwIt(t);']))
        self.assertFalse(f(['      throw t;'], ['      throw (T)u;']))
        self.assertFalse(f(['      return (T)t;'], ['      return (T)t;']))
        self.assertFalse(f(['      return x;'], ['      return (T)x;']))
        self.assertFalse(f(['      throw new Box().get(t);'], ['      throw new Box<String>().get(t);']))


class TestKtMembersWithoutMetadata(unittest.TestCase):
    """kt_field_without_property and kt_constructor_without_metadata: Java members of classes without Kotlin
    metadata written in Kotlin."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_field_without_property(self):
        f = self.tool.kt_field_without_property
        self.assertTrue(f(["   open int label;", "   open Object L$0;", "   open long J$0;"],
                          ["   @JvmField internal var label: Int = 0", "   @JvmField internal var `L$0`: Any? = null",
                           "   @JvmField internal var `J$0`: Long = 0L"]))
        self.assertTrue(f(["      open double D$0;", "      open float F$0;"],
                          ["      @JvmField internal var `D$0`: Double = 0.0", "      @JvmField internal var `F$0`: Float = 0.0F"]))
        self.assertTrue(f(["   private Foo f;"], ["   private var f: Foo? = null"]))
        self.assertTrue(f(["   public final int n;"], ["   @JvmField public val n: Int"]))
        self.assertTrue(f(["   protected int n = 5;"], ["   @JvmField protected var n: Int = 5"]))
        self.assertFalse(f(["   open int label;"], ["   @JvmField internal var label: Long = 0L"]))
        self.assertFalse(f(["   open int label;"], ["   @JvmField internal var label: Int = 1"]))
        self.assertFalse(f(["   open int label;"], ["   @JvmField internal val label: Int = 0"]))
        self.assertFalse(f(["   open int label;"], ["   @JvmField internal var other: Int = 0"]))
        self.assertFalse(f(["   private int n;"], ["   @JvmField private var n: Int = 0"]))
        self.assertFalse(f(["   public static Foo INSTANCE;"], ["   @JvmField var INSTANCE: Foo? = null"]))
        self.assertFalse(f(["   open Object L$0;"], ["   @JvmField internal var `L$0`: Any = null"]))

    def test_constructor_without_metadata(self):
        f = self.tool.kt_constructor_without_metadata
        self.assertTrue(f(["         {"], ["         init {"]))
        self.assertTrue(f(["   fun `X$1`(`this$0`: Y, `$completion`: Continuation) {", "      super(`$completion`)"],
                          ["   public constructor(`this$0`: Y, `$completion`: Continuation) : super(`$completion`) {"]))
        self.assertTrue(f(["   fun `X$1`(", "      var1: String, var2: Int", "   ) {", "      super(var1, false, 2, null)"],
                          ["   public constructor(var1: String, var2: Int) : super(var1) {"]))
        self.assertTrue(f(["   fun `X$1`(var1: Function1) {"], ["   public constructor(var1: Function1) {"]))
        self.assertTrue(f(["   open fun `X$1`() {"], ["   internal constructor() {"]))
        self.assertTrue(f(["   fun `X$1`(a: Int) {", "      this(a, 0)"], ["   constructor(a: Int) : this(a, 0) {"]))
        self.assertFalse(f(["         {"], ["         init"]))
        self.assertTrue(f(["   val task: TimerTask = object : TimerTask {", "      {"],
                          ["   val task: TimerTask = object : TimerTask() {", "      init {"]))
        self.assertFalse(f(["   val task: TimerTask = object : TimerTask {", "      {"],
                           ["   val task: Other = object : TimerTask() {", "      init {"]))
        self.assertFalse(f(["   fun `X$1`(a: Int) {", "      super(a)"], ["   public constructor(a: Int) : super(a, 1) {"]))
        self.assertFalse(f(["   fun `X$1`(a: Int) {", "      super(a)"], ["   public constructor(a: Long) : super(a) {"]))
        self.assertFalse(f(["   fun `X$1`(a: Int) {", "      super(a)"], ["   public constructor(a: Int) {", "      super(a)"]))
        self.assertFalse(f(["   fun `X$1`(a: Int) {", "      foo(a)"], ["   public constructor(a: Int) {"]))
        self.assertFalse(f(["   fun get(a: Int) {"], ["   public fun get(a: Int) {"]))
        self.assertTrue(f(["// $VF: local visibility outside of methodSupplier", "internal class `X$1` : Task {",
                           "   fun `X$1`(var1: String) {", "      super(var1, false, 2, null)"],
                          ["/* $VF: local visibility outside of method */ internal class `X$1` : Task {",
                           "   public constructor(var1: String) : super(var1) {"]))
        self.assertFalse(f(["// $VF: local visibility outside of methodSupplier", "internal class `X$1` : Task {",
                            "   fun `X$1`(var1: String) {", "      super(var1)"],
                           ["/* $VF: local visibility outside of method */ internal class `X$1` : Task {",
                            "   public constructor(var1: Int) : super(var1) {"]))


    def test_delegation_call_arguments(self):
        f = self.tool.kt_delegation_call_arguments
        self.assertTrue(f(["   public constructor(pattern: String)  {", "      val var10001: Pattern = Pattern.compile(pattern)",
                           "      this(var10001)", "   }"],
                          ["   public constructor(pattern: String) : this(Pattern.compile(pattern))"]))
        self.assertTrue(f(['   public constructor(scheme: String, realm: String) : Intrinsics.checkParameterIsNotNull(scheme, "scheme") {',
                           '      Intrinsics.checkParameterIsNotNull(realm, "realm")',
                           '      val var10002: java.util.Map = Collections.singletonMap("realm", realm)',
                           '      Intrinsics.checkExpressionValueIsNotNull(var10002, "singletonMap(\\"realm\\", realm)")',
                           '      this(scheme, var10002)', '   }'],
                          ['   public constructor(scheme: String, realm: String) : this(scheme, Collections.singletonMap("realm", realm))']))
        self.assertTrue(f(["   constructor(a: String, b: String)  {", "      val var10001: String = f(a)", "      val var10002: String = g(b)",
                           "      this(var10001, var10002, 1)", "   }"],
                          ["   constructor(a: String, b: String) : this(f(a), g(b), 1)"]))
        self.assertFalse(f(["   public constructor(pattern: String)  {", "      val var10001: Pattern = Pattern.compile(pattern)",
                            "      this(var10001)", "   }"],
                           ["   public constructor(pattern: String) : this(Pattern.compile(other))"]))
        self.assertFalse(f(["   public constructor(pattern: String)  {", "      val x: Pattern = Pattern.compile(pattern)",
                            "      this(x)", "   }"],
                           ["   public constructor(pattern: String) : this(Pattern.compile(pattern))"]))
        self.assertFalse(f(["   public constructor(pattern: String)  {", "      log(pattern)", "      val var10001: Pattern = Pattern.compile(pattern)",
                            "      this(var10001)", "   }"],
                           ["   public constructor(pattern: String) : this(Pattern.compile(pattern))"]))


class TestKtValid(unittest.TestCase):
    """The ktvalid fixes (Kotlin output kotlinc accepts): each hunk counts under the first of them that changed it,
    and is accepted only when, with their forms written alike, both sides are the same tokens."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def r(self, fix):
        return self.tool.RECOGNISERS[fix]

    def test_accessor_backing_field(self):
        f = self.r('kt_accessor_backing_field')
        self.assertTrue(f(['      public get() {', '         return this.maxRequests'],
                          ['      public get() {', '         return field']))
        self.assertTrue(f(['         TestA.shared = "$value."'], ['         field = "$value."']))
        self.assertFalse(f(['         return this.maxRequests'], ['         return this.other']))

    def test_string_literal_escape(self):
        f = self.r('kt_string_literal_escape')
        self.assertTrue(f(['   check(x, "$this$commonWrite")'], ['   check(x, "\\$this\\$commonWrite")']))
        self.assertTrue(f(['   return "$key258EAF"'], ['   return "${key}258EAF"']))
        self.assertFalse(f(['   return "$key258EAF"'], ['   return "${key}258EAG"']))

    def test_abstract_function_body(self):
        f = self.r('kt_abstract_function_body')
        self.assertTrue(f(['   public abstract fun read(): Long {', '   }'], ['   public abstract fun read(): Long']))
        self.assertFalse(f(['   public fun read(): Long {', '   }'], ['   public fun read(): Long']))

    def test_misplaced_modality_and_override(self):
        self.assertTrue(self.r('kt_misplaced_modality')(['      public final get() {'], ['      public get() {']))
        self.assertTrue(self.r('kt_misplaced_modality')(['internal final val BASE64: ByteArray'], ['internal val BASE64: ByteArray']))
        self.assertTrue(self.r('kt_missed_override')(['   public open fun buffer(): Buffer {'], ['   public override fun buffer(): Buffer {']))
        self.assertFalse(self.r('kt_missed_override')(['   public open fun buffer(): Buffer {'], ['   public override fun buffer(): Buffer? {']))

    def test_initializers(self):
        self.assertTrue(self.r('kt_default_property_initializer')(['   private final var closed: Boolean'],
                                                                   ['   private final var closed: Boolean = false']))
        self.assertTrue(self.r('kt_companion_property_initializer')(
            ['      public final val TLS_A: CipherSuite'], ['      public final val TLS_A: CipherSuite = Companion.init("A", 1)']))
        self.assertFalse(self.r('kt_default_property_initializer')(['   private final var closed: Boolean = true'],
                                                                    ['   private final var closed: Boolean = false']))

    def test_names(self):
        self.assertTrue(self.r('kt_outer_this')(['      if (Pipe.this.sourceClosed) {'], ['      if (this@Pipe.sourceClosed) {']))
        self.assertTrue(self.r('kt_extension_receiver')(['   return `$this$utf8Size`.length()'], ['   return this.length']))
        self.assertTrue(self.r('kt_extension_receiver')(['   return Integer.bitCount(var0)'], ['   return Integer.bitCount(this)']))
        self.assertTrue(self.r('kt_object_instance')(['      SegmentPool.INSTANCE.recycle(s)'], ['      SegmentPool.recycle(s)']))
        self.assertTrue(self.r('kt_super_property_access')(['      return this.isActive'], ['      return super.isActive']))
        self.assertFalse(self.r('kt_outer_this')(['      if (Pipe.this.size == 1) {'], ['      if (this@Pipe.size == 2) {']))

    def test_types_and_members(self):
        self.assertTrue(self.r('kt_throws_class_name')(['   @Throws(java/io/IOException::class)'], ['   @Throws(IOException::class)']))
        self.assertTrue(self.r('kt_throwable_cast')(['   throw EOFException() as java.lang.Throwable'], ['   throw EOFException()']))
        self.assertTrue(self.r('kt_mapped_java_member')(['   if (s.charAt(i) == c && s.length() > 1) {'],
                                                        ['   if (s[i] == c && s.length > 1) {']))
        self.assertTrue(self.r('kt_mapped_java_type')(['   val tag: java.lang.String = f()'], ['   val tag: String = f()']))
        self.assertTrue(self.r('kt_nested_annotation_argument')(['@Deprecated(replaceWith = @ReplaceWith(expression = "x"))'],
                                                                ['@Deprecated(replaceWith = ReplaceWith(expression = "x"))']))
        self.assertFalse(self.r('kt_mapped_java_member')(['   if (s.length() > 1) {'], ['   if (s.length > 2) {']))
        self.assertTrue(self.r('kt_setter_parameter_name')(['      public set(<set-?>) {', '         field = var1'],
                                                           ['      public set(value) {', '         field = value']))
        self.assertTrue(self.r('kt_java_class_literal')(['   val n: String = C.class.getName()'],
                                                        ['   val n: String = C::class.java.getName()']))
        self.assertTrue(self.r('kt_java_class_literal')(['   m = c.getMethod("set", byte[].class, String[][].class)'],
                                                        ['   m = c.getMethod("set", ByteArray::class.java, Array<Array<String>>::class.java)']))
        self.assertFalse(self.r('kt_java_class_literal')(['   m = c.getMethod("set", byte[].class)'],
                                                         ['   m = c.getMethod("get", ByteArray::class.java)']))


class TestJavacLayout(unittest.TestCase):
    """jvm_javac_layout: the same statements with ifs and exits laid out the way javac placed them."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_layouts(self):
        f = self.tool.javac_layout
        self.assertTrue(f(['      return x == null ? 0 : x.hashCode();'],
                          ['      if (x == null) {', '         return 0;', '      }', '', '      return x.hashCode();']))
        self.assertTrue(f(['      return (List<T>)(c ? a : b);'],
                          ['      if (c) {', '         return (List<T>)a;', '      }', '', '      return (List<T>)b;']))
        self.assertTrue(f(['      return v > 0;'],
                          ['      if (v > 0) {', '         return true;', '      }', '', '      return false;']))
        self.assertTrue(f(['      if (index >= 0 && index < n) {', '         return t[index];', '      } else {',
                           '         throw new X();', '      }'],
                          ['      if (index < 0 || index >= n) {', '         throw new X();', '      }', '',
                           '      return t[index];']))
        self.assertTrue(f(['      if (s != null) {', '         this.add(s);', '      }'],
                          ['      if (s == null) {', '         return;', '      }', '', '      this.add(s);']))
        self.assertTrue(f(['         if (!s.isEmpty()) {', '            n++;', '         }'],
                          ['         if (s.isEmpty()) {', '            continue;', '         }', '', '         n++;']))
        self.assertTrue(f(['      int y;', '      if (c) {', '         y = 1;', '      } else {', '         y = 2;',
                           '      }'],
                          ['      int y;', '      if (!c) {', '         y = 2;', '      } else {', '         y = 1;',
                           '      }']))
        self.assertTrue(f(['      k = a != null && b ? x : y;'], ['      k = a == null || !b ? y : x;']))

    def test_other_changes_are_not(self):
        f = self.tool.javac_layout
        self.assertFalse(f(['      return x == null ? 0 : x.hashCode();'],
                           ['      if (x == null) {', '         return 1;', '      }', '', '      return x.hashCode();']))
        self.assertFalse(f(['      if (s != null) {', '         this.add(s);', '      }'],
                           ['      if (t == null) {', '         return;', '      }', '', '      this.add(s);']))
        self.assertFalse(f(['      if (s != null) {', '         this.add(s);', '      }'],
                           ['      if (s == null) {', '         return;', '      }', '', '      this.remove(s);']))
        self.assertFalse(f(['      return a + 1;'], ['      return a - 1;']))
        self.assertFalse(f(['      k = c ? x : y;'], ['      k = c ? x : z;']))

    def test_option_names_are_ids(self):
        self.assertIn("jvm_javac_layout", check.fix_ids())
class TestKtOps2(unittest.TestCase):
    """kt_negated_equality, kt_unboxing_cast_type (kt_unboxed_cast with kt_operator_precedence),
    kt_binary_numeric_promotion, kt_copied_equality and kt_operator_line_break."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_negated_equality(self):
        f = self.tool.kt_negated_equality
        self.assertTrue(f(['   return (method == "PROPFIND") xor true'], ['   return method != "PROPFIND"']))
        self.assertTrue(f(['   && !((a == b) xor true)'], ['   && !(a != b)']))
        self.assertTrue(f(['   return (a === b) xor true'], ['   return a !== b']))
        self.assertTrue(f(['   return c or ((a == b) xor true)'], ['   return c or (a != b)']))
        self.assertFalse(f(['   return (a == b) xor true'], ['   return a == b']))
        self.assertFalse(f(['   return (a == b) xor c'], ['   return a != b']))
        self.assertFalse(f(['   return (a == b) xor true'], ['   return a !== b']))

    def test_unboxing_cast_type(self):
        f = self.tool.kt_unboxing_cast_type
        self.assertTrue(f(['   if (predicate(e) as java.lang.Boolean) {'], ['   if (predicate(e) as Boolean) {']))
        self.assertTrue(f(['   return init(i) as Character'], ['   return init(i) as Char']))
        self.assertTrue(f(['   return f() as Character + 1'], ['   return (f() as Char).code + 1']))
        self.assertFalse(f(['   return x as java.lang.Boolean'], ['   return x as Int']))
        self.assertFalse(f(['   return x as java.lang.String'], ['   return x as String']))
        g = self.tool.kt_unboxed_cast
        self.assertTrue(g(['   if (!p(x) as java.lang.Boolean) {'], ['   if (!(p(x) as Boolean)) {']))
        self.assertTrue(g(['   return !x as java.lang.Boolean && y'], ['   return !(x as Boolean) && y']))
        self.assertFalse(g(['   if (!p(x) as java.lang.Boolean) {'], ['   if (!p(x) as Boolean) {']))
        self.assertFalse(g(['   if (!p(x) as java.lang.Boolean) {'], ['   if ((!p(x)) as Boolean) {']))

    def test_binary_numeric_promotion(self):
        f = self.tool.kt_binary_numeric_promotion
        self.assertTrue(f(['   return c - 48'], ['   return c.code - 48']))
        self.assertTrue(f(['   if (b == 61 && x) {'], ['   if (b.toInt() == 61 && x) {']))
        self.assertTrue(f(['   if (l != v) {'], ['   if (l != v.toLong()) {']))
        self.assertTrue(f(['   return 31 * this.first + this.last'], ['   return 31 * this.first.code + this.last.code']))
        self.assertTrue(f(['   return -c'], ['   return -c.code']))
        self.assertTrue(f(['   return (s.charAt(i) - 48) * 10'], ['   return (s.charAt(i).code - 48) * 10']))
        self.assertFalse(f(['   return f(c)'], ['   return f(c.code)']))
        self.assertFalse(f(['   return c - 48'], ['   return c.code - 49']))
        self.assertFalse(f(['   return c.code - 48'], ['   return c - 48']))

    def test_copied_equality(self):
        f = self.tool.kt_copied_equality
        self.assertTrue(f(['   public fun f(a: String, b: Boolean = a === "x"): Boolean {'],
                          ['   public fun f(a: String, b: Boolean = a == "x"): Boolean {']))
        self.assertTrue(f(['      return object : Base(a === c)'], ['      return object : Base(a == c)']))
        self.assertFalse(f(['      return a === c'], ['      return a == c']))

    def test_operator_line_break(self):
        f = self.tool.kt_operator_line_break
        self.assertTrue(f(['      return a', '         + b'], ['      return a +', '         b']))
        self.assertTrue(f(['      return 31', '         * (', '         x)'], ['      return 31 *', '         (', '         x)']))
        self.assertFalse(f(['      return a', '         && b'], ['      return a &&', '         b']))
        self.assertFalse(f(['      return a', '         + b'], ['      return a -', '         b']))
        self.assertFalse(f(['      return a + b'], ['      return a +', '         b', '         + c']))
        u = self.tool.kt2_unbroken
        self.assertEqual(u(['      return a +', '         b']), ['      return a', '         + b'])
        self.assertEqual(u(['      x = (a ==', '         b) &&', '         c']), ['      x = (a', '         == b) &&', '         c'])
        self.assertEqual(u(['   // a +', '   b']), ['   // a +', '   b'])

    def test_delegation(self):
        f = self.tool.kt_negated_equality
        self.assertTrue(f(['   if (a == b xor true) {', '      return (E)result'],
                          ['   if (a != b) {', '      return result as E']))
        self.assertFalse(f(['   if (a == b xor true) {', '      return x'], ['   if (a != b) {', '      return y']))


class TestJavaSugar(unittest.TestCase):
    """compound_assignment_shape, implicit_outer_instance and the try-with-resources fixes' recognisers."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_compound_assignment(self):
        f = self.tool.compound_assignment_shape
        self.assertTrue(f(['      this.b = this.b + buffer.remaining();'], ['      this.b += buffer.remaining();']))
        self.assertTrue(f(['      d[i] = (byte)(d[i] & ~s[i]);'], ['      d[i] &= ~s[i];']))
        self.assertTrue(f(['      this.n += 1;'], ['      this.n = this.n + 1;']))
        self.assertTrue(f(['      x |= x >>> n;'], ['      x = x >>> n | x;']))
        self.assertTrue(f(['      this.t = this.t + (a - b);'], ['      this.t += a - b;']))
        self.assertTrue(f(['      s.on((c, l) -> this.n = this.n + c.length());'], ['      s.on((c, l) -> this.n += c.length());']))
        self.assertTrue(f(['      Label var10000 = this.read(o);', '      var10000.flags = (short)(var10000.flags | 1);'],
                          ['      this.read(o).flags |= 1;']))
        self.assertFalse(f(['      this.b = this.b + 1;'], ['      this.b -= 1;']))
        self.assertFalse(f(['      this.b = this.c + 1;'], ['      this.b += 1;']))
        self.assertFalse(f(['      x = x - y;'], ['      x = y - x;']))
        self.assertFalse(f(['      this.b = this.b + 1;'], ['      this.b = this.b + 1;']))

    def test_implicit_outer_instance(self):
        f = self.tool.implicit_outer_instance
        self.assertTrue(f(['         return LocalCache.this.new KeyIterator();'], ['         return new LocalCache.KeyIterator();']))
        self.assertTrue(f(['      return A.this.new B(x, y);'], ['      return new A.C.B(x, y);']))
        self.assertFalse(f(['      return a.new B();'], ['      return new A.B();']))
        self.assertFalse(f(['      return A.this.new B(x);'], ['      return new A.B(y);']))

    def test_field_resource(self):
        f = self.tool.twr_field_resource
        self.assertTrue(f(['         try (this.watchService) {'], ['         try (WatchService ws = this.watchService) {']))
        self.assertFalse(f(['         try (this.watchService) {'], ['         try (WatchService ws = this.other) {']))

    def test_twr_resugar(self):
        f = self.tool.twr_resugar
        a = ['   public int get() {', '      Closeable c = this.lock.read();', '', '      int var2;', '      try {',
             '         var2 = this.size();', '      } catch (Throwable var5) {', '         if (c != null) {',
             '            try {', '               c.close();', '            } catch (Throwable var4) {',
             '               var5.addSuppressed(var4);', '            }', '         }', '', '         throw var5;',
             '      }', '', '      if (c != null) {', '         c.close();', '      }', '', '      return var2;', '   }']
        b = ['   public int get() {', '      try (Closeable c = this.lock.read()) {', '         return this.size();',
             '      }', '   }']
        self.assertTrue(f(a, b, 1, 23, 1, 4, set()))
        other = list(b)
        other[2] = '         return this.length();'
        self.assertFalse(f(a, other, 1, 23, 1, 4, set()))
        unclosed = [x.replace('c.close();', 'c.reset();') for x in a]
        self.assertFalse(f(unclosed, b, 1, 23, 1, 4, set()))




class TestJavacLayoutLoops(unittest.TestCase):
    """jvm_javac_layout in loop bodies: loop conditions as ifs, a for loop's init and update as statements, and
    the method around hunks spread over a loop."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_loop_headers(self):
        f = self.tool.javac_layout
        self.assertTrue(f(['      int i = 0;', '', '      while (i < n && a[i] != k) {', '         i++;', '      }'],
                          ['      int i;', '      for (i = 0; i < n; i++) {', '         if (a[i] == k) {',
                           '            break;', '         }', '      }']))
        self.assertTrue(f(['      E c;', '      do {', '         c = it.next();', '      } while (it.hasNext());', '',
                           '      return c;'],
                          ['      while (true) {', '         E c = it.next();', '         if (!it.hasNext()) {',
                           '            return c;', '         }', '      }']))
        self.assertTrue(f(['      Map.Entry e;', '      e = m.first();'], ['      Map.Entry e = m.first();']))

    def test_other_loop_changes_are_not(self):
        f = self.tool.javac_layout
        self.assertFalse(f(['      while (i < n && a[i] != k) {', '         i++;', '      }'],
                           ['      while (i < n) {', '         i++;', '      }']))
        self.assertFalse(f(['      for (i = 0; i < n; i++) {', '         s += i;', '      }'],
                           ['      for (i = 0; i < n; i += 2) {', '         s += i;', '      }']))

    def test_method_context(self):
        f = self.tool.javac_layout_method
        a = ['   int f(Iterator<E> it) {', '      E c;', '      do {', '         c = it.next();',
             '      } while (it.hasNext());', '', '      return c;', '   }']
        b = ['   int f(Iterator<E> it) {', '      while (true) {', '         E c = it.next();',
             '         if (!it.hasNext()) {', '            return c;', '         }', '      }', '   }']
        self.assertTrue(f(a, b, 1, 4, 1, 3, set()))
        lost = b[:3] + ['         if (!it.hasNext()) {', '            return null;', '         }', '      }', '   }']
        self.assertFalse(f(a, lost, 1, 4, 1, 3, set()))
class TestJwrong3(unittest.TestCase):
    """The recognisers of vararg_spread_resolution, unseen_class_overload, double_constant_literal,
    concat_operand_type and field_read_reuse."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_double_constant_literal(self):
        f = self.tool.double_constant_literal
        self.assertTrue(f(['      return bytes / 1.0737418E9F;'], ['      return bytes / 1.073741824E9;']))
        self.assertTrue(f(['      return d * Float.MAX_VALUE;'], ['      return d * 3.4028234663852886E38;']))
        self.assertTrue(f(['   static final double M = -9.223372E18F;'], ['   static final double M = -9.223372036854776E18;']))
        self.assertFalse(f(['      return bytes / 1.0737418E9F;'], ['      return bytes / 1.0737419E9;']))
        self.assertFalse(f(['      return x / 2.0F;'], ['      return y / 2.0;']))

    def test_concat_operand_type(self):
        f = self.tool.concat_operand_type
        self.assertTrue(f(['      return "r:" + (c ? 91 : 40) + a;'], ["      return \"r:\" + (c ? '[' : '(') + a;"]))
        self.assertTrue(f(['      return "v " + c2;'], ['      return "v " + (int)c2;']))
        self.assertTrue(f(['      return "f=" + this.f + " n=" + this.n;'], ['      return "f=" + (double)this.f + " n=" + (float)this.n;']))
        self.assertFalse(f(['      return "v " + c2;'], ['      return "v " + (String)c2;']))
        self.assertFalse(f(['      return "v " + c2;'], ['      return "v " + c3;']))

    def test_vararg_spread_resolution(self):
        f = self.tool.vararg_spread_resolution
        self.assertTrue(f(['      return this.build();'], ['      return this.build(new String[0]);']))
        self.assertTrue(f(['      setter.invoke(null, callbacks);'], ['      setter.invoke(null, (Object)callbacks);']))
        self.assertTrue(f(['      return pick("a");'], ['      return pick(new String[]{"a"});']))
        self.assertTrue(f(['      return name("x", 4);'], ['      return name("x", new Object[]{4});']))
        self.assertFalse(f(['      return pick("a");'], ['      return pick(new String[]{"b"});']))

    def test_unseen_class_overload(self):
        f = self.tool.unseen_class_overload
        self.assertTrue(f(['      return Sink.value(f);'], ['      return Sink.value((double)f);']))
        self.assertTrue(f(['      return Sink.flag(b);'], ['      return Sink.flag(b.booleanValue());']))
        self.assertTrue(f(['      logger.debug("to {}", addresses);'], ['      logger.debug("to {}", (Object)addresses);']))
        self.assertTrue(f(['      builder.value(kv.floatValue());'], ['      builder.value((double)kv.floatValue());']))
        self.assertFalse(f(['      return Sink.value(f);'], ['      return Sink.value(g);']))

    def test_field_read_reuse(self):
        f = self.tool.field_read_reuse_scope
        a = ['   int loop(Lock l) {', '      while (true) {', '         int i = l.count;', '         if (l.count <= 0) {',
             '            return -1;', '         }', '', '         if (l.tryInc(i)) {', '            return i;', '         }',
             '      }', '   }']
        b = ['   int loop(Lock l) {', '      int i;', '      while ((i = l.count) > 0) {', '         if (l.tryInc(i)) {',
             '            return i;', '         }', '      }', '', '      return -1;', '   }']
        self.assertTrue(f(a, b, 1, 6, 1, 3, set()))
        twice = list(b)
        twice[3] = '         if (l.tryInc(l.count)) {'
        self.assertFalse(f(a, twice, 1, 6, 1, 3, set()))
        renamed = [x.replace('tryInc', 'tryDec') for x in b]
        self.assertFalse(f(a, renamed, 1, 6, 1, 3, set()))
class TestJavaSwitch(unittest.TestCase):
    """switch_case_order, switch_table_fillers, switch_synthetic_default and jvm_javac_layout's boolean comparisons."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_case_order(self):
        f = self.tool.switch_case_order_scope
        a = ['   int f(int x) {', '      switch (x) {', '         case 1:', '            return 10;', '         case 3:',
             '            return 30;', '         case 4:', '         default:', '            return 0;', '      }', '   }']
        b = ['   int f(int x) {', '      switch (x) {', '         case 3:', '            return 30;', '         case 1:',
             '            return 10;', '         default:', '            return 0;', '      }', '   }']
        self.assertTrue(f(a, b, 2, 4, 2, 2, set()))
        changed = [x.replace('return 30;', 'return 31;') for x in b]
        self.assertFalse(f(a, changed, 2, 4, 2, 2, set()))

    def test_case_order_breaks(self):
        f = self.tool.switch_case_order_scope
        a = ['      switch (x) {', '         case 1:', '            y = 1;', '            break;', '         case 2:',
             '            y = 2;', '      }']
        b = ['      switch (x) {', '         case 2:', '            y = 2;', '            break;', '         case 1:',
             '            y = 1;', '      }']
        self.assertTrue(f(a, b, 1, 4, 1, 4, set()))

    def test_table_fillers(self):
        f = self.tool.switch_table_fillers_scope
        a = ['      switch (x) {', '         case 1:', '            return 1;', '         case 3:', '         default:',
             '            return 0;', '      }']
        b = ['      switch (x) {', '         case 1:', '            return 1;', '         default:', '            return 0;',
             '      }']
        self.assertTrue(f(a, b, 3, 4, 3, 3, set()))
        self.assertFalse(f(a, b, 1, 2, 1, 1, set()))

    def test_synthetic_default(self):
        f = self.tool.switch_synthetic_default
        self.assertTrue(f(['         default:', '            throw new MatchException(null, null);'], []))
        self.assertTrue(f(['         default -> throw new MatchException(null, null);'], []))
        self.assertTrue(f(['         default:', '            throw new IncompatibleClassChangeError();'], []))
        self.assertFalse(f(['         default:', '            throw new IllegalStateException();'], []))

    def test_expression_return(self):
        f = self.tool.switch_expression_return_scope
        a = ['      switch (f) {', '         case FLAT:', '         case TREE:', '            return n * 2L;',
             '         case LEAF:', '            throw new IllegalStateException();', '         default:',
             '            throw new MatchException(null, null);', '      }', '   }']
        b = ['', '      return switch (f) {', '         case FLAT, TREE -> n * 2L;',
             '         case LEAF -> throw new IllegalStateException();', '      };', '   }']
        self.assertTrue(f(a, b, 0, 9, 0, 5, set()))
        wrong = [x.replace('n * 2L', 'n * 3L') for x in b]
        self.assertFalse(f(a, wrong, 0, 9, 0, 5, set()))

    def test_boolean_compare(self):
        f = self.tool.javac_boolean_compare
        self.assertTrue(f(['      return !this.items.isEmpty();'], ['      return false == this.items.isEmpty();']))
        self.assertTrue(f(['      return b != null && !b.isEmpty();'], ['      return (b == null || b.isEmpty()) == false;']))
        self.assertTrue(f(['      return a <= 3;'], ['      return a > 3 == false;']))
        self.assertTrue(f(['      if (!this.flag) {'], ['      if (this.flag != true) {']))
        self.assertTrue(f(['      if (!(o instanceof Map<?, ?> map)) {'], ['      if (false == (o instanceof Map)) {']))
        self.assertFalse(f(['      return !this.items.isEmpty();'], ['      return false == this.items.isFull();']))
        self.assertFalse(f(['      return !x;'], ['      return !x;']))

    def test_boolean_pattern(self):
        f = self.tool.javac_boolean_pattern_scope
        a = ['      if (!(o instanceof Map<?, ?> map)) {', '         return null;', '      }', '', '      return map;']
        b = ['      if (false == (o instanceof Map)) {', '         return null;', '      }', '',
             '      Map<?, ?> map = (Map<?, ?>)o;', '      return map;']
        self.assertTrue(f(a, b, 4, 4, 4, 5, set()))
        self.assertFalse(f(a, b, 4, 4, 5, 6, set()))
class TestInstanceofForms(unittest.TestCase):
    """pattern_binding_shape and jvm_source_patterns (a test and a cast for a pattern), jvm_source_locals (a kept
    pattern subject), and the four across a method (instanceof_forms)."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_instanceof_cast(self):
        f = self.tool.instanceof_cast
        self.assertTrue(f(['      if (chg instanceof ContextOp cop) {'],
                          ['      if (chg instanceof ContextOp) {', '         ContextOp cop = (ContextOp)chg;']))
        self.assertTrue(f(['      if (!(that instanceof Key mk)) {', '         return false;', '      }'],
                          ['      if (!(that instanceof Key)) {', '         return false;', '      }', '',
                           '      Key mk = (Key)that;']))
        self.assertTrue(f(['      } else if (o instanceof List<?> l) {'],
                          ['      } else if (o instanceof List) {', '         List<?> l = (List<?>)o;']))
        self.assertTrue(f(['      if (!(that instanceof Key mk)) {'], ['      if (!(that instanceof Key)) {']))
        self.assertFalse(f(['      if (chg instanceof ContextOp cop) {'],
                           ['      if (chg instanceof ContextOp) {', '         ContextOp cop = (ContextOp)other;']))
        self.assertFalse(f(['      if (chg instanceof ContextOp cop) {'], ['      if (chg instanceof ContextOp) {', '         use(chg);']))
        self.assertFalse(f(['      if (a instanceof B b) {'], ['      if (a instanceof C) {']))

    def test_pattern_subject_local(self):
        f = self.tool.pattern_subject_local
        self.assertTrue(f(['      if (this.ctx.getEnvironment() instanceof Env env) {'],
                          ['      Environment environment = this.ctx.getEnvironment();',
                           '      if (environment instanceof Env env) {']))
        self.assertTrue(f(['      return t.get() instanceof Validator v ? v : null;'],
                          ['      Object value = t.get();', '      return value instanceof Validator v ? v : null;']))
        self.assertTrue(f(['      if ((a != null ? a.src() : null) instanceof S s && s.ok()) {'],
                          ['      Iterable<S> src = a != null ? a.src() : null;', '      if (src instanceof S s && s.ok()) {']))
        self.assertTrue(f(['      if (b.has(n) && b.get(n) instanceof R r) {', '         return r.m();', '      }'],
                          ['      if (b.has(n)) {', '         Def d = b.get(n);', '         if (d instanceof R r) {',
                           '            return r.m();', '         }', '      }']))
        self.assertFalse(f(['      if (this.ctx.getEnvironment() instanceof Env env) {'],
                           ['      Environment environment = this.ctx.other();', '      if (environment instanceof Env env) {']))
        self.assertFalse(f(['      if (x.get() instanceof Env env) {'],
                           ['      Object v = x.get();', '      use(v);', '      if (v instanceof Env env) {']))

    def test_instanceof_cast_subject_local(self):
        f = self.tool.instanceof_cast_subject_local
        self.assertTrue(f(['         if (prod.get(i) instanceof NonTerminal nt) {'],
                          ['         Symbol px = prod.get(i);', '         if (px instanceof NonTerminal) {',
                           '            NonTerminal nt = (NonTerminal)px;']))
        self.assertFalse(f(['         if (prod.get(i) instanceof NonTerminal nt) {'],
                           ['         Symbol px = prod.get(j);', '         if (px instanceof NonTerminal) {',
                            '            NonTerminal nt = (NonTerminal)px;']))

    def test_cast_declaration_context(self):
        f = self.tool.instanceof_cast_declaration
        a = ['   boolean eq(Object that) {', '      if (!(that instanceof Key mk)) {', '         return false;', '      }', '',
             '      return mk.k == this.k;', '   }']
        b = ['   boolean eq(Object that) {', '      if (!(that instanceof Key)) {', '         return false;', '      }', '',
             '      Key mk = (Key)that;', '      return mk.k == this.k;', '   }']
        self.assertTrue(f(a, b, 5, 5, 5, 6, set()))
        other = list(b)
        other[5] = '      Key mk = (Key)this;'
        self.assertFalse(f(a, other, 5, 5, 5, 6, set()))

    def test_instanceof_forms(self):
        f = self.tool.instanceof_forms
        a = ['   public void set(Factory f) {', '      if (f instanceof Lbf lbf) {', '         this.f = lbf;', '      } else {',
             '         throw new IllegalStateException();', '      }', '   }']
        b = ['   public void set(Factory f) {', '      if (!(f instanceof Lbf lbf)) {', '         throw new IllegalStateException();',
             '      }', '', '      this.f = lbf;', '   }']
        self.assertTrue(f(a, b, 1, 4, 1, 2, set()))
        c = ['   Object get(Map m) {', '      if (m.get(k) instanceof Holder h) {', '         Object var5 = h.value;',
             '         put(var5);', '      }', '   }']
        d = ['   Object get(Map m) {', '      Object value = m.get(k);', '      if (value instanceof Holder h) {',
             '         value = h.value;', '         put(value);', '      }', '   }']
        self.assertTrue(f(c, d, 1, 4, 1, 5, set()))
        e = list(b)
        e[5] = '      this.g = lbf;'
        self.assertFalse(f(a, e, 1, 4, 1, 2, set()))
class TestQualifiedSuperCall(unittest.TestCase):
    """qualified_super_call: `super(..)` written `o.super(..)`, `o.new S(..) {..}` and javac's checks dropped."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    def test_qualified_super_call(self):
        f = self.tool.qualified_super_call
        self.assertTrue(f(['         super(2);'], ['         o.super(2);']))
        self.assertTrue(f(['         Objects.requireNonNull(o);', '         super(y * 2);'], ['         o.super(y * 2);']))
        self.assertTrue(f(['         super(y + 1);'], ['         new Outer(y).super(y + 1);']))
        self.assertTrue(f(['         Outer var10001 = os[i];', '         Objects.requireNonNull(os[i]);', '         super(i);'],
                          ['         os[i].super(i);']))
        self.assertTrue(f(['      Objects.requireNonNull(o);', '      return new Outer.Inner(o, n) {', '         {',
                           '            Objects.requireNonNull(x0);', '         }', ''],
                          ['      return o.new Inner(n) {']))
        self.assertFalse(f(['         super(2);'], ['         o.super(3);']))
        self.assertFalse(f(['         super(2);'], ['         super(2);']))
        self.assertFalse(f(['         Objects.requireNonNull(o);', '         super(2);'], ['         super(2);']))
        self.assertFalse(f(['      return new Outer.Inner(o, n) {'], ['      return p.new Inner(n) {']))

    def test_var_merge_declared_types(self):
        f = self.tool.var_merge_declared_types
        self.assertTrue(f(['      c = (char)i;', '      return c * 2;'], ['      int start = i;', '      return start * 2;']))
        self.assertTrue(f(['      yearStart = i.toChar()', '      val n: Int = i - yearStart.code'],
                          ['      val var36: Int = i', '      val n: Int = i - var36']))
        self.assertFalse(f(['      c = (char)i;'], ['      int start = i + 1;']))

    def test_qualified_super_call_external(self):
        f = self.tool.qualified_super_call
        self.assertTrue(f(['         super(HalfFloatComparator.this, context);'], ['         super(context);']))
        self.assertTrue(f(['         super(o, 3);'], ['         o.super(3);']))
        self.assertTrue(f(['         Objects.requireNonNull(o);', '         super(o, v * 2);'], ['         o.super(v * 2);']))
        self.assertFalse(f(['         super(o, 3);'], ['         super(o);']))
        self.assertFalse(f(['         super(a, 3);'], ['         b.super(3);']))

    def test_external_inherited_member_type(self):
        f = self.tool.external_inherited_member_type
        self.assertTrue(f(['   public static class B extends A.W implements Bucket {'],
                          ['   public static class B extends A.W implements pkglib.Filters.Bucket {']))
        self.assertTrue(f(['import java.io.Reader;', '      private LinesIterator(Reader reader) {'],
                          ['      private LinesIterator(java.io.Reader reader) {']))
        self.assertFalse(f(['      private LinesIterator(Reader reader) {'], ['      private LinesIterator(Writer reader) {']))
        self.assertFalse(f(['      return x.y;'], ['      return x.y;']))

    def test_lambda_return_type_argument(self):
        f = self.tool.lambda_return_type_argument
        self.assertTrue(f(['            .<List<TransportVersion>>map(VersionExtension::getTransportVersions)'],
                          ['            .map(VersionExtension::getTransportVersions)']))
        self.assertFalse(f(['            .<List<T>>map(f)'], ['            .map(g)']))
        self.assertFalse(f(['            .map(f)'], ['            .<List<T>>map(f)']))

    def test_type_annotation_outer_arguments(self):
        f = self.tool.type_annotation_outer_arguments
        self.assertTrue(f(['   private CombinedFuture.@Nullable Task<?> task;'], ['   private CombinedFuture<V>.@Nullable Task<?> task;']))
        self.assertTrue(f(['      LocalCache.@Nullable WriteThroughEntry next;'], ['      LocalCache<K, V>.@Nullable WriteThroughEntry next;']))
        self.assertFalse(f(['   private List<V> task;'], ['   private List<V> task;']))
        self.assertFalse(f(['   private A.@N B x;'], ['   private A<V>.@N C x;']))

    def test_ktvalid2_hunks(self):
        f = self.tool.kv2_hunk
        self.assertTrue(f(['         val var10000: ResponseBody = response.body',
                           '         if (var10000 == null) {', '            Intrinsics.throwNpe()', '         }', '',
                           '         val source: BufferedSource = var10000.source()'],
                          ['         val source: BufferedSource = response.body!!.source()']))
        self.assertTrue(f(['      if (this.stream == null) {', '         Intrinsics.throwNpe()', '      }', '',
                           '      this.stream.readTimeout()'], ['      this.stream!!.readTimeout()']))
        self.assertTrue(f(['      val count: Int = 0'], ['      var count: Int = 0']))
        self.assertTrue(f(['   public final var a: Int = 1'], ['   @get:Synchronized', '   public final var a: Int = 1']))
        self.assertTrue(f(['   @JvmStatic', '   fun {', '      x = 1', '   }'], ['   init {', '      x = 1', '   }']))
        self.assertTrue(f(['   private final val names: Array<String>'],
                          ['   private final val names: Array<String> = arrayOf("a", "b")']))
        self.assertTrue(f(['   public final val NAME: Map<A, B>', '   @JvmStatic', '   fun {',
                           '      val var0: Hpack = Hpack()', '      INSTANCE = var0',
                           '      NAME = var0.nameToFirstIndex()', '   }'],
                          ['   public final val NAME: Map<A, B> = Hpack.nameToFirstIndex()']))
        self.assertFalse(f(['      this.stream.readTimeout()'], ['      this.stream!!.writeTimeout()']))
        self.assertFalse(f(['      val count: Int = 0'], ['      var count: Int = 1']))
        self.assertFalse(f(['   @JvmStatic', '   fun {', '      x = 1', '   }'], ['   init {', '      x = 2', '   }']))
        self.assertFalse(f(['      public final var headerCount: Int'], ['      public final var headerCount: Int = 0']))
        self.assertFalse(f(['         val var9: Int = index + 1', '         this.table[var9] = entry'],
                           ['         this.table[index + 1] = entry']))



class TestJkmode(unittest.TestCase):
    """split_constructor_call and nested_param_rename's setter parameters, on Kotlin classes written as Java."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    SPLIT_OLD = ["   public final Entry build() {",
                 "      Entry var10000 = new Entry;",
                 "      if (this.name == null) {",
                 "         throw new IllegalStateException(\"name == null\");",
                 "      }",
                 "",
                 "      var10000./* $VF: Unable to resugar constructor */<init>(this.index, this.name);",
                 "      return var10000;",
                 "   }"]
    SPLIT_NEW = ["   public final Entry build() {",
                 "      if (this.name == null) {",
                 "         throw new IllegalStateException(\"name == null\");",
                 "      } else {",
                 "         return new Entry(this.index, this.name);",
                 "      }",
                 "   }"]

    def test_split_constructor_call(self):
        f = self.tool.split_constructor_call
        self.assertTrue(f(self.SPLIT_OLD, self.SPLIT_NEW))
        self.assertTrue(f(["      val var10000: Pair3 = Pair3", "      if (this.label == null) {", "         return null",
                           "      } else {", "         var10000./* $VF: Unable to resugar constructor */<init>(this.count, this.label)",
                           "         return var10000", "      }"],
                          ["      return if (this.label == null) null else Pair3(this.count, this.label)"]))
        self.assertFalse(f(self.SPLIT_OLD, [x.replace("this.index", "this.other") for x in self.SPLIT_NEW]))
        self.assertFalse(f(self.SPLIT_OLD, [x.replace("new Entry(", "Entry.of(") for x in self.SPLIT_NEW]))
        self.assertFalse(f(self.SPLIT_NEW, self.SPLIT_NEW))

    def test_split_constructor_call_scope(self):
        f = self.tool.split_constructor_call_scope
        a = ["class A {"] + self.SPLIT_OLD + ["}"]
        b = ["class A {"] + self.SPLIT_NEW + ["}"]
        self.assertTrue(f(a, b, 2, 3, 2, 2, set()))
        self.assertFalse(f(a, a, 2, 3, 2, 2, set()))

    def test_nested_param_invalid_name(self):
        f = self.tool.nested_param_invalid_name
        self.assertTrue(f(["   public final void setJob(@NotNull JobSupport <set-?>) {"],
                          ["   public final void setJob(@NotNull JobSupport var1) {"]))
        self.assertTrue(f(["      public void setIndex(int <set-?>) {"], ["      public void setIndex(int var1) {"]))
        self.assertFalse(f(["   public final void setJob(@NotNull JobSupport <set-?>) {"],
                           ["   public final void setJob(@NotNull JobSupport job) {"]))
        self.assertTrue(f(["   private static final int component1(int[] $v$c$kotlin-UIntArray$-$this$component1$0) {"],
                          ["   private static final int component1(int[] var0) {"]))
        self.assertTrue(f(["      int[] $v$c$kotlin-UIntArray$-$this$f$0, int index"], ["      int[] var0, int index"]))
        self.assertFalse(f(["      this.job = <set-?>;"], ["      this.job = var1;"]))
        self.assertFalse(f(["   void f(int count) {"], ["   void f(int var1) {"]))
        self.assertTrue(f(["   public final void setTags(@NotNull Map<Class<?>, Object> <set-?>) {"],
                          ["   public final void setTags(@NotNull Map<Class<?>, Object> var1) {"]))
        self.assertTrue(f(["   private static final int at_Q/* $VF was: at-Q */(",
                           "      int[] $v$c$kotlin-UIntArray$-$this$at$0, int index, Function1<? super Integer, UInt> d",
                           "   ) {"],
                          ["   private static final int at_Q/* $VF was: at-Q */(int[] var0, int index, Function1<? super Integer, UInt> d) {"]))
        self.assertFalse(f(["   private static final int at(", "      int[] $v$c$kotlin-UIntArray$-$this$at$0, int index", "   ) {"],
                           ["   private static final int at(int[] var0, int other) {"]))



class TestKtunrep(unittest.TestCase):
    """synchronized_break_exit, inline_marker_finally_copy, kt_unrepresentable_declared_type."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load("kuna_fix_diff", TOOL)

    SBE_OLD = ["   public fun close(code: Int): Boolean {",
               "      var var5: Boolean",
               "      run label53@{",
               "         synchronized (this) {",
               "            if (this.error != 0) {",
               "               var5 = false",
               "               return@label53",
               "            }",
               "",
               "            this.error = code",
               "         }",
               "",
               "         return true",
               "      }",
               "",
               "      // $VF: monitorexit",
               "      return var5",
               "   }"]
    SBE_NEW = ["   public fun close(code: Int): Boolean {",
               "      synchronized (this) {",
               "         if (this.error != 0) {",
               "            return false",
               "         }",
               "",
               "         this.error = code",
               "      }",
               "",
               "      return true",
               "   }"]

    def test_synchronized_break_exit(self):
        f = self.tool.synchronized_break_exit
        self.assertTrue(f(self.SBE_OLD, self.SBE_NEW))
        self.assertFalse(f(self.SBE_OLD, [x.replace("return false", "return true") for x in self.SBE_NEW]))
        self.assertFalse(f(self.SBE_OLD, [x.replace("this.error = code", "this.other = code") for x in self.SBE_NEW]))
        self.assertFalse(f(self.SBE_OLD, self.SBE_NEW[:1] + ["      // $VF: monitorexit"] + self.SBE_NEW[1:]))
        self.assertFalse(f(self.SBE_NEW, self.SBE_NEW))

    def test_synchronized_break_exit_scope(self):
        f = self.tool.synchronized_break_exit_scope
        a = ["class A {"] + self.SBE_OLD + ["}"]
        b = ["class A {"] + self.SBE_NEW + ["}"]
        self.assertTrue(f(a, b, 2, 4, 2, 2, set()))
        self.assertFalse(f(a, a, 2, 4, 2, 2, set()))
        note = ["   // $VF: Could not create synchronized statement, marking monitor enters and exits",
                "   // Please report this to the kuna issue tracker, at https://github.com/Noelo-Lab/kuna/issues"]
        self.assertTrue(f(["x"] + note + ["y"], ["x", "y"], 1, 3, 1, 1, set()))
        self.assertFalse(f(["x", "   int a = 1;", "y"], ["x", "y"], 1, 2, 1, 1, set()))

    IMF_OLD = ["   public inline fun <T> f(get: () -> T?): T? {",
               "      synchronized (this){} // $VF: monitorenter ",
               "      var var9: Boolean = false /* VF: Semaphore variable */",
               "",
               "      var var7: Any",
               "      run label51@{",
               "         var var11: Any",
               "         try {",
               "            var9 = true",
               "            val var10000: Any = get()",
               "            if (var10000 == null) {",
               "               var7 = null",
               "               var9 = false",
               "               return@label51",
               "            }",
               "",
               "            var11 = var10000",
               "            var9 = false",
               "         } finally {",
               "            if (var9) {",
               "               InlineMarker.finallyStart(1)",
               "               // $VF: monitorexit",
               "               InlineMarker.finallyEnd(1)",
               "            }",
               "         }",
               "",
               "         InlineMarker.finallyStart(1)",
               "         // $VF: monitorexit",
               "         InlineMarker.finallyEnd(1)",
               "         return var11 as T",
               "      }",
               "",
               "      InlineMarker.finallyStart(2)",
               "      // $VF: monitorexit",
               "      InlineMarker.finallyEnd(2)",
               "      return var7 as T",
               "   }"]
    IMF_NEW = ["   public inline fun <T> f(get: () -> T?): T? {",
               "      synchronized (this) {",
               "         val var10000: Any = get()",
               "         if (var10000 == null) {",
               "            return null",
               "         } else {",
               "            return var10000 as T",
               "         }",
               "      }",
               "   }"]

    def test_inline_marker_finally_copy(self):
        f = self.tool.inline_marker_finally_copy
        self.assertTrue(f(self.IMF_OLD, self.IMF_NEW))
        self.assertFalse(f(self.IMF_OLD, [x.replace("get()", "other()") for x in self.IMF_NEW]))
        self.assertFalse(f(self.IMF_OLD, [x.replace("return null", "return var10000.next") for x in self.IMF_NEW]))
        self.assertFalse(f(self.IMF_NEW, self.IMF_NEW))
        a = ["class A {"] + self.IMF_OLD + ["}"]
        b = ["class A {"] + self.IMF_NEW + ["}"]
        self.assertTrue(self.tool.inline_marker_finally_copy_scope(a, b, 2, 4, 2, 3, set()))

    def test_kt_unrepresentable_declared_type(self):
        f = self.tool.kt_unrepresentable_declared_type
        self.assertTrue(f(["      val var10003: <unrepresentable> = JobSupport::register"],
                          ["      val var10003 = JobSupport::register"]))
        self.assertTrue(f(["   private final val cache: <unrepresentable> = object : ClassValue<Any>() {", "x"],
                          ["   private final val cache = object : ClassValue<Any>() {", "x"]))
        self.assertFalse(f(["      val a: <unrepresentable> = b"], ["      val a = c"]))
        self.assertFalse(f(["      var collector: <unrepresentable>"], ["      var collector"]))


if __name__ == "__main__":
    unittest.main()
