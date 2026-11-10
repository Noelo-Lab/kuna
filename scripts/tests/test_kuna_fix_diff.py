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


if __name__ == "__main__":
    unittest.main()
