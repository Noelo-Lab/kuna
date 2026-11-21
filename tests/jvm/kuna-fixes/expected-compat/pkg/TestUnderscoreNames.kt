package pkg

public class TestUnderscoreNames {
   public fun twice(_: Int): Int {
      return _ * 2
   }

   public fun sum(xs: IntArray): Int {
      var __: Int = 0

      for (_ in xs) {
         __ += _
      }

      return __
   }
}
