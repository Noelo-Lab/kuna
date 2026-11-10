package pkg

public fun allDigits(s: String): Boolean {
   val length: Int = s.length
   if (length == 0) {
      return false
   } else {
      for (index in (if (s[0] == '-') 1 else 0) until length) {
         val var3: Char = s[index]
         if ('0' > var3 || var3 >= ':') {
            return false
         }
      }

      return true
   }
}

public fun sumFrom(start: Int?, end: Int): Int {
   var total: Int = 0
   var k: Int = start ?: 0
   if (k <= end) {
      while (true) {
         total += k
         if (k == end) {
            break
         }

         k++
      }
   }

   return total
}

public fun countDown(from: Int, extra: Int?): Int {
   var total: Int = 0
   var k: Int = from
   val var4: Int = extra ?: 1
   if (var4 <= from) {
      while (true) {
         total += k
         if (k == var4) {
            break
         }

         k--
      }
   }

   return total
}
