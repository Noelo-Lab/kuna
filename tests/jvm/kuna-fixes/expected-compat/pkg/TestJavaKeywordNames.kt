package pkg

import kotlin.jvm.functions.Function2

public class TestJavaKeywordNames {
   public final val same: (Any?, Any?) -> Boolean = { old, new ->
      old == var2
   } as Function2

   public fun shift(new: Int, default: Int): Int {
      return var1 * 31 + var2
   }

   public fun count(s: String, char: Char): Int {
      var var3: Int = 0
      var var4: Int = 0

      for (var5 in s.length()..var4) {
         if (s.charAt(var4) == var2) {
            var3++
         }
      }

      return var3
   }

   public fun pick(flag: Boolean): (Int, Int) -> Int {
      return if (flag) { old: Int, var1: Int ->
         old - var1
      } else { a: Int, b: Int ->
         a + b
      }
   }
}
