package pkg

import kotlin.jvm.internal.Intrinsics

public fun parseSigned(s: CharSequence): Int {
   var i: Int = 0
   var start: Char = s.charAt(0)
   val var10000: Char
   if (start != '+' && start != '-') {
      var10000 = ' '
   } else {
      i++
      var10000 = start
   }

   start = (char)i
   var n: Int = 0

   while (i < s.length()) {
      val digits: Char = s.charAt(i)
      if ('0' > digits || digits >= ':') {
         break
      }

      n = n * 10 + (s.charAt(i) - '0')
      i++
   }

   if (i - start > 3 && Intrinsics.compare(s.charAt(start), 50) >= 0) {
      return -1
   } else {
      return if (var10000 == 45) -n else n
   }
}
