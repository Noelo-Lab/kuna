import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktdefaults", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


class TestKtDefaults(unittest.TestCase):
    """The recognisers of kt_default_args_bridge_lookup, kt_default_value_names,
    kt_default_value_coercion_cast and kt_delegated_member_default."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def fix(self, fix, a, b):
        return all(self.tool.RECOGNISERS[fix](a[i1:i2], b[j1:j2]) for i1, i2, j1, j2 in hunks(a, b))

    def test_lookup(self):
        a = ["   public fun String.utf8Size(beginIndex: Int = ..., endIndex: Int = ...): Long {"]
        b = ["   public fun String.utf8Size(beginIndex: Int = 0, endIndex: Int = this.length): Long {"]
        self.assertTrue(self.fix("kt_default_args_bridge_lookup", a, b))
        self.assertFalse(self.fix("kt_default_value_names", a, b))

    def test_lookup_wrapped_and_generic(self):
        a = ["   separator: CharSequence = ...,", "   transform: ((T) -> CharSequence)? = ..."]
        b = ["   separator: CharSequence = \", \",", "   transform: ((T) -> CharSequence)? = null"]
        self.assertTrue(self.fix("kt_default_args_bridge_lookup", a, b))
        a = ["public suspend fun <T> Flow<T>.toList(destination: MutableList<T> = ...): List<T> {"]
        b = ["public suspend fun <T> Flow<T>.toList(destination: MutableList<T> = ArrayList()): List<T> {"]
        self.assertTrue(self.fix("kt_default_args_bridge_lookup", a, b))

    def test_lookup_lambda_spans_lines(self):
        a = ["public suspend fun ProducerScope<*>.awaitClose(block: () -> Unit = ...) {"]
        b = ["public suspend fun ProducerScope<*>.awaitClose(block: () -> Unit = { ", "      Unit", "   }) {"]
        self.assertTrue(self.fix("kt_default_args_bridge_lookup", a, b))

    def test_names(self):
        a = ["   fun ByteArray.binarySearch(element: Byte, toIndex: Int = `$this$binarySearch`.size): Int {"]
        b = ["   fun ByteArray.binarySearch(element: Byte, toIndex: Int = this.size): Int {"]
        self.assertTrue(self.fix("kt_default_value_names", a, b))
        a = ["   public fun String.pad(width: Int = this.length): String {"]
        b = ["   public fun String.pad(width: Int = this@Box.length): String {"]
        self.assertTrue(self.fix("kt_default_value_names", a, b))
        a = ["   public abstract fun area(scale: Int = 2, base: Int = var1 + 1): Int"]
        b = ["   public abstract fun area(scale: Int = 2, base: Int = scale + 1): Int"]
        self.assertTrue(self.fix("kt_default_value_names", a, b))

    def test_cast(self):
        a = ["      public fun String.toRequestBody(contentType: MediaType? = null as MediaType?): RequestBody {"]
        b = ["      public fun String.toRequestBody(contentType: MediaType? = null): RequestBody {"]
        self.assertTrue(self.fix("kt_default_value_coercion_cast", a, b))
        a = ["   prefix: CharSequence = \"\" as CharSequence,"]
        b = ["   prefix: CharSequence = \"\","]
        self.assertTrue(self.fix("kt_default_value_coercion_cast", a, b))

    def test_composed(self):
        a = ["public inline fun UByte.toHexString(format: HexFormat = ...): String {",
             "   return HexExtensionsKt.toHexString(var0, format)"]
        b = ["public inline fun UByte.toHexString(format: HexFormat = HexFormat.Companion.Default): String {",
             "   return HexExtensionsKt.toHexString(this, format)"]
        self.assertTrue(self.tool.RECOGNISERS["kt_default_args_bridge_lookup"](a, b))
        b = ["public inline fun UByte.toHexString(format: HexFormat = HexFormat.Companion.Default): String {",
             "   return HexExtensionsKt.toHexString(this, format)", "   foo()"]
        self.assertFalse(self.tool.RECOGNISERS["kt_default_args_bridge_lookup"](a, b))

    def test_joined_hunk(self):
        """A header whose default changed no longer anchors the line matching, so the generated overloads
        removed before it share its hunk; with the default put back, the file's hunks are the others' fixes'."""
        overload = ["   @JvmOverloads", "   @JvmName(name = \"create\")", "   fun ByteArray.create(): RequestBody {",
                    "      create$default(this, `$this$x`, null, 7, null)", "   }", ""]
        a = ["class C {"] + overload + ["   @JvmName(name = \"create\")",
                                        "   public fun File.asRequestBody(contentType: MediaType? = ...): RequestBody {",
                                        "      foo()", "   }", "}"]
        b = ["class C {", "   @JvmName(name = \"create\")",
             "   public fun File.asRequestBody(contentType: MediaType? = null): RequestBody {", "      foo()", "   }", "}"]
        joined = [h for h in hunks(a, b) if h[1] - h[0] > 1]
        self.assertTrue(joined and all(self.tool.ktdefaults_composed(a, b, *h, set()) for h in joined))
        b.insert(4, "      if (x) return 1")
        self.assertFalse(self.tool.ktdefaults_composed(a, b, *hunks(a, b)[-1], set()))
        a = [x.replace("= ...)", "= 2)") for x in a]
        b = [x.replace("= null)", "= 3)") for x in b]
        self.assertFalse(any(self.tool.ktdefaults_composed(a, b, *h, set()) for h in hunks(a, b)))

    def test_delegated(self):
        a = ["   public override fun cancel(cause: Throwable? = ...): Boolean {"]
        b = ["   public override fun cancel(cause: Throwable?): Boolean {"]
        self.assertTrue(self.fix("kt_delegated_member_default", a, b))
        self.assertFalse(self.fix("kt_default_args_bridge_lookup", a, b))

    def test_rejects_other_changes(self):
        for a, b in [
            (["   public fun f(x: Int = 1): Int {"], ["   public fun f(x: Int = 2): Int {"]),
            (["   public fun f(x: Int = ...): Int {"], ["   public fun f(x: Int = 2): Int {", "      foo()"]),
            (["      val x: Foo = bar as Foo"], ["      val x: Foo = bar"]),
            (["      f(a = 1)"], ["      f(a = 2)"]),
            (["   public fun f(x: Int = this.n): Int {"], ["   public fun f(x: Int = that.n): Int {"]),
        ]:
            for fix in ("kt_default_args_bridge_lookup", "kt_default_value_names",
                        "kt_default_value_coercion_cast", "kt_delegated_member_default"):
                self.assertFalse(self.fix(fix, a, b), (fix, a, b))


if __name__ == "__main__":
    unittest.main()
