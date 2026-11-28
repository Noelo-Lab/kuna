package pkg

class TestForRangeCounter {
   fun weighted(a: IntArray): Int {
      var s = 0
      var i = 0
      val n = a.size
      while (i < n) {
         s += a[i] * i
         i++
      }
      return s
   }

   fun skipping(a: IntArray, limit: Int): Int {
      var c = 0
      var i = 0
      while (i < limit) {
         c += a[i]
         i += a[i]
         i++
      }
      return c
   }

   fun sumUntilBig(a: IntArray): Int {
      var total = 0
      var i = 0
      while (total < 100) {
         total += a[i]
         i++
      }
      return total
   }
}
