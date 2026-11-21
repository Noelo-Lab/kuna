package pkg

public class TestForLoopContinueUpdate {
   public fun colonOffset(input: CharArray, pos: Int, limit: Int): Int {
      label35@ for (i in pos..limit) {
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
      }

      return limit
   }
}
