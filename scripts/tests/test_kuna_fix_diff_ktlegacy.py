import difflib
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktlegacy", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hunks(a, b):
    return [h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if h[0] != "equal"]


CHECK = '      throw TypeCastException("null cannot be cast to non-null type kotlin.Array<T>")'


class TestKtLegacy(unittest.TestCase):
    """The recognisers of kt_legacy_parameter_checks, kt_inlined_to_typed_array and kt_narrow_constant_local."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def rec(self, fix, old, new):
        return self.tool.RECOGNISERS[fix](old, new)

    def test_parameter_checks(self):
        old = ['   public fun f(a: String, b: List<String>): Int {',
               '      Intrinsics.checkParameterIsNotNull(a, "a")',
               '      Intrinsics.checkParameterIsNotNull(b, "b")',
               '      return a.length + b.size']
        new = [old[0], old[3]]
        self.assertTrue(self.rec('kt_legacy_parameter_checks', old[1:3], []))
        self.assertTrue(self.rec('kt_legacy_parameter_checks', old, new))
        self.assertTrue(self.rec('kt_legacy_parameter_checks', ['   Intrinsics.checkParameterIsNotNull(this, "\\$this$f")'], []))
        self.assertFalse(self.rec('kt_legacy_parameter_checks', old, [old[0], '      return a.length']))
        self.assertFalse(self.rec('kt_legacy_parameter_checks', ['      Intrinsics.checkNotNull(a)'], []))

    def test_parameter_checks_with_another_fix(self):
        a = ['class C {', '   public fun f(s: String): Int {', '      Intrinsics.checkParameterIsNotNull(s, "s")',
             '      return s.length()', '   }', '}']
        b = ['class C {', '   public fun f(s: String): Int {', '      return s.length', '   }', '}']
        fix = self.tool.CONTEXT_RECOGNISERS['kt_legacy_parameter_checks']
        self.assertTrue(all(fix(a, b, *h, set()) for h in hunks(a, b)))
        c = ['class C {', '   public fun f(s: String): Int {', '      return 42 + other(s)', '   }', '}']
        self.assertFalse(any(fix(a, c, *h, set()) for h in hunks(a, c)))

    def test_to_typed_array(self):
        old = ['      val var10000: Array<Any> = this.names.toArray(arrayOfNulls(0))',
               '      if (var10000 == null) {', CHECK, '      } else {',
               '         return Headers(var10000 as Array<String>)', '      }']
        self.assertTrue(self.rec('kt_inlined_to_typed_array', old, ['      return Headers(this.names.toTypedArray())']))
        self.assertTrue(self.rec('kt_inlined_to_typed_array', ['   f(c.toArray(arrayOfNulls(0)))'],
                                 ['   f(c.toTypedArray<Any>())']))
        self.assertTrue(self.rec('kt_inlined_to_typed_array',
                                 ['      val names: List<String> = alpn(p)',
                                  '      val var11: Array<Any> = names.toArray(arrayOfNulls(0))',
                                  '      x.set(var11 as Array<String>)'],
                                 ['      x.set(alpn(p).toTypedArray())']))
        self.assertFalse(self.rec('kt_inlined_to_typed_array', old, ['      return Other(this.names.toTypedArray())']))
        self.assertFalse(self.rec('kt_inlined_to_typed_array', ['   f(c.toArray(arrayOfNulls(1)))'],
                                  ['   f(c.toTypedArray())']))

    def test_to_typed_array_file(self):
        a = ['class C {', '   fun f(c: List<String>) {',
             '      val var10000: Array<Any> = c.toArray(arrayOfNulls(0))',
             '      if (var10000 == null) {', CHECK, '      } else {',
             '         g(var10000 as Array<String>)', '         run label7@{', '            return@label7', '         }',
             '      }', '   }', '}']
        b = ['class C {', '   fun f(c: List<String>) {', '      g(c.toTypedArray())', '      run label9@{',
             '         return@label9', '      }', '   }', '}']
        fix = self.tool.CONTEXT_RECOGNISERS['ktlegacy_file']
        self.assertTrue(all(fix(a, b, *h, set()) for h in hunks(a, b)))
        b2 = b[:2] + ['      h(c.toTypedArray())'] + b[3:]
        self.assertFalse(any(fix(a, b2, *h, set()) for h in hunks(a, b2)))

    def test_narrow_constant_local(self):
        self.assertTrue(self.rec('kt_narrow_constant_local', ['      var var10000: Byte'], ['      var var10000: Int']))
        self.assertTrue(self.rec('kt_narrow_constant_local', ['      val v: Short = 256', '      return v.toInt()'],
                                 ['      val v: Int = 256', '      return v']))
        self.assertFalse(self.rec('kt_narrow_constant_local', ['      val v: Byte = x'], ['      val v: Int = y']))
        self.assertFalse(self.rec('kt_narrow_constant_local', ['      f(v: Byte)'], ['      f(v: Long)']))
        a = ['   fun f(): Int {', '      val v: Byte', '      v = 2', '      return g(v.toInt(), 1)', '   }']
        b = ['   fun f(): Int {', '      val v: Int', '      v = 2', '      return g(v, 1)', '   }']
        fix = self.tool.CONTEXT_RECOGNISERS['kt_narrow_constant_local']
        self.assertTrue(fix(a, b, 3, 4, 3, 4, set()))
        self.assertFalse(fix(a, a[:3] + ['      return g(w, 1)'] + a[4:], 3, 4, 3, 4, set()))


if __name__ == "__main__":
    unittest.main()
