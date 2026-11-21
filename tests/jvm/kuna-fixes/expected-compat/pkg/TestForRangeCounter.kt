package pkg

public class TestForRangeCounter {
   public fun weighted(a: IntArray): Int {
      var s: Int = 0
      var i: Int = 0

      for (n in a.length..i) {
         s += a[i] * i
      }

      return s
   }

   public fun skipping(a: IntArray, limit: Int): Int {
      var c: Int = 0

      repeat(limit) { var5 ->
         c += a[var5]
         var5 += a[var5]
      }

      return c
   }

   public fun sumUntilBig(a: IntArray): Int {
      var total: Int = 0

      repeat(99) { i ->
         total += a[i]
      }

      return total
   }
}
