package pkg

import java.util.Map.Entry
import kotlin.enums.EnumEntries

public class TestMappedJavaMember {
   public fun lengths(s: String, cs: CharSequence, sb: StringBuilder): Int {
      return s.length + cs.length + sb.length
   }

   public fun firstChar(s: String, i: Int): Char {
      return (s[0].code + s[i + 1].code).toChar()
   }

   public fun counts(list: List<String>, map: Map<String, Int>, arr: IntArray, objs: Array<Any>): Int {
      return list.size + map.size + map.keys.size + map.values.size + map.entries.size + arr.size + objs.size
   }

   public fun firstEntry(map: Map<String, Int>): String {
      val entry: Entry = CollectionsKt.first(map.entries as java.lang.Iterable) as Entry
      return "${entry.key}${entry.value}"
   }

   public fun describe(t: Throwable): String? {
      var var10000: String = t.message
      if (var10000 == null) {
         val var2: Throwable = t.cause
         var10000 = if (var2 != null) var2.message else null
      }

      return var10000
   }

   public fun kind(x: Any): String {
      val var10000: String = x.javaClass.getName()
      return var10000
   }

   public fun total(n: Number): Long {
      return n.toLong() + n.toInt()
   }

   public fun drop(list: MutableList<String>): String {
      return list.removeAt(0) as String
   }

   public fun colorName(c: pkg.TestMappedJavaMember.Color): String {
      return "${c.name}${c.ordinal}"
   }

   public enum class Color {
      RED,
      GREEN;

      @JvmStatic
      fun getEntries(): EnumEntries<TestMappedJavaMember.Color> {
         return `$ENTRIES`
      }
   }
}
