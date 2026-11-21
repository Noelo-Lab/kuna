package pkg

public object TestStaticInitializer {
   public final val table: IntArray
   public final val ready: Boolean

   init {
      val t: IntArray = IntArray(16)
      var i: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      val e: Int = t.size
      while (i < e) {
         t[i] = i * i

         i++
      }

      table = t

      var var4: Boolean
      try {
         var4 = t[15] > 100
      } catch (var3: IllegalStateException) {
         var4 = false
      }

      ready = var4
   }

   public class Platform {
      public companion object {
         public final val isSupported: Boolean
         public final val level: Int

         init {
            var var0: Boolean
            try {
               Class.forName("pkg.Missing")
               var0 = true
            } catch (var2: ClassNotFoundException) {
               var0 = false
            }

            isSupported = var0
            level = if (isSupported) 2 else 1
         }
      }
   }
}
