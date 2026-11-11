import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_kttypes2", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestKtTypes2(unittest.TestCase):
    """The recognisers of kt_class_header_metadata, kt_enclosing_type_parameter and kt_default_impls_delegate."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, fix, old, new):
        return self.tool.RECOGNISERS[fix](old, new)

    def test_class_headers(self):
        fix = "kt_class_header_metadata"
        self.assertTrue(self.accepts(fix, ["public interface Flow<T> {"], ["public interface Flow<out T> {"]))
        self.assertTrue(self.accepts(fix, ["private object EmptyFlow : Flow {"], ["private object EmptyFlow : Flow<Nothing> {"]))
        self.assertTrue(self.accepts(
            fix,
            ["public class Headers private constructor(namesAndValues: Array<String>) : java.lang.Iterable<Pair<out String, out String>>, KMappedMarker {"],
            ["public class Headers private constructor(namesAndValues: Array<String>) : Iterable<Pair<String, String>> {"]))
        self.assertTrue(self.accepts(fix, ["   ReaderRunnable(r: R) : Http2Reader.Handler, Function0<Unit> {"],
                                     ["   ReaderRunnable(r: R) : Http2Reader.Handler, () -> Unit {"]) is False)
        self.assertTrue(self.accepts(
            fix,
            ["   public inner class ReaderRunnable internal constructor(reader: Http2Reader) : Http2Reader.Handler, Function0<Unit> {"],
            ["   public inner class ReaderRunnable internal constructor(reader: Http2Reader) : Http2Reader.Handler, () -> Unit {"]))
        self.assertTrue(self.accepts(fix, ["public fun interface FlowCollector<T> {"], ["public fun interface FlowCollector<in T> {"]))
        self.assertTrue(self.accepts(fix, ["   BiFunction<T, Throwable, Unit> {"], ["   BiFunction<T?, Throwable?, Unit> {"]))
        self.assertTrue(self.accepts(fix, ["   java.util.Map<K, V>,", "   Serializable,", "   KMutableMap {"],
                                     ["   MutableMap<K, V>,", "   Serializable {"]))
        self.assertFalse(self.accepts(fix, ["public interface Flow<T> {"], ["public interface Source<out T> {"]))
        self.assertFalse(self.accepts(fix, ["class A(x: Int) : B(x) {"], ["class A(x: Int) : B(y) {"]))
        self.assertFalse(self.accepts(fix, ["   foo(a, b) {"], ["   foo(a, c) {"]))

    def test_collection_stubs_and_qualified_names(self):
        fix = "kt_class_header_metadata"
        stub = ["", "   override fun remove() {",
                '      throw UnsupportedOperationException("Operation is not supported for read-only collection")', "   }"]
        self.assertTrue(self.accepts(fix, stub, []))
        self.assertTrue(self.accepts(fix, ["   }", ""] + stub[1:3], []))
        self.assertFalse(self.accepts(fix, ["   override fun remove() {", "      throw IllegalStateException()", "   }"], []))
        self.assertTrue(self.accepts(fix, ["   public final val factory: Factory"],
                                     ["   public final val factory: okhttp3.EventListener.Factory"]))
        self.assertFalse(self.accepts(fix, ["   val x: Int = a.b"], ["   val x: Int = b"]))

    def test_enclosing_type_parameters(self):
        fix = "kt_enclosing_type_parameter"
        self.assertTrue(self.accepts(fix, ["   return object : Flow<Any> {", "      override suspend fun collect(collector: FlowCollector<Any>) {"],
                                     ["   return object : Flow<T> {", "      override suspend fun collect(collector: FlowCollector<T>) {"]))
        self.assertTrue(self.accepts(fix, ["         val c: FlowCollector = collector"], ["         val c: FlowCollector<in T> = collector"]))
        self.assertTrue(self.accepts(fix, ["      c = x.`L$1` as FlowCollector<*>"], ["      c = x.`L$1` as FlowCollector<in T>"]))
        self.assertTrue(self.accepts(fix, ["      val i: Iterator<*> = s.iterator()"], ["      val i: Iterator<T> = s.iterator()"]))
        self.assertFalse(self.accepts(fix, ["   return object : Flow<Any> {"], ["   return object : Flow<String> {"]))
        self.assertFalse(self.accepts(fix, ["   val a: Int = 1"], ["   val a: Int = 2"]))

    def test_default_impls_delegates(self):
        fix = "kt_default_impls_delegate"
        delegate = ["", "   override fun plus(other: Job): Job {", "      return Job.DefaultImpls.plus(this, other)", "   }"]
        self.assertTrue(self.accepts(fix, delegate, []))
        self.assertTrue(self.accepts(fix, ["   /** @deprecated */", "   @Deprecated(message = \"x\")"] + delegate[1:], []))
        static = ["   // $VF: Class flags could not be determined", "   internal class DefaultImpls {", "      @JvmStatic",
                  "      fun <R> fold(`$this`: Job, initial: R, operation: (R?, Element?) -> R): R {",
                  "         return kotlin.coroutines.CoroutineContext.Element.DefaultImpls.fold(`$this`, initial as R, operation)",
                  "      }", "   }"]
        self.assertTrue(self.accepts(fix, static, []))
        self.assertTrue(self.accepts(fix, static[1:], ["   internal class DefaultImpls"]))
        self.assertFalse(self.accepts(fix, ["   override fun plus(other: Job): Job {", "      return other", "   }"], []))
        self.assertFalse(self.accepts(fix, delegate, ["   override fun plus(other: Job): Job {"]))


if __name__ == "__main__":
    unittest.main()
