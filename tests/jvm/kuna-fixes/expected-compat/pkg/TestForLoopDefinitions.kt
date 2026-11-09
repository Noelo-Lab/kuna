package pkg

public class TestForLoopDefinitions {
   public fun jumpy(a: IntArray): Int {
      var c: Int = 0

      repeat(99) { var4 ->
         c += a[var4]
         var4 += a[var4]
      }

      return c + var4
   }
}
