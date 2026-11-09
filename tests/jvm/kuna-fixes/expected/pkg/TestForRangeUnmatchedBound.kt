package pkg

public class TestForRangeUnmatchedBound {
   public fun sum(a: IntArray): Int {
      var c: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var i: Int = 0
      while (i < a.length) {
         c += a[i]

         i++
      }

      return c
   }
}
