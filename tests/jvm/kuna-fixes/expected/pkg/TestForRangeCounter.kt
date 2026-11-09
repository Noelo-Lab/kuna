package pkg

public class TestForRangeCounter {
   public fun weighted(a: IntArray): Int {
      var s: Int = 0
      var i: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      val n: Int = a.length
      while (i < n) {
         s += a[i] * i

         i++
      }

      return s
   }

   public fun skipping(a: IntArray, limit: Int): Int {
      var c: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var var5: Int = 0
      while (var5 < limit) {
         c += a[var5]
         var5 += a[var5]

         var5++
      }

      return c
   }

   public fun sumUntilBig(a: IntArray): Int {
      var total: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var i: Int = 0
      while (total < 100) {
         total += a[i]

         i++
      }

      return total
   }
}
