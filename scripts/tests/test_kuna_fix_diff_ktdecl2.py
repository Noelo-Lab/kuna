import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktdecl2", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestKtDecl2(unittest.TestCase):
    """The recognisers of kt_escaped_class_name, kt_inner_class_constructor, kt_data_class_property_parameters,
    kt_function_parameter_name, kt_contract_import and kt_unlabeled_sequence_break."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, fix, old, new):
        return self.tool.RECOGNISERS[fix](old, new)

    def test_escaped_class_name(self):
        fix = "kt_escaped_class_name"
        self.assertTrue(self.accepts(fix, ["public val Okio: -DeprecatedOkio = -DeprecatedOkio"],
                                     ["public val Okio: `-DeprecatedOkio` = `-DeprecatedOkio`"]))
        self.assertTrue(self.accepts(fix, ["      return Outer.in(v)"], ["      return Outer.`in`(v)"]))
        self.assertFalse(self.accepts(fix, ["   val x = in"], ["   val x = `in`"]))
        self.assertFalse(self.accepts(fix, ["   val a: Foo = f"], ["   val a: `Foo` = g"]))

    def test_inner_class_constructor(self):
        fix = "kt_inner_class_constructor"
        self.assertTrue(self.accepts(fix, ["      return Cache.RealCacheRequest(editor)"],
                                     ["      return RealCacheRequest(editor)"]))
        self.assertTrue(self.accepts(fix, ["   x(A.B.Inner(c.openInner(r), n).buffer())"],
                                     ["   x(Inner(c.openInner(r), n).buffer())"]))
        self.assertFalse(self.accepts(fix, ["      return Cache.RealCacheRequest(editor)"],
                                      ["      return RealCacheRequest(other)"]))
        self.assertFalse(self.accepts(fix, ["   f(Inner(a))", "   g(Outer.Inner(b))"], ["   f(Inner(a))", "   g(Inner(b))"]))

    def test_function_parameter_name(self):
        fix = "kt_function_parameter_name"
        self.assertTrue(self.accepts(fix, ["      return var1 is Header && this.name == (var1 as Header).name"],
                                     ["      return other is Header && this.name == (other as Header).name"]))
        self.assertTrue(self.accepts(fix, ['      return "$var2$var1"'], ['      return "$prefix$name"']))
        self.assertFalse(self.accepts(fix, ["   return var1 + var2"], ["   return x + x"]))
        self.assertFalse(self.accepts(fix, ['      return "$var1"'], ['      return "$this"']))
        self.assertFalse(self.accepts(fix, ["   return a + b"], ["   return c + b"]))

    def test_contract_import(self):
        fix = "kt_contract_import"
        old = ["   public fun <R> run(block: () -> R): R {", "      contract {"]
        self.assertTrue(self.accepts(fix, old, ["   @OptIn(ExperimentalContracts::class)",
                                                "   public fun <R> run(block: () -> R): R {",
                                                "      kotlin.contracts.contract {"]))
        self.assertTrue(self.accepts(fix, [], ["   @OptIn(ExperimentalContracts::class)"]))
        self.assertFalse(self.accepts(fix, old, ["   @OptIn(ExperimentalContracts::class)",
                                                 "   public fun <R> run(block: () -> R): Any {", "      contract {"]))

    def test_unlabeled_sequence_break(self):
        fix = "kt_unlabeled_sequence_break"
        self.assertTrue(self.accepts(fix, ["         delegate = x", "         return@run"],
                                     ["         delegate = x", "         break"]))
        self.assertFalse(self.accepts(fix, ["         return@run"], ["         continue"]))
        self.assertFalse(self.accepts(fix, ["         break"], ["         break"]))

    def test_data_class_property_parameters(self):
        ctx = self.tool.CONTEXT_RECOGNISERS["kt_data_class_property_parameters"]
        a = ["public data class Header(name: String, value: Int) {",
             "   public final val name: String",
             "   public final var value: Int",
             "",
             "   init {",
             "      this.name = name",
             "      this.value = value",
             "   }",
             "",
             "   public operator fun component1(): String {",
             "      return this.name",
             "   }",
             "",
             "   public fun twice(): Int {",
             "      return this.value * 2",
             "   }",
             "}"]
        b = ["public data class Header(public final val name: String, public final var value: Int) {",
             "   public fun twice(): Int {",
             "      return this.value * 2",
             "   }",
             "}"]
        self.assertTrue(ctx(a, b, 0, 13, 0, 1, set()))
        self.assertFalse(ctx(a, b, 13, 15, 1, 3, set()))
        wrong = ["public data class Header(public final val title: String, public final var value: Int) {"] + b[1:]
        self.assertFalse(ctx(a, wrong, 0, 13, 0, 1, set()))


if __name__ == "__main__":
    unittest.main()
