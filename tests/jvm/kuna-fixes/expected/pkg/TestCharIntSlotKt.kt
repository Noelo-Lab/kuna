package pkg

import kotlin.jvm.internal.Intrinsics

public fun parseSigned(s: CharSequence): Int {
   var i: Int = 0
   val start: Char = s.charAt(0)
   val var10000: Char
   if (start != '+' && start != '-') {
      var10000 = ' '
   } else {
      i++
      var10000 = start
   }

   val var6: Int = i
   var n: Int = 0

   while (i < s.length()) {
      val digits: Char = s.charAt(i)
      if ('0' > digits || digits >= ':') {
         break
      }

      n = n * 10 + (s.charAt(i) - '0')
      i++
   }

   if (i - var6 > 3 && Intrinsics.compare(s.charAt(var6), 50) >= 0) {
      return -1
   } else {
      return if (var10000.code == 45) -n else n
   }
}
