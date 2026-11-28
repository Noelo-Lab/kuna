package pkg

class TestForLoopContinueUpdate {
   fun colonOffset(input: CharArray, pos: Int, limit: Int): Int {
      var i = pos
      while (i < limit) {
         when (input[i]) {
            '[' -> {
               while (++i < limit) {
                  if (input[i] == ']') break
               }
            }
            ':' -> return i
         }
         i++
      }
      return limit
   }
}
