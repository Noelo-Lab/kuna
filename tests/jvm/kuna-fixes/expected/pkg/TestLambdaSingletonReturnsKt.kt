package pkg

import kotlin.jvm.functions.Function1
import kotlin.jvm.functions.Function2

public val clamp: (Int) -> Int = { x ->
   if (x < 0) {
      0
   } else {
      if (x > 100) 100 else x
   }
} as Function1

public val firstNegative: (IntArray) -> Int = { values ->
   var found: Int = -1

   for (v in values) {
      if (v < 0) {
         found = v
         break
      }
   }

   found
} as Function1

public val sign: (Int) -> String = { x ->
   val var10000: String
   if (x < 0) {
      System.out.println(x)
      var10000 = "negative"
   } else {
      var10000 = if (x == 0) "zero" else "positive"
   }

   var10000
} as Function1

public val search: (List<String>, String) -> Int = lambda_1@{ items, key ->
   val var3: java.util.Iterator = (items as java.lang.Iterable).iterator()
   var var4: Int = 0

   while (var3.hasNext()) {
      val i: Int = var4++
      if (var3.next() as String == key) {
         return@lambda_1 i
      }
   }

   return@lambda_1 -1
} as Function2
