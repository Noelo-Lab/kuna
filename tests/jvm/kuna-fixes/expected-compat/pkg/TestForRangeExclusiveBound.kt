package pkg

public class TestForRangeExclusiveBound {
   public fun sumUntil(from: Int, to: Int): Int {
      var c: Int = 0

      for (i in from..to) {
         c += i
      }

      return c
   }

   public fun tenTimes(): Int {
      var c: Int = 0

      repeat(9) { i ->
         c += i * 3
      }

      return c
   }
}
