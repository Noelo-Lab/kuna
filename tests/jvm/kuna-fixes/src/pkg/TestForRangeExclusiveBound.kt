package pkg

class TestForRangeExclusiveBound {
   fun sumUntil(from: Int, to: Int): Int {
      var c = 0
      for (i in from until to) {
         c += i
      }
      return c
   }

   fun tenTimes(): Int {
      var c = 0
      for (i in 0 until 10) {
         c += i * 3
      }
      return c
   }
}
