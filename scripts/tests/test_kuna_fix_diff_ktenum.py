import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktenum", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestEnumConstantInValuesArray(unittest.TestCase):
    """The recogniser of enum_constant_in_values_array: bare enum constants gain their arguments."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["enum_constant_in_values_array"])

    def test_constants_gain_arguments(self):
        old = ["   NO_ERROR,", "   PROTOCOL_ERROR,", "   HTTP_1_1_REQUIRED;"]
        new = ["   NO_ERROR(0),", "   PROTOCOL_ERROR(1),", "   HTTP_1_1_REQUIRED(13);"]
        self.assertTrue(self.f(old, new))
        self.assertTrue(self.f(["   TLS_1_3,"], ['   TLS_1_3("TLSv1.3"),']))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   A,", "   B;"], ["   A(1),", "   C(2);"]))
        self.assertFalse(self.f(["   A,"], ["   A(1);"]))
        self.assertFalse(self.f(["   A(1),"], ["   A(2),"]))
        self.assertFalse(self.f(["   A,"], ["   A,"]))
        self.assertFalse(self.f(["   val x: Int"], ["   val x: Int(1)"]))
        self.assertFalse(self.f(["   A,", "   B;"], ["   A(1),"]))


class TestKtExtensionProperty(unittest.TestCase):
    """The recogniser of kt_extension_property: the receiver in the header, `this` in the accessors."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["kt_extension_property"])

    def test_header_and_body(self):
        old = ["public val secondOrNull: T?", "   public get() {",
               "      return (if (`$this$secondOrNull`.size > 1) `$this$secondOrNull`.get(1) else null) as T"]
        new = ["public val <T> List<T>.secondOrNull: T?", "   public get() {",
               "      return (if (this.size > 1) this.get(1) else null) as T"]
        self.assertTrue(self.f(old, new))

    def test_member_extension(self):
        old = ["   private final val scaled: Int", "      private get() {", "         return `$this$scaled` * this.scale"]
        new = ["   private final val Int.scaled: Int", "      private get() {",
               "         return this * this@TestMemberExtensionProperty.scale"]
        self.assertTrue(self.f(old, new))

    def test_reflowed_body(self):
        old = ["      return if (`$this$x`.a() > 1)", "         5", "         else", "         3", "      }"]
        new = ["      return if (this.a() > 1) 5 else 3", "   }"]
        self.assertTrue(self.f(old, new))

    def test_string_template(self):
        old = ['      throw IllegalStateException(("no Job in it: $`$this$job`").toString())']
        new = ['      throw IllegalStateException(("no Job in it: $this").toString())']
        self.assertTrue(self.f(old, new))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   return `$this$x`.a()"], ["   return this.b()"]))
        self.assertFalse(self.f(['   f("\\$this\\$x")'], ['   f("this")']))
        self.assertFalse(self.f(["public val x: Int"], ["public val x: Long"]))
        self.assertFalse(self.f(["   return this.a()"], ["   return this@C.a()"]))
        self.assertFalse(self.f(["   return y.a()"], ["   return this.a()"]))


class TestKtInterfaceCompanionField(unittest.TestCase):
    """The recogniser of kt_interface_companion_field: the Java-form field is gone."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["kt_interface_companion_field"])

    def test_field_removed(self):
        old = ["   @JvmStatic", "   Job.Key Key = Job.Key.`$$INSTANCE`;", ""]
        self.assertTrue(self.f(old, []))
        self.assertTrue(self.f(["   Interceptor.Companion Companion = Interceptor.Companion.`$$INSTANCE`;"], [""]))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   @JvmStatic", "   Job.Key Key = Job.Key.other;"], []))
        self.assertFalse(self.f(["   val x: Int = 1"], []))
        self.assertFalse(self.f(["   Job.Key Key = Job.Key.`$$INSTANCE`;"], ["   val Key: Job.Key"]))


class TestKtGeneratedOverloadHidden(unittest.TestCase):
    """The recogniser of kt_generated_overload_hidden: generated `@JvmOverloads` overloads are gone."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["kt_generated_overload_hidden"])

    def test_overloads_removed(self):
        old = ["   @JvmOverloads", "   @Throws(IOException::class)", "   fun copyTo(out: OutputStream): Buffer {",
               "      return `copyTo$default`(this, out, 0L, 0L, 6, null)", "   }", "",
               "   @JvmOverloads", "   fun copyTo(out: OutputStream, offset: Long): Buffer {",
               "      return `copyTo$default`(this, out, offset, 0L, 4, null)", "   }", ""]
        self.assertTrue(self.f(old, []))
        self.assertTrue(self.f(["   @JvmOverloads", "   fun <T> pick(first: T, second: T): T {",
                                "      return `pick$default`(this, first, second, false, 4, null) as T", "   }"], []))
        self.assertTrue(self.f(["@JvmOverloads", "fun File.sink(): Sink {",
                                "   return Okio.`sink$default`(`$this$sink`, false, 1, null)", "}", ""], [""]))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   fun f(): Int {", "      return `f$default`(this, 0, 1, null)", "   }"], []))
        self.assertFalse(self.f(["   @JvmOverloads", "   fun f(): Int {", "      return `g$default`(this, 0, 1, null)",
                                 "   }"], []))
        self.assertFalse(self.f(["   @JvmOverloads", "   fun f(): Int {", "      return 1", "   }"], []))
        self.assertFalse(self.f(["   @JvmOverloads", "   fun f(): Int {", "      return `f$default`(this, 0, 1, null)",
                                 "   }"], ["   fun f(): Int"]))


class TestKtGeneratedOverloadHiddenScope(unittest.TestCase):
    """The context recogniser of kt_generated_overload_hidden: a removal cut across overload blocks."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().CONTEXT_RECOGNISERS["kt_generated_overload_hidden"])

    def test_shifted_removal(self):
        a = ["   fun f(a: Int, b: Int = 0): Int {", "      return a + b", "   }", "",
             "   @JvmOverloads", "   fun f(a: Int): Int {", "      return `f$default`(this, a, 0, 2, null)", "   }", "",
             "   fun g() {", "   }"]
        b = a[:4] + a[9:]
        self.assertTrue(self.f(a, b, 3, 8, 4, 4, set()))
        self.assertFalse(self.f(a, b, 0, 3, 0, 0, set()))
        c = a[:4] + ["   fun h() {"] + a[9:]
        self.assertFalse(self.f(a, c, 3, 8, 4, 5, set()))

    def test_removal_ending_on_the_next_annotation(self):
        a = ["   @JvmOverloads", "   fun f(a: Int): Int {", "      return `f$default`(this, a, 0, 2, null)", "   }", "",
             "   @JvmOverloads", "   fun f(a: Int, b: Int = 0): Int {", "      return a + b", "   }"]
        b = a[:1] + a[6:]
        self.assertTrue(self.f(a, b, 1, 6, 1, 1, set()))
        c = ["   @JvmOverloads", "   fun f(a: Int): Int {", "      return 1", "   }", "",
             "   @JvmOverloads", "   fun f(a: Int, b: Int = 0): Int {", "      return a + b", "   }"]
        self.assertFalse(self.f(c, c[:1] + c[6:], 1, 6, 1, 1, set()))

    def test_composed_with_another_fix(self):
        a = ["   fun basic(u: String, p: String = \"x\"): String {",
             "      return \"Basic ${ByteString.Companion.encodeString(u, charset).base64()}\"", "   }", "",
             "   @JvmOverloads", "   fun basic(u: java.lang.String): java.lang.String {",
             "      basic$default(u, null, 2, null)", "   }", "}"]
        b = [a[0], "      return \"Basic ${u.encode(charset).base64()}\"", "   }", "}"]
        self.assertTrue(self.f(a, b, 1, 7, 1, 2, set()))
        b2 = [a[0], "      return \"Other ${u.encode(charset).base64()}\"", "   }", "}"]
        self.assertFalse(self.f(a, b2, 1, 7, 1, 2, set()))


class TestKtExtensionCallTypeArgument(unittest.TestCase):
    """The recogniser of kt_extension_call's explicit type argument for a boxed primitive receiver."""

    @classmethod
    def setUpClass(cls):
        cls.f = staticmethod(load().RECOGNISERS["kt_extension_call_type_argument"])

    def test_type_argument_added(self):
        old = ["      return this.coerceIn(range as ClosedFloatingPointRange<Int>).toInt()"]
        new = ["      return this.coerceIn<Int>(range as ClosedFloatingPointRange<Int>).toInt()"]
        self.assertTrue(self.f(old, new))

    def test_other_changes_are_not_taken(self):
        self.assertFalse(self.f(["   this.f(x)"], ["   this.g<Int>(x)"]))
        self.assertFalse(self.f(["   this.f(x)"], ["   this.f(x)"]))
        self.assertFalse(self.f(["   this.f(x)"], ["   this.f<Int>(y)"]))


if __name__ == "__main__":
    unittest.main()
