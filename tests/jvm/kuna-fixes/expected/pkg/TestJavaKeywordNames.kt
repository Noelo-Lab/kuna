package pkg

import kotlin.jvm.functions.Function2

public class TestJavaKeywordNames {
   public final val same: (Any?, Any?) -> Boolean = { old, new ->
      old == new
   } as Function2

   public fun shift(new: Int, default: Int): Int {
      return new * 31 + default
   }

   public fun count(s: String, char: Char): Int {
      var int: Int = 0
      var var4: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      val var5: Int = s.length()
      while (var4 < var5) {
         if (s.charAt(var4) == char) {
            int++
         }

         var4++
      }

      return int
   }

   public fun pick(flag: Boolean): (Int, Int) -> Int {
      return if (flag) { old: Int, new: Int ->
         old - new
      } else { a: Int, b: Int ->
         a + b
      }
   }
}
