import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "tests/jvm/kuna_fix_diff.py"


def load():
    spec = importlib.util.spec_from_file_location("kuna_fix_diff_ktgenerics", TOOL)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestKtGenerics(unittest.TestCase):
    """The recognisers of kt_implicit_widening, kt_object_monitor_call and kt_raw_generic_type."""

    @classmethod
    def setUpClass(cls):
        cls.tool = load()

    def accepts(self, fix, old, new):
        return self.tool.RECOGNISERS[fix](old, new)

    def test_widening_conversions(self):
        fix = "kt_implicit_widening"
        self.assertTrue(self.accepts(fix, ["      val b0: Int = this[i++]"], ["      val b0: Int = this[i++].toInt()"]))
        self.assertTrue(self.accepts(fix, ["      return c"], ["      return c.code.toDouble()"]))
        self.assertTrue(self.accepts(fix, ["   f((b.toByte()))"], ["   f((b.toByte()).toInt())"]))
        self.assertTrue(self.accepts(fix, ['   throw E("x ${w.writeByte(b).s()}")'],
                                     ['   throw E("x ${w.writeByte(b.toInt()).s()}")']))
        self.assertFalse(self.accepts(fix, ["      val b0: Int = this[i++]"], ["      val b0: Long = this[i++].toLong()"]))
        self.assertFalse(self.accepts(fix, ["      return c"], ["      return c"]))

    def test_raw_generic_types_compose(self):
        """With the types written back raw, another Kotlin fix's recogniser takes the rest of the hunk."""
        old = ["      val authParams: java.util.Map = this.m()", "      val v: java.lang.String = c.name()"]
        new = ["      val authParams: MutableMap<String, String> = this.m()", "      val v: String = c.name()"]
        self.assertTrue(self.accepts("kt_raw_generic_type", old, new))
        self.assertEqual(self.tool.kg_raw_line("      val a: MutableMap<String, List<Int>>? = x as Map.Entry<K, V>"),
                         "      val a: java.util.Map? = x as Entry")

    def test_monitor_calls(self):
        fix = "kt_object_monitor_call"
        self.assertTrue(self.accepts(fix, ["         this.notifyAll()"], ["         (this as java.lang.Object).notifyAll()"]))
        self.assertTrue(self.accepts(fix, ["   monitor.wait(a, b.toInt())"], ["   (monitor as java.lang.Object).wait(a, b.toInt())"]))
        self.assertFalse(self.accepts(fix, ["         this.notifyAll()"], ["         (this as java.lang.Object).notify()"]))

    def test_raw_generic_types(self):
        fix = "kt_raw_generic_type"
        self.assertTrue(self.accepts(fix, ["      val i: java.util.Iterator = l.iterator()"],
                                     ["      val i: MutableIterator<String> = l.iterator()"]))
        self.assertTrue(self.accepts(fix, ["      if (x is java.util.Collection && (x as java.util.Collection).isEmpty()) {"],
                                     ["      if (x is Collection<*> && (x as Collection<String>).isEmpty()) {"]))
        self.assertTrue(self.accepts(fix, ["      val r: ObjectRef = ObjectRef()", "      r.element = v as T"],
                                     ["      val r: ObjectRef<Any?> = ObjectRef()", "      r.element = v"]))
        self.assertTrue(self.accepts(fix, ["   val pins: MutableList<Pin> = ArrayList(20) as java.util.List"],
                                     ["   val pins: MutableList<Pin> = ArrayList(20)"]))
        self.assertTrue(self.accepts(fix, ["      total += (i.next() as String).length"], ["      total += i.next().length"]))
        self.assertTrue(self.accepts(fix, ["      return r as MutableMap.MutableEntry<K, V>"], ["      return r as Map.Entry<K, V>"]))
        self.assertFalse(self.accepts(fix, ["      val n: Int = f(x) as Int"], ["      val n: Int = f(x)"]))
        self.assertFalse(self.accepts(fix, ["      val v: java.lang.String = c.name()"], ["      val v: String = c.name()"]))
        self.assertFalse(self.accepts(fix, ["   public open fun buffer(): Buffer {"], ["   public open fun buffer(): Buffer? {"]))
        self.assertFalse(self.accepts(fix, ["   val v: Int = (v or v ushr 1)"], ["   val v: Int = ((v or v) ushr 1)"]))
        self.assertFalse(self.accepts(fix, ["      val i: java.util.Iterator = l.iterator()"],
                                      ["      val i: MutableIterator<String> = m.iterator()"]))


if __name__ == "__main__":
    unittest.main()
