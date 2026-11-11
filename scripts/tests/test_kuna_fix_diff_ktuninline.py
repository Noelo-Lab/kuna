import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktuninline", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


USE_OLD = """   public fun string(): String {
      val var1: Closeable = this.source()
      var var3: Throwable? = null as Throwable?

      try {
         return (var1 as BufferedSource).readString((var1 as BufferedSource).readBomAsCharset(this.charset()))
      } catch (var8: Throwable) {
         var3 = var8
         throw var8
      } finally {
         var1.closeFinally(var3)
      }
   }""".split("\n")
USE_NEW = """   public fun string(): String {
      return this.source().use { source ->
         source.readString(source.readBomAsCharset(this.charset()))
      }
   }""".split("\n")


class TestInlinedStdlibCall(unittest.TestCase):
    """kt_inlined_stdlib_call's recognisers: copies that became calls, and changes that are not."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def plain(self, old, new):
        return self.tool.RECOGNISERS["kt_inlined_stdlib_call"](old, new)

    def test_radix_and_format(self):
        old = ['      val var10000: StringBuilder = StringBuilder().append("was 0x")',
               '      val var23: String = Integer.toString(`b$iv`.toInt(), checkRadix(checkRadix(16)))',
               '      Intrinsics.checkExpressionValueIsNotNull(var23, "java.lang.Integer.toStri…(this, checkRadix(radix))")',
               '      throw NumberFormatException(var10000.append(var23).toString())']
        new = ['      throw NumberFormatException("was 0x${`b$iv`.toString(16)}")']
        self.assertTrue(self.plain(old, new))
        old = ['      val var19: StringCompanionObject = StringCompanionObject',
               '      val var22: Array<Any> = arrayOf(offset, buffer.size)',
               '      val var28: String = java.lang.String.format("offset=%s > size=%s", Arrays.copyOf(var22, var22.size))',
               '      Intrinsics.checkExpressionValueIsNotNull(var28, "java.lang.String.format(format, *args)")',
               '      throw ArrayIndexOutOfBoundsException(var28)']
        new = ['      throw ArrayIndexOutOfBoundsException(String.format("offset=%s > size=%s", offset, buffer.size))']
        self.assertTrue(self.plain(old, new))
        self.assertTrue(self.plain(['      return ByteString(this.data.copyOfRangeImpl(0, 64))'],
                                   ['      return ByteString(this.data.copyOfRange(0, 64))']))

    def test_use_split_into_hunks(self):
        f = self.tool.CONTEXT_RECOGNISERS["kt_inlined_stdlib_call"]
        hs = hunks(USE_OLD, USE_NEW)
        self.assertTrue(len(hs) > 0)
        self.assertTrue(all(f(USE_OLD, USE_NEW, *h, set()) for h in hs))

    def test_map(self):
        old = ['      val `$this$map$iv`: Iterable<RealCall.AsyncCall> = this.readyAsyncCalls',
               '      val `destination$iv$iv`: MutableCollection<RealCall> = ArrayList(`$this$map$iv`.collectionSizeOrDefault(10))',
               '',
               '      for (`item$iv$iv` in `$this$map$iv`) {',
               '         `destination$iv$iv`.add((`item$iv$iv` as RealCall.AsyncCall).call)',
               '      }',
               '',
               '      return `destination$iv$iv` as List<RealCall>']
        new = ['      return this.readyAsyncCalls.map { it.call }']
        f = self.tool.CONTEXT_RECOGNISERS["kt_inlined_stdlib_call"]
        self.assertTrue(all(f(old, new, *h, set()) for h in hunks(old, new)))

    def test_other_changes_are_not_claimed(self):
        self.assertFalse(self.plain(['      return a.copyOfRangeImpl(0, n)'], ['      return b.copyOfRange(0, n)']))
        self.assertFalse(self.plain(['      x.closeFinally(e)'], ['      x.close()']))
        old = list(USE_OLD)
        new = [l.replace("this.charset()", "Charsets.UTF_8") for l in USE_NEW]
        f = self.tool.CONTEXT_RECOGNISERS["kt_inlined_stdlib_call"]
        self.assertFalse(all(f(old, new, *h, set()) for h in hunks(old, new)))


if __name__ == "__main__":
    unittest.main()
