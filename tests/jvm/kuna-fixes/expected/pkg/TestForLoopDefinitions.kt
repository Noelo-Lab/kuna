package pkg

public class TestForLoopDefinitions {
   public fun jumpy(a: IntArray): Int {
      var c: Int = 0

      var var4: Int
      // $VF: Unable to resugar Kotlin loop from Java for loop
      var4 = 0
      while (var4 < 100) {
         c += a[var4]
         var4 += a[var4]

         var4++
      }

      return c + var4
   }
}
