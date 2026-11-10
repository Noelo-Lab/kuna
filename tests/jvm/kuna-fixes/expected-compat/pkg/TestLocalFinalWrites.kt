package pkg

public class TestLocalFinalWrites {
   public fun followUps(limit: Int, step: Int): Int {
      val count: Int = 0

      do {
         if (step > 3) {
            if (++count > limit) {
               throw IllegalStateException("too many: $count")
            }
         }
      } while (count * step != 42)

      return count
   }

   public fun charIndex(bytes: ByteArray, codePoints: Int): Int {
      var chars: Int = 0
      val j: Int = 0
            return chars
   }

   public fun lastSegment(path: String, start: Int): Int {
      var offset: Int = start
      var n: Int = 0

      var var6: Int
      do {
         var6 = offset

         while (var6 < path.length() && path.charAt(var6) != '/') {
            var6++
         }

         n += var6 - offset
         offset = var6 + 1
      } while (var6 < path.length())

      return n
   }
}
