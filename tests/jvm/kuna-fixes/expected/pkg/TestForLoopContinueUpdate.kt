package pkg

public class TestForLoopContinueUpdate {
   public fun colonOffset(input: CharArray, pos: Int, limit: Int): Int {
      // $VF: Unable to resugar Kotlin loop from Java for loop
      var i: Int = pos
      if (i < limit) {
         label35@ do {
            val var5: Char = input[i]
            if (input[i] == '[') {
               while (true) {
                  i++
                  if (i >= limit || input[i] == ']') {
                     continue@label35
                  }
               }
            } else if (var5 == ':') {
               return i
            }
         } while (run { i++; i < limit })
      }

      return limit
   }
}
