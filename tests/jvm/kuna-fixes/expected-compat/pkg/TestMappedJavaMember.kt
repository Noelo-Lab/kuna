package pkg

import java.util.Map.Entry
import kotlin.enums.EnumEntries

public class TestMappedJavaMember {
   public fun lengths(s: String, cs: CharSequence, sb: StringBuilder): Int {
      return s.length() + cs.length() + sb.length()
   }

   public fun firstChar(s: String, i: Int): Char {
      return (char)(s.charAt(0) + s.charAt(i + 1))
   }

   public fun counts(list: List<String>, map: Map<String, Int>, arr: IntArray, objs: Array<Any>): Int {
      return list.size() + map.size() + map.keySet().size() + map.values().size() + map.entrySet().size() + arr.length + objs.length
   }

   public fun firstEntry(map: Map<String, Int>): String {
      val entry: Entry = CollectionsKt.first(map.entrySet() as java.lang.Iterable) as Entry
      return "${entry.getKey()}${entry.getValue()}"
   }

   public fun describe(t: Throwable): String? {
      var var10000: java.lang.String = t.getMessage()
      if (var10000 == null) {
         val var2: java.lang.Throwable = t.getCause()
         var10000 = if (var2 != null) var2.getMessage() else null
      }

      return var10000
   }

   public fun kind(x: Any): String {
      val var10000: java.lang.String = x.getClass().getName()
      return var10000
   }

   public fun total(n: Number): Long {
      return n.longValue() + n.intValue()
   }

   public fun drop(list: MutableList<String>): String {
      return list.remove(0) as java.lang.String
   }

   public fun colorName(c: pkg.TestMappedJavaMember.Color): String {
      return "${c.name()}${c.ordinal()}"
   }

   public enum class Color {
      RED,
      GREEN;

      @JvmStatic
      fun getEntries(): EnumEntries<TestMappedJavaMember.Color> {
         $ENTRIES
      }
   }
}
