package pkg

class TestForRangeUnmatchedBound {
   fun sum(a: IntArray): Int {
      var c = 0
      var i = 0
      while (i < a.size) {
         c += a[i]
         i++
      }
      return c
   }
}
